// Tearout Farm, a MIDI instrument. Notes play the rendered patch at their pitch; a new note ducks the ones
// still ringing.
use crossbeam_channel::{bounded, Sender};
use nih_plug::prelude::*;
use std::sync::atomic::Ordering;
use std::sync::Arc;

pub mod analysis;
pub mod editor;
pub mod export;
pub mod grab;
pub mod limiter;
#[cfg(target_os = "macos")]
#[allow(unexpected_cfgs)]
pub mod macos;
pub mod params;
pub mod presets;
pub mod roll;
pub mod tips;
#[cfg(windows)]
pub mod win;
pub mod worker;

use params::PlugParams;
use worker::{fold_note, Shared, Sound};

const MAX_VOICES: usize = 16;
/// How long a live note waits for a render of freshly automated knobs before playing what there is.
const WAIT_MS: f64 = 25.0;

/// Set by the standalone app, where the editor plays from the computer keyboard.
pub static STANDALONE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

struct Voice {
    s: Arc<Sound>,
    pos: f64,
    rate: f64,
    gain: f32,
    duck: f32,
    duck_target: f32,
    /// 0..1: how far this voice has crossfaded to its highpassed self, once a newer note took the sub
    choke: f32,
    hp: [Hp; 2],
}

/// 4th-order Linkwitz-Riley highpass (two Butterworth biquads), for choked voices.
#[derive(Clone, Copy, Default)]
struct Hp { z: [[f32; 2]; 2] }

#[derive(Clone, Copy, Default)]
struct HpCoef { b: [f32; 3], a: [f32; 2] }

impl HpCoef {
    fn new(f: f64, sr: f64) -> Self {
        let w = 2.0 * std::f64::consts::PI * f / sr;
        let (c, s) = (w.cos(), w.sin());
        let al = s / (2.0 * std::f64::consts::FRAC_1_SQRT_2);
        let a0 = 1.0 + al;
        Self { b: [((1.0 + c) / 2.0 / a0) as f32, (-(1.0 + c) / a0) as f32, ((1.0 + c) / 2.0 / a0) as f32], a: [(-2.0 * c / a0) as f32, ((1.0 - al) / a0) as f32] }
    }
}

impl Hp {
    fn run(&mut self, k: &HpCoef, mut x: f32) -> f32 {
        for z in self.z.iter_mut() {
            let y = k.b[0] * x + z[0];
            z[0] = k.b[1] * x - k.a[0] * y + z[1];
            z[1] = k.b[2] * x - k.a[1] * y;
            x = y;
        }
        x
    }
}

/// Crossover below which a newer note takes over the low end.
const CHOKE_HZ: f64 = 150.0;

pub struct Tearout {
    params: Arc<PlugParams>,
    shared: Arc<Shared>,
    note_tx: Sender<u8>,
    worker: Option<std::thread::JoinHandle<()>>,
    voices: Vec<Voice>,
    events: Vec<(usize, u8, f32)>,
    sr: f64,
    limiter: limiter::Limiter,
    hp_coef: HpCoef,
    /// rendering offline (a bounce or export), where a note can wait for its exact render
    offline: bool,
    /// notes waiting for the render of the patch as it is now: note, velocity, samples waited
    pending: Vec<(u8, f32, usize)>,
}

impl Default for Tearout {
    fn default() -> Self {
        let params = Arc::new(PlugParams::default());
        let shared = Arc::new(Shared::default());
        let (tx, rx) = bounded::<u8>(64);
        let (p2, s2) = (params.clone(), shared.clone());
        let worker = std::thread::Builder::new().name("tearout-render".into()).spawn(move || worker::run(p2, s2, rx)).ok();
        Self { params, shared, note_tx: tx, worker, voices: Vec::with_capacity(MAX_VOICES), events: Vec::with_capacity(256), sr: 48000.0, limiter: limiter::Limiter::new(48000.0), hp_coef: HpCoef::new(CHOKE_HZ, 48000.0), offline: false, pending: Vec::with_capacity(16) }
    }
}

impl Drop for Tearout {
    fn drop(&mut self) {
        self.shared.quit.store(true, Ordering::Relaxed);
        if let Some(h) = self.worker.take() { let _ = h.join(); }
    }
}

impl Tearout {
    /// Play `events` (sample offset, note, velocity) and mix every voice into `l`/`r`.
    pub fn mix(&mut self, events: &[(usize, u8, f32)], l_out: &mut [f32], r_out: &mut [f32]) {
        let rel_ms = self.params.duck_rel.value() as f64;
        let dstep = (1000.0 / (rel_ms.max(1.0) * self.sr)) as f32; // duck recovery per sample toward target
        let attack = (1000.0 / (3.0 * self.sr)) as f32; // 3 ms duck-down
        let choke_step = (1000.0 / (10.0 * self.sr)) as f32; // 10 ms handover of the lows
        let hk = self.hp_coef;
        // notes that waited for a fresh render start once it lands, or after WAIT_MS with what there is
        if !self.pending.is_empty() {
            let fresh = self.fresh();
            let limit = (WAIT_MS * self.sr / 1000.0) as usize;
            let mut k = 0;
            while k < self.pending.len() {
                let (note, vel, waited) = self.pending[k];
                if fresh || waited >= limit { self.pending.remove(k); self.start(note, vel); } else { self.pending[k].2 += l_out.len(); k += 1; }
            }
        }
        let mut ei = 0;
        for i in 0..l_out.len() {
            while ei < events.len() && events[ei].0 <= i { let (_, nt, vel) = events[ei]; self.note_on(nt, vel); ei += 1; }
            let (mut l, mut r) = (0f32, 0f32);
            for v in self.voices.iter_mut() {
                let ip = v.pos as usize;
                if ip + 1 >= v.s.l.len() { v.gain = 0.0; continue; }
                let fr = (v.pos - ip as f64) as f32;
                let g = v.gain * v.duck;
                let (mut xl, mut xr) = (v.s.l[ip] + (v.s.l[ip + 1] - v.s.l[ip]) * fr, v.s.r[ip] + (v.s.r[ip + 1] - v.s.r[ip]) * fr);
                if v.choke > 0.0 {
                    let (hl, hr) = (v.hp[0].run(&hk, xl), v.hp[1].run(&hk, xr));
                    xl += (hl - xl) * v.choke;
                    xr += (hr - xr) * v.choke;
                    v.choke = (v.choke + choke_step).min(1.0);
                }
                l += g * xl;
                r += g * xr;
                v.pos += v.rate;
                // duck down fast, then ride the target back up over the release time
                v.duck = if v.duck > v.duck_target { (v.duck - attack).max(v.duck_target) } else { v.duck_target };
                v.duck_target = (v.duck_target + dstep).min(1.0);
            }
            l_out[i] = l;
            r_out[i] = r;
        }
        self.voices.retain(|v| v.gain > 0.0);
    }

    /// Whether the rendered bank is of the patch as the knobs are now.
    fn fresh(&self) -> bool {
        let key = worker::hash(&worker::patch(&self.params, &self.shared), false);
        self.shared.bank.try_lock().is_ok_and(|b| b.key == key && b.base.is_some())
    }

    /// A note arrives. Automation that moved the knobs since the last render gets rendered first: offline the
    /// note waits for it (and for its exact pitch), live it waits up to WAIT_MS.
    fn note_on(&mut self, note: u8, vel: f32) {
        self.shared.last_note.store(note, Ordering::Relaxed);
        self.shared.notes_seen.fetch_add(1, Ordering::Relaxed);
        if self.offline {
            let _ = self.note_tx.try_send(note);
            let slot = worker::note_slot(note);
            let t0 = std::time::Instant::now();
            while t0.elapsed().as_secs_f64() < 5.0 {
                let key = worker::hash(&worker::patch(&self.params, &self.shared), false);
                let ready = self.shared.bank.lock().is_ok_and(|b| b.key == key && b.notes.get(slot).is_some_and(|s| s.is_some()));
                if ready { break; }
                std::thread::sleep(std::time::Duration::from_millis(1));
            }
        } else if !self.fresh() {
            let _ = self.note_tx.try_send(note);
            self.pending.push((note, vel, 0));
            return;
        }
        self.start(note, vel);
    }

    fn start(&mut self, note: u8, vel: f32) {
        let (sound, exact) = {
            let Ok(b) = self.shared.bank.try_lock() else { return };
            let dice = if self.params.dice.value() { b.dice.clone() } else { None };
            if let Some(d) = dice {
                self.shared.dice_used.store(true, Ordering::Relaxed);
                (Some(d), false)
            } else if let Some(s) = b.notes.get(worker::note_slot(note)).cloned().flatten() {
                (Some(s), true)
            } else {
                (b.base.clone(), false)
            }
        };
        let Some(s) = sound else { return };
        if !exact { let _ = self.note_tx.try_send(note); }
        let target = fold_note(note);
        let rate = 2f64.powf((target - s.note) / 12.0) * tearout_dsp::SR / self.sr;
        let duck = self.params.duck.value();
        for v in self.voices.iter_mut() {
            v.duck_target = v.duck_target.min(1.0 - duck);
            // one sub at a time: earlier notes hand their lows to this one
            if v.choke == 0.0 { v.choke = 1e-6; }
        }
        if self.voices.len() >= MAX_VOICES { self.voices.remove(0); }
        let vk = self.params.vel.value();
        let gain = 1.0 - vk + vk * vel;
        self.voices.push(Voice { s, pos: 0.0, rate, gain, duck: 1.0, duck_target: 1.0, choke: 0.0, hp: [Hp::default(); 2] });
    }
}

impl Plugin for Tearout {
    const NAME: &'static str = "Tearout Farm";
    const VENDOR: &'static str = "lyraaaa";
    const URL: &'static str = "";
    const EMAIL: &'static str = "lyramakesmusic@gmail.com";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout { main_input_channels: None, main_output_channels: NonZeroU32::new(2), ..AudioIOLayout::const_default() }];
    const MIDI_INPUT: MidiConfig = MidiConfig::Basic;
    const SAMPLE_ACCURATE_AUTOMATION: bool = false;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> { self.params.clone() }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        editor::create(self.params.clone(), self.shared.clone(), self.note_tx.clone())
    }

    fn initialize(&mut self, _layout: &AudioIOLayout, cfg: &BufferConfig, ctx: &mut impl InitContext<Self>) -> bool {
        self.sr = cfg.sample_rate as f64;
        self.offline = cfg.process_mode == ProcessMode::Offline;
        self.limiter = limiter::Limiter::new(self.sr);
        self.hp_coef = HpCoef::new(CHOKE_HZ, self.sr);
        ctx.set_latency_samples(self.limiter.latency());
        true
    }

    fn reset(&mut self) { self.voices.clear(); self.pending.clear(); self.limiter.reset(); }

    fn process(&mut self, buffer: &mut Buffer, _aux: &mut AuxiliaryBuffers, ctx: &mut impl ProcessContext<Self>) -> ProcessStatus {
        let n = buffer.samples();
        let out = buffer.as_slice();
        for ch in out.iter_mut() { ch.fill(0.0); }
        if let Some(t) = ctx.transport().tempo { self.shared.tempo.store(t.to_bits(), Ordering::Relaxed); }
        let mut events = std::mem::take(&mut self.events);
        events.clear();
        while let Some(ev) = ctx.next_event() {
            if let NoteEvent::NoteOn { timing, note, velocity, .. } = ev { events.push((timing as usize, note, velocity)); }
        }
        while let Ok((note, vel)) = self.shared.keys.1.try_recv() { events.push((0, note, vel)); }
        while let Ok(s) = self.shared.preview.1.try_recv() {
            if self.voices.len() >= MAX_VOICES { self.voices.remove(0); }
            let rate = tearout_dsp::SR / self.sr;
            self.voices.push(Voice { s, pos: 0.0, rate, gain: 0.8, duck: 1.0, duck_target: 1.0, choke: 0.0, hp: [Hp::default(); 2] });
        }
        if editor::AUDITION.swap(false, Ordering::Relaxed) {
            let note = self.params.gun.snapshot().0[tearout_dsp::id::NOTE] as u8;
            events.push((0, note, 0.8));
        }
        let (l, r) = out.split_at_mut(1);
        self.mix(&events, &mut l[0][..n], &mut r[0][..n]);
        let g = 10f32.powf(self.params.out.value() / 20.0);
        if g != 1.0 { for x in l[0][..n].iter_mut().chain(r[0][..n].iter_mut()) { *x *= g; } }
        let on = self.params.limit.value();
        self.limiter.process(&mut l[0][..n], &mut r[0][..n], on);
        // the editor's live spectrum reads the output from here
        if let Ok(mut sc) = self.shared.scope.try_lock() {
            let (buf, pos, sr) = &mut *sc;
            *sr = self.sr;
            for i in 0..n { buf[*pos] = 0.5 * (l[0][i] + r[0][i]); *pos = (*pos + 1) % buf.len(); }
        }
        self.events = events;
        ProcessStatus::Normal
    }
}

impl ClapPlugin for Tearout {
    const CLAP_ID: &'static str = "com.lyra.tearout-gun";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Tearout Farm");
    const CLAP_MANUAL_URL: Option<&'static str> = None;
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::Instrument, ClapFeature::Synthesizer, ClapFeature::Stereo];
}

impl Vst3Plugin for Tearout {
    const VST3_CLASS_ID: [u8; 16] = *b"TearoutGunLyra01";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[Vst3SubCategory::Instrument, Vst3SubCategory::Synth];
}

nih_export_clap!(Tearout);
nih_export_vst3!(Tearout);

#[cfg(test)]
mod tests {
    use super::*;

    fn settle(t: &Tearout) {
        let t0 = std::time::Instant::now();
        while !t.fresh() && t0.elapsed().as_secs() < 10 { std::thread::sleep(std::time::Duration::from_millis(2)); }
        assert!(t.fresh(), "worker never rendered the patch");
    }

    /// A knob change (here a mute, which changes the patch like automation does) right at a note: offline the note
    /// plays the new render; live it waits for it, within WAIT_MS.
    #[test]
    fn notes_follow_knob_changes() {
        let mut t = Tearout::default();
        let (mut l, mut r) = (vec![0f32; 4096], vec![0f32; 4096]);
        let home = t.params.gun.snapshot().0[tearout_dsp::id::NOTE] as u8;
        settle(&t);

        t.offline = true;
        t.shared.mute.fetch_xor(1, Ordering::Relaxed);
        assert!(!t.fresh());
        t.mix(&[(0, home, 1.0)], &mut l, &mut r);
        let key = worker::hash(&worker::patch(&t.params, &t.shared), false);
        let b = t.shared.bank.lock().unwrap();
        assert_eq!(b.key, key, "offline note played before the new render");
        assert!(Arc::ptr_eq(&t.voices.last().unwrap().s, b.base.as_ref().unwrap()), "offline note played a stale render");
        drop(b);

        t.offline = false;
        t.voices.clear();
        t.shared.mute.fetch_xor(1, Ordering::Relaxed);
        t.mix(&[(0, home, 1.0)], &mut l[..128], &mut r[..128]);
        assert_eq!(t.voices.len(), 0, "live note started on a stale render");
        let mut waited = 128;
        while t.voices.is_empty() { std::thread::sleep(std::time::Duration::from_micros(2600)); t.mix(&[], &mut l[..128], &mut r[..128]); waited += 128; }
        let b = t.shared.bank.lock().unwrap();
        let fresh = Arc::ptr_eq(&t.voices[0].s, b.base.as_ref().unwrap());
        println!("live note waited {:.1} ms, fresh render: {fresh}", waited as f64 / 48.0);
        assert!(waited as f64 / 48.0 <= WAIT_MS + 3.0 * 128.0 / 48.0, "live note waited past WAIT_MS");
        drop(b);
        // how long the worker takes to have a knob change rendered, settled and not
        for _ in 0..5 {
            settle(&t);
            std::thread::sleep(std::time::Duration::from_millis(300));
            let t0 = std::time::Instant::now();
            t.shared.mute.fetch_xor(1, Ordering::Relaxed);
            let _ = t.note_tx.try_send(home);
            while !t.fresh() { std::thread::sleep(std::time::Duration::from_micros(200)); }
            println!("render after a change, worker idle: {:.1} ms", t0.elapsed().as_secs_f64() * 1000.0);
        }
    }
}
