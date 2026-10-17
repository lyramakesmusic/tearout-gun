// Tearout gun as a MIDI instrument. Notes play the rendered patch at their pitch; a new note ducks the ones
// still ringing.
use crossbeam_channel::{bounded, Sender};
use nih_plug::prelude::*;
use std::sync::atomic::Ordering;
use std::sync::Arc;

pub mod analysis;
pub mod editor;
pub mod params;
pub mod presets;
pub mod worker;

use params::PlugParams;
use worker::{fold_note, Shared, Sound};

const MAX_VOICES: usize = 16;

struct Voice {
    s: Arc<Sound>,
    pos: f64,
    rate: f64,
    gain: f32,
    duck: f32,
    duck_target: f32,
}

pub struct Tearout {
    params: Arc<PlugParams>,
    shared: Arc<Shared>,
    note_tx: Sender<u8>,
    worker: Option<std::thread::JoinHandle<()>>,
    voices: Vec<Voice>,
    events: Vec<(usize, u8, f32)>,
    sr: f64,
}

impl Default for Tearout {
    fn default() -> Self {
        let params = Arc::new(PlugParams::default());
        let shared = Arc::new(Shared::default());
        let (tx, rx) = bounded::<u8>(64);
        let (p2, s2) = (params.clone(), shared.clone());
        let worker = std::thread::Builder::new().name("tearout-render".into()).spawn(move || worker::run(p2, s2, rx)).ok();
        Self { params, shared, note_tx: tx, worker, voices: Vec::with_capacity(MAX_VOICES), events: Vec::with_capacity(256), sr: 48000.0 }
    }
}

impl Drop for Tearout {
    fn drop(&mut self) {
        self.shared.quit.store(true, Ordering::Relaxed);
        if let Some(h) = self.worker.take() { let _ = h.join(); }
    }
}

impl Tearout {
    fn note_on(&mut self, note: u8, vel: f32) {
        let (sound, exact) = {
            let Ok(b) = self.shared.bank.try_lock() else { return };
            let dice = if self.params.dice.value() { b.dice.clone() } else { None };
            if let Some(d) = dice {
                self.shared.dice_used.store(true, Ordering::Relaxed);
                (Some(d), false)
            } else if let Some(s) = b.notes.get(note as usize).cloned().flatten() {
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
        for v in self.voices.iter_mut() { v.duck_target = v.duck_target.min(1.0 - duck); }
        if self.voices.len() >= MAX_VOICES { self.voices.remove(0); }
        let vk = self.params.vel.value();
        let gain = 1.0 - vk + vk * vel;
        self.voices.push(Voice { s, pos: 0.0, rate, gain, duck: 1.0, duck_target: 1.0 });
    }
}

impl Plugin for Tearout {
    const NAME: &'static str = "Tearout Gun";
    const VENDOR: &'static str = "lyra";
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

    fn initialize(&mut self, _layout: &AudioIOLayout, cfg: &BufferConfig, _ctx: &mut impl InitContext<Self>) -> bool {
        self.sr = cfg.sample_rate as f64;
        true
    }

    fn reset(&mut self) { self.voices.clear(); }

    fn process(&mut self, buffer: &mut Buffer, _aux: &mut AuxiliaryBuffers, ctx: &mut impl ProcessContext<Self>) -> ProcessStatus {
        let n = buffer.samples();
        let out = buffer.as_slice();
        for ch in out.iter_mut() { ch.fill(0.0); }
        let mut events = std::mem::take(&mut self.events);
        events.clear();
        while let Some(ev) = ctx.next_event() {
            if let NoteEvent::NoteOn { timing, note, velocity, .. } = ev { events.push((timing as usize, note, velocity)); }
        }
        if editor::AUDITION.swap(false, Ordering::Relaxed) {
            let note = self.params.gun.snapshot().0[tearout_dsp::id::NOTE] as u8;
            events.push((0, note, 0.8));
        }
        let rel_ms = self.params.duck_rel.value() as f64;
        let dstep = (1000.0 / (rel_ms.max(1.0) * self.sr)) as f32; // duck recovery per sample toward target
        let attack = (1000.0 / (3.0 * self.sr)) as f32; // 3 ms duck-down
        let mut ei = 0;
        for i in 0..n {
            while ei < events.len() && events[ei].0 <= i { let (_, nt, vel) = events[ei]; self.note_on(nt, vel); ei += 1; }
            let (mut l, mut r) = (0f32, 0f32);
            for v in self.voices.iter_mut() {
                let ip = v.pos as usize;
                if ip + 1 >= v.s.l.len() { v.gain = 0.0; continue; }
                let fr = (v.pos - ip as f64) as f32;
                let g = v.gain * v.duck;
                l += g * (v.s.l[ip] + (v.s.l[ip + 1] - v.s.l[ip]) * fr);
                r += g * (v.s.r[ip] + (v.s.r[ip + 1] - v.s.r[ip]) * fr);
                v.pos += v.rate;
                // duck down fast, then ride the target back up over the release time
                v.duck = if v.duck > v.duck_target { (v.duck - attack).max(v.duck_target) } else { v.duck_target };
                v.duck_target = (v.duck_target + dstep).min(1.0);
            }
            out[0][i] = l;
            out[1][i] = r;
        }
        self.voices.retain(|v| v.gain > 0.0);
        self.events = events;
        ProcessStatus::Normal
    }
}

impl ClapPlugin for Tearout {
    const CLAP_ID: &'static str = "com.lyra.tearout-gun";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("tearout gun synth");
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
