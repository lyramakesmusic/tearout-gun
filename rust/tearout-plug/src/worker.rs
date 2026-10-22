// Background renderer. Watches the patch, renders it (and per-note / dice variants for the audio thread),
// splits it into stems and builds each lane's spectrogram and waveform for the editor.
use crate::analysis::{colorize, reassigned, wave_cols};
use crate::params::PlugParams;
use crossbeam_channel::{Receiver, RecvTimeoutError};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tearout_dsp::id::*;
use tearout_dsp::{from_norm, to_norm, Engine, Params, N, SPEC};

pub const VIEW_MS: f64 = 533.0;
pub const SPEC_COLS: usize = 820;
pub const SPEC_ROWS: usize = 360;
pub const WAVE_COLS: usize = 820;

pub struct Sound { pub l: Vec<f32>, pub r: Vec<f32>, pub note: f64 }

/// What the audio thread plays: the patch at its own note, renders at other notes as they finish, and the
/// next dice roll.
#[derive(Default)]
pub struct Bank {
    pub base: Option<Arc<Sound>>,
    pub notes: Vec<Option<Arc<Sound>>>,
    pub dice: Option<Arc<Sound>>,
}

pub struct Lane {
    pub name: &'static str,
    pub color: [u8; 3],
    pub level_db: f64,
    pub rgba: Vec<u8>,
    pub wave: Vec<(f32, f32)>,
    pub l: Arc<Vec<f32>>,
    pub r: Arc<Vec<f32>>,
}

pub struct View {
    pub version: u64,
    pub patch: u64,
    pub p: Params,
    pub lanes: Vec<Lane>,
    pub peak_db: f64,
}

pub struct Shared {
    pub bank: Mutex<Bank>,
    pub view: Mutex<Option<Arc<View>>>,
    pub view_version: AtomicU64,
    pub quit: AtomicBool,
    /// set by the audio thread when it consumed the dice roll
    pub dice_used: AtomicBool,
    /// last MIDI note played, so dice rolls and background renders start there
    pub last_note: std::sync::atomic::AtomicU8,
    /// which sample slots hold decoded audio
    pub loaded: Mutex<[bool; 3]>,
    /// host tempo in BPM, as f64 bits
    pub tempo: AtomicU64,
    /// lanes muted and soloed for listening, one bit per [`lane_bit`]
    pub mute: std::sync::atomic::AtomicU32,
    pub solo: std::sync::atomic::AtomicU32,
    /// knob groups random, mutate and dice leave alone
    pub locked: Mutex<Vec<&'static str>>,
}

/// Mute/solo bit of a stem lane: the layers in [`LAYERS`] order, then reverb.
pub fn lane_bit(name: &str) -> Option<u32> {
    LAYERS.iter().position(|l| l.0 == name).map(|k| k as u32).or((name == "reverb").then_some(LAYERS.len() as u32))
}

fn lane_level(bit: u32) -> usize { LAYERS.get(bit as usize).map_or(REV_LVL, |l| l.1) }

/// The patch to render: the knobs, with the burst interval set from the host tempo when shots follow the grid,
/// and muted (or unsoloed) lanes silent.
pub fn patch(params: &PlugParams, sh: &Shared) -> Params {
    let mut p = full_patch(params, sh);
    let (mute, solo) = (sh.mute.load(Ordering::Relaxed), sh.solo.load(Ordering::Relaxed));
    for bit in 0..=LAYERS.len() as u32 {
        if mute & (1 << bit) != 0 || (solo != 0 && solo & (1 << bit) == 0) { p.0[lane_level(bit)] = -60.0; }
    }
    p
}

/// The patch with every lane sounding.
pub fn full_patch(params: &PlugParams, sh: &Shared) -> Params {
    let mut p = params.gun.snapshot();
    let beats = crate::params::GRID[(params.shots_grid.value().round() as usize).min(crate::params::GRID.len() - 1)].0;
    grid_interval(&mut p, beats, f64::from_bits(sh.tempo.load(Ordering::Relaxed)));
    p
}

/// Burst interval for `shots` spread across `beats` at `bpm`: the shots and the gap after the last one fill
/// the span, with accel bending the spacing as the knob does.
pub fn grid_interval(p: &mut Params, beats: f64, bpm: f64) {
    let shots = p.0[B_SHOTS].round().max(1.0) as usize;
    if beats <= 0.0 || shots < 2 { return; }
    let span = beats * 60000.0 / bpm.clamp(20.0, 999.0);
    let w = |s: usize| 2f64.powf(-p.0[B_ACCEL] * ((s - 1) as f64 / (shots - 1) as f64) * 1.5);
    let sum: f64 = (1..shots).map(w).sum::<f64>() + w(shots - 1);
    let s = &SPEC[B_RATE];
    p.0[B_RATE] = (span / sum).clamp(s.min, s.max);
}

impl Default for Shared {
    fn default() -> Self {
        Self { bank: Mutex::new(Bank { notes: vec![None; NOTE_SLOTS], ..Default::default() }), view: Mutex::new(None), view_version: AtomicU64::new(0), quit: AtomicBool::new(false), dice_used: AtomicBool::new(true), last_note: std::sync::atomic::AtomicU8::new(36), loaded: Mutex::new([false; 3]), tempo: AtomicU64::new(150f64.to_bits()), mute: Default::default(), solo: Default::default(), locked: Mutex::new(Vec::new()) }
    }
}

pub const LAYERS: [(&str, usize, [u8; 3]); 8] = [
    ("sub", SUB_LVL, [86, 180, 233]),
    ("transient", TR_LVL, [230, 159, 0]),
    ("body", BODY_LVL, [63, 199, 154]),
    ("synth", SYN_LVL, [204, 121, 167]),
    ("spice", SPICE_LVL, [240, 228, 66]),
    ("sample 1", S1_LVL, [213, 94, 0]),
    ("sample 2", S1_LVL + 10, [0, 158, 115]),
    ("sample 3", S1_LVL + 20, [176, 196, 222]),
];

/// Sample slot (1–3) a lane plays, if it is a sample lane.
pub fn sample_slot(name: &str) -> Option<usize> { name.strip_prefix("sample ").and_then(|k| k.parse().ok()) }

/// Decode an audio file to stereo f32 at its own rate.
pub fn decode(path: &str) -> Result<(Vec<f32>, Vec<f32>, f64), String> {
    use symphonia::core::{audio::SampleBuffer, codecs::DecoderOptions, formats::FormatOptions, io::MediaSourceStream, meta::MetadataOptions, probe::Hint};
    let f = std::fs::File::open(path).map_err(|e| e.to_string())?;
    let mss = MediaSourceStream::new(Box::new(f), Default::default());
    let mut hint = Hint::new();
    if let Some(ext) = std::path::Path::new(path).extension().and_then(|e| e.to_str()) { hint.with_extension(ext); }
    let probed = symphonia::default::get_probe().format(&hint, mss, &FormatOptions::default(), &MetadataOptions::default()).map_err(|e| e.to_string())?;
    let mut fmt = probed.format;
    let track = fmt.default_track().ok_or("no audio track")?;
    let (tid, rate) = (track.id, track.codec_params.sample_rate.unwrap_or(48000) as f64);
    let mut dec = symphonia::default::get_codecs().make(&track.codec_params, &DecoderOptions::default()).map_err(|e| e.to_string())?;
    let (mut l, mut r) = (Vec::new(), Vec::new());
    // ten seconds is plenty for a layer
    let cap = (rate * 10.0) as usize;
    while l.len() < cap {
        let Ok(pk) = fmt.next_packet() else { break };
        if pk.track_id() != tid { continue; }
        let Ok(buf) = dec.decode(&pk) else { continue };
        let ch = buf.spec().channels.count().max(1);
        let mut sb = SampleBuffer::<f32>::new(buf.capacity() as u64, *buf.spec());
        sb.copy_interleaved_ref(buf);
        for fr in sb.samples().chunks(ch) { l.push(fr[0]); r.push(if ch > 1 { fr[1] } else { fr[0] }); }
    }
    if l.is_empty() { return Err("no audio".into()); }
    Ok((l, r, rate))
}
pub const MIX_COLOR: [u8; 3] = [230, 230, 230];
pub const REV_COLOR: [u8; 3] = [154, 160, 166];

/// MIDI note folded by octaves into the engine's note range.
/// Number of distinct engine notes; the bank keeps one render per slot.
pub const NOTE_SLOTS: usize = (SPEC[NOTE].max - SPEC[NOTE].min) as usize + 1;

pub fn note_slot(n: u8) -> usize { (fold_note(n) - SPEC[NOTE].min) as usize }

pub fn fold_note(n: u8) -> f64 {
    let s = &SPEC[NOTE];
    let mut m = n as f64;
    while m > s.max { m -= 12.0; }
    while m < s.min { m += 12.0; }
    m
}

fn hash(p: &Params, skip_note: bool) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    for i in 0..N { if !(skip_note && i == NOTE) { p.0[i].to_bits().hash(&mut h); } }
    h.finish()
}

/// Dice ranges: half-width in normalized units per core knob, scaled by the dice amount.
pub const DICE: [(usize, f64); 12] = [
    (SUB_LVL, 0.05), (SUB_TAU, 0.3), (SUB_SWELL, 0.3), (SUB_DRIVE, 0.3), (TR_LVL, 0.15), (BODY_LVL, 0.05),
    (BODY_DEC, 0.1), (BODY_SHAPE, 0.1), (BODY_HOLD, 0.2), (BODY_DRIVE, 0.2), (CR_MIX, 0.1), (MST_HIGH, 0.1),
];
/// Knobs that move the sound less than a fresh noise seed: dice roams their full range.
pub const FREE: [usize; 14] = [
    BODY_FM_RATIO, BODY_FM_MOD, BODY_FM_IDX, BODY_FM_TAU, BODY_COMB_F, BODY_COMB_FB, BODY_COMB_DAMP, BODY_MODE_TILT,
    BODY_Q, BODY_METAL, BODY_FLT_RES, BODY_FLT_TAU, REV_TONE, REV_CHAR,
];

pub fn roll(p: &Params, amt: f64, r: &mut tearout_dsp::dsp::Rng) -> Params {
    let mut q = p.clone();
    for &(i, w) in DICE.iter() {
        let s = &SPEC[i];
        q.0[i] = from_norm(s, (to_norm(s, p.0[i]) + (r.next() * 2.0 - 1.0) * w * amt * 2.0).clamp(0.0, 1.0));
    }
    for &i in FREE.iter() {
        let s = &SPEC[i];
        let u = to_norm(s, p.0[i]);
        q.0[i] = from_norm(s, u + (r.next() - u) * amt);
    }
    q.0[SEED] = (1.0 + r.next() * 9998.0).floor();
    q
}

pub fn run(params: Arc<PlugParams>, sh: Arc<Shared>, notes: Receiver<u8>) {
    let mut main = Engine::new();
    let mut other = Engine::new();
    let mut stem_eng: Vec<Engine> = (0..LAYERS.len() + 2).map(|_| Engine::new()).collect();
    let mut last = 0u64;
    let mut last_view = 0u64;
    let mut rng = tearout_dsp::dsp::Rng::new(1234.0);
    let mut pending_view: Option<Params> = None;
    let mut have = vec![false; NOTE_SLOTS];
    let mut loaded: [String; 3] = Default::default();
    let render_note = |eng: &mut Engine, p: &Params, slot: usize| {
        let mut q = p.clone();
        q.0[NOTE] = SPEC[NOTE].min + slot as f64;
        let o = eng.render(&q);
        Arc::new(Sound { l: o.l, r: o.r, note: q.0[NOTE] })
    };
    while !sh.quit.load(Ordering::Relaxed) {
        let want = params.samples.read().map(|s| s.clone()).unwrap_or_default();
        if want != loaded {
            for k in 0..3 {
                if want[k] == loaded[k] { continue; }
                let audio = if want[k].is_empty() { None } else { decode(&want[k]).map_err(|e| nih_plug::nih_log!("sample {}: {e}", want[k])).ok() };
                for e in [&mut main, &mut other].into_iter().chain(stem_eng.iter_mut()) {
                    e.set_sample(k + 1, audio.as_ref().map(|(l, r, sr)| (&l[..], &r[..], *sr)));
                }
            }
            loaded = want;
            *sh.loaded.lock().unwrap() = [0, 1, 2].map(|k| main.has_sample(k + 1));
            last = 0;
            last_view = 0;
        }
        let p = patch(&params, &sh);
        let k = hash(&p, false);
        if k != last {
            last = k;
            let o = main.render(&p);
            let base = Arc::new(Sound { l: o.l, r: o.r, note: p.0[NOTE] });
            let home = (p.0[NOTE] - SPEC[NOTE].min) as usize;
            have.iter_mut().for_each(|h| *h = false);
            have[home] = true;
            let mut b = sh.bank.lock().unwrap();
            for n in b.notes.iter_mut() { *n = None; }
            b.notes[home] = Some(base.clone());
            b.base = Some(base);
            b.dice = None;
            drop(b);
            // the note being played renders next, so it keeps sounding exact while knobs move
            let hot = note_slot(sh.last_note.load(Ordering::Relaxed));
            if hot != home {
                let s = render_note(&mut other, &p, hot);
                have[hot] = true;
                sh.bank.lock().unwrap().notes[hot] = Some(s);
            }
            sh.dice_used.store(true, Ordering::Relaxed);
            pending_view = Some(p.clone());
        }
        let busy = have.iter().any(|h| !h) || pending_view.is_some();
        // notes the audio thread asked for come first
        match notes.recv_timeout(Duration::from_millis(if busy { 0 } else { 8 })) {
            Ok(n) => {
                let slot = note_slot(n);
                if !have[slot] {
                    let s = render_note(&mut other, &p, slot);
                    have[slot] = true;
                    sh.bank.lock().unwrap().notes[slot] = Some(s);
                }
                continue;
            }
            Err(RecvTimeoutError::Disconnected) => break,
            Err(RecvTimeoutError::Timeout) => {}
        }
        if params.dice.value() && sh.dice_used.swap(false, Ordering::Relaxed) {
            let locked = sh.locked.lock().unwrap().clone();
            let mut q = crate::roll::keep(&p, roll(&p, params.dice_amt.value() as f64, &mut rng), &locked);
            q.0[NOTE] = fold_note(sh.last_note.load(Ordering::Relaxed));
            let o = other.render(&q);
            sh.bank.lock().unwrap().dice = Some(Arc::new(Sound { l: o.l, r: o.r, note: q.0[NOTE] }));
            continue;
        }
        // the view is the slow part: build it only once the patch has settled for one poll
        if let Some(vp) = pending_view.take() {
            if hash(&patch(&params, &sh), false) != hash(&vp, false) { continue; }
            let kv = hash(&vp, false);
            if kv == last_view { continue; }
            last_view = kv;
            let b = sh.bank.lock().unwrap().base.clone();
            let Some(mix) = b else { continue };
            let full = full_patch(&params, &sh);
            let v = build_view(&full, &mix, &mut stem_eng, sh.view_version.load(Ordering::Relaxed) + 1);
            *sh.view.lock().unwrap() = Some(Arc::new(v));
            sh.view_version.fetch_add(1, Ordering::Release);
            continue;
        }
        // fill the rest of the keyboard, nearest the last played note first
        let at = note_slot(sh.last_note.load(Ordering::Relaxed)) as i64;
        if let Some(slot) = (0..NOTE_SLOTS).filter(|&s| !have[s]).min_by_key(|&s| (s as i64 - at).abs()) {
            let s = render_note(&mut other, &p, slot);
            have[slot] = true;
            sh.bank.lock().unwrap().notes[slot] = Some(s);
        }
    }
}

/// Lanes for the view: `mix` as heard, and every layer of the full patch `p` on its own.
fn build_view(p: &Params, mix: &Sound, eng: &mut [Engine], version: u64) -> View {
    let sr = tearout_dsp::SR;
    let t1 = (VIEW_MS * sr / 1000.0) as usize;
    // stems: each layer soloed (others muted), reverb = mix minus the dry render
    let mut stems: Vec<(&'static str, [u8; 3], f64, Vec<f32>, Vec<f32>)> = Vec::new();
    let (head, last) = eng.split_at_mut(LAYERS.len());
    for (k, &(name, lvl, col)) in LAYERS.iter().enumerate() {
        if p.0[lvl] <= -59.0 || sample_slot(name).is_some_and(|slot| !head[0].has_sample(slot)) { continue; }
        let mut q = p.clone();
        for &(_, o, _) in LAYERS.iter() { if o != lvl { q.0[o] = -60.0; } }
        q.0[REV_LVL] = -60.0;
        let o = head[k].render(&q);
        stems.push((name, col, p.0[lvl], o.l, o.r));
    }
    if p.0[REV_LVL] > -59.0 {
        let mut q = p.clone();
        q.0[REV_LVL] = -60.0;
        let (dry_eng, wet_eng) = last.split_at_mut(1);
        let o = dry_eng[0].render(&q);
        let wet = wet_eng[0].render(p);
        let n = wet.l.len().min(o.l.len());
        let l: Vec<f32> = (0..n).map(|i| wet.l[i] - o.l[i]).collect();
        let r: Vec<f32> = (0..n).map(|i| wet.r[i] - o.r[i]).collect();
        stems.push(("reverb", REV_COLOR, p.0[REV_LVL], l, r));
    }
    let mut all = vec![("mix", MIX_COLOR, 0.0, mix.l.clone(), mix.r.clone())];
    all.extend(stems);
    let lanes: Vec<Lane> = std::thread::scope(|sc| {
        let hs: Vec<_> = all
            .iter()
            .map(|(name, col, lvl, l, r)| {
                sc.spawn(move || {
                    let m: Vec<f32> = l.iter().zip(r).map(|(a, b)| 0.5 * (a + b)).collect();
                    let g = reassigned(&m, sr, t1, SPEC_COLS, SPEC_ROWS, 16);
                    Lane { name, color: *col, level_db: *lvl, rgba: colorize(&g, *col), wave: wave_cols(l, r, t1, WAVE_COLS), l: Arc::new(l.clone()), r: Arc::new(r.clone()) }
                })
            })
            .collect();
        hs.into_iter().map(|h| h.join().unwrap()).collect()
    });
    let pk = mix.l.iter().chain(&mix.r).fold(0f32, |a, &b| a.max(b.abs())) as f64;
    View { version, patch: hash(p, false), p: p.clone(), lanes, peak_db: 20.0 * pk.max(1e-9).log10() }
}
