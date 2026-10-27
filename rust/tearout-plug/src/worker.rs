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

/// An audio-thread event for the log: positions are the host's timeline in samples.
#[derive(Clone, Copy)]
pub enum Trace {
    /// the plugin (re)started, rendering offline or live
    Start { offline: bool, sr: f64 },
    /// knobs changed since the last block (up to three of them, with their new values), or the transport
    /// started or stopped
    Block { pos: i64, playing: bool, changed: [(u16, f64); 3], n: u16, fast: bool },
    /// a note started: whether the render matched the knobs, and how long it waited for one
    Note { pos: i64, note: u8, shots: f64, fresh: bool, waited_ms: f64 },
    /// a young note crossfaded to a newer render
    Swap { note: u8, age_ms: f64, shots: f64 },
}

impl Trace {
    fn line(&self) -> String {
        let knob = |(i, v): (u16, f64)| format!("{} {}", SPEC[i as usize].id, crate::params::fmt_value(&SPEC[i as usize], v));
        match *self {
            Trace::Start { offline, sr } => format!("start: {} at {sr} Hz", if offline { "offline render" } else { "live" }),
            Trace::Block { pos, playing, changed, n, fast } => {
                let list: Vec<String> = changed[..(n as usize).min(3)].iter().map(|&c| knob(c)).collect();
                format!("block @{pos}{}{}: {}{}", if playing { "" } else { " (stopped)" }, if fast { " (faster than realtime)" } else { "" }, list.join(", "), if n > 3 { format!(" +{} more", n - 3) } else { String::new() })
            }
            Trace::Note { pos, note, shots, fresh, waited_ms } => format!("note {note} @{pos}: shots {shots}, render {}{}", if fresh { "matched the knobs" } else { "was older than the knobs" }, if waited_ms > 0.05 { format!(", waited {waited_ms:.1} ms") } else { String::new() }),
            Trace::Swap { note, age_ms, shots } => format!("note {note} switched to the newer render {age_ms:.1} ms in (shots {shots})"),
        }
    }
}

/// What the audio thread plays: the patch at its own note, renders at other notes as they finish, and the
/// next dice roll.
#[derive(Default)]
pub struct Bank {
    /// [`hash`] of the patch `base` and `notes` were rendered from
    pub key: u64,
    /// the patch at the note last played, which other notes play transposed until their own renders land
    pub base: Option<Arc<Sound>>,
    pub notes: Vec<Option<Arc<Sound>>>,
    pub dice: Option<Arc<Sound>>,
    /// [`hash`] of the patch the dice roll varies
    pub dice_key: u64,
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
    /// columns of each lane's spectrogram and waveform, spanning the whole sound
    pub cols: usize,
    /// length of the sound, ms
    pub len_ms: f64,
    pub version: u64,
    pub patch: u64,
    pub p: Params,
    pub lanes: Vec<Lane>,
    pub peak_db: f64,
    /// the mix's third-octave balance, dB, and its loudest band
    pub tob: [f64; crate::analysis::TOB_BANDS],
    pub tob_max: f64,
}

/// Spectrograms of one time window of a view, at full resolution.
pub struct Zoom { pub view: u64, pub t0: f64, pub span: f64, pub rgba: Vec<Vec<u8>> }

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
    /// the time window the editor shows while zoomed in (start, span ms), and the sharper spectrograms made
    /// for it
    pub zoom_req: Mutex<Option<(f64, f64)>>,
    pub zoom: Mutex<Option<Arc<Zoom>>>,
    /// a stem the editor plays on its own
    pub preview: (crossbeam_channel::Sender<Arc<Sound>>, crossbeam_channel::Receiver<Arc<Sound>>),
    /// notes played so far, for the editor's activity light
    pub notes_seen: AtomicU64,
    /// the last 8192 output samples (mono) and the next write position, with the output rate
    pub scope: Mutex<(Vec<f32>, usize, f64)>,
    /// notes the editor plays from the computer keyboard, for the audio thread
    pub keys: (crossbeam_channel::Sender<(u8, f32)>, crossbeam_channel::Receiver<(u8, f32)>),
    /// what the audio thread saw, for tearout-log.txt
    pub trace: (crossbeam_channel::Sender<Trace>, crossbeam_channel::Receiver<Trace>),
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
        Self { bank: Mutex::new(Bank { notes: vec![None; NOTE_SLOTS], ..Default::default() }), view: Mutex::new(None), view_version: AtomicU64::new(0), quit: AtomicBool::new(false), dice_used: AtomicBool::new(true), last_note: std::sync::atomic::AtomicU8::new(36), loaded: Mutex::new([false; 3]), tempo: AtomicU64::new(150f64.to_bits()), mute: Default::default(), solo: Default::default(), locked: Mutex::new(Vec::new()), keys: crossbeam_channel::bounded(64), trace: crossbeam_channel::bounded(1024), scope: Mutex::new((vec![0.0; 8192], 0, 48000.0)), notes_seen: AtomicU64::new(0), preview: crossbeam_channel::bounded(4), zoom_req: Mutex::new(None), zoom: Mutex::new(None) }
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
pub const REV_COLOR: [u8; 3] = [226, 74, 84];

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

pub fn hash(p: &Params, skip_note: bool) -> u64 {
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
    // after a knob change, the other notes in play render alongside the one being played
    let mut helpers: Vec<Engine> = (0..3).map(|_| Engine::new()).collect();
    // the editor's lanes build on their own thread, so a knob change always renders for the audio thread at once
    let job: Arc<Mutex<Option<(Params, Arc<Sound>)>>> = Arc::new(Mutex::new(None));
    let viewer = {
        let (params, sh, job) = (params.clone(), sh.clone(), job.clone());
        std::thread::Builder::new().name("tearout-view".into()).spawn(move || views(params, sh, job)).ok()
    };
    let mut last = 0u64;
    let mut last_view = 0u64;
    let mut rng = tearout_dsp::dsp::Rng::new(1234.0);
    let mut pending_view: Option<Params> = None;
    let mut have = vec![false; NOTE_SLOTS];
    let mut played = vec![false; NOTE_SLOTS];
    let mut loaded: [String; 3] = Default::default();
    let render_note = |eng: &mut Engine, p: &Params, slot: usize| {
        let mut q = p.clone();
        q.0[NOTE] = SPEC[NOTE].min + slot as f64;
        let o = eng.render(&q);
        Arc::new(Sound { l: o.l, r: o.r, note: q.0[NOTE] })
    };
    while !sh.quit.load(Ordering::Relaxed) {
        let lines: Vec<String> = sh.trace.1.try_iter().map(|t| t.line()).collect();
        if !lines.is_empty() { crate::export::log_lines(&lines); }
        let want = params.samples.read().map(|s| s.clone()).unwrap_or_default();
        if want != loaded {
            for k in 0..3 {
                if want[k] == loaded[k] { continue; }
                let audio = if want[k].is_empty() { None } else { decode(&want[k]).map_err(|e| nih_plug::nih_log!("sample {}: {e}", want[k])).ok() };
                for e in [&mut main, &mut other].into_iter().chain(helpers.iter_mut()) {
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
            // the note being played renders first, so a note waiting on these knobs gets them soonest; the
            // patch's own note (the editor's view) follows
            let home = (p.0[NOTE] - SPEC[NOTE].min) as usize;
            let hot = note_slot(sh.last_note.load(Ordering::Relaxed));
            have.iter_mut().for_each(|h| *h = false);
            played[hot] = true;
            let mut along: Vec<usize> = std::iter::once(home).chain((0..NOTE_SLOTS).filter(|&s| played[s])).filter(|&s| s != hot).collect();
            along.dedup();
            along.sort_by_key(|&s| (s != home, (s as i64 - hot as i64).abs()));
            along.dedup();
            along.truncate(helpers.len());
            let done: Vec<(usize, Arc<Sound>)> = std::thread::scope(|sc| {
                let p = &p;
                let hs: Vec<_> = along.iter().zip(helpers.iter_mut()).map(|(&slot, eng)| sc.spawn(move || (slot, render_note(eng, p, slot)))).collect();
                let s = render_note(&mut main, p, hot);
                let mut b = sh.bank.lock().unwrap();
                for n in b.notes.iter_mut() { *n = None; }
                b.base = Some(s.clone());
                b.key = k;
                b.dice = None;
                b.notes[hot] = Some(s);
                drop(b);
                hs.into_iter().filter_map(|h| h.join().ok()).collect()
            });
            have[hot] = true;
            let mut b = sh.bank.lock().unwrap();
            for (slot, s) in done { have[slot] = true; b.notes[slot] = Some(s); }
            drop(b);
            sh.dice_used.store(true, Ordering::Relaxed);
            pending_view = Some(p.clone());
        }
        let busy = have.iter().any(|h| !h) || pending_view.is_some();
        // notes the audio thread asked for come first
        match notes.recv_timeout(Duration::from_millis(if busy { 0 } else { 8 })) {
            Ok(n) => {
                // knobs that moved while waiting render first
                if hash(&patch(&params, &sh), false) != last { continue; }
                let slot = note_slot(n);
                played[slot] = true;
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
            crate::roll::hold_stems(&p, &mut q);
            q.0[NOTE] = fold_note(sh.last_note.load(Ordering::Relaxed));
            let o = other.render(&q);
            let mut b = sh.bank.lock().unwrap();
            b.dice = Some(Arc::new(Sound { l: o.l, r: o.r, note: q.0[NOTE] }));
            b.dice_key = k;
            continue;
        }
        // the view is the slow part: build it only once the patch has settled for one poll
        if let Some(vp) = pending_view.take() {
            if hash(&patch(&params, &sh), false) != hash(&vp, false) { continue; }
            let kv = hash(&vp, false);
            if kv == last_view { continue; }
            last_view = kv;
            let home = (vp.0[NOTE] - SPEC[NOTE].min) as usize;
            let b = sh.bank.lock().unwrap().notes[home].clone();
            let Some(mix) = b else { continue };
            *job.lock().unwrap() = Some((full_patch(&params, &sh), mix));
            continue;
        }
        // the other notes that have been played get their exact renders, nearest the last played first
        let at = note_slot(sh.last_note.load(Ordering::Relaxed)) as i64;
        if let Some(slot) = (0..NOTE_SLOTS).filter(|&s| played[s] && !have[s]).min_by_key(|&s| (s as i64 - at).abs()) {
            let s = render_note(&mut other, &p, slot);
            have[slot] = true;
            sh.bank.lock().unwrap().notes[slot] = Some(s);
        }
    }
    if let Some(h) = viewer { let _ = h.join(); }
}

/// The editor's lanes: each patch handed over in `job` (the newest one) gets its stems and spectrograms; a
/// zoomed-in window gets sharper ones of its own.
fn views(params: Arc<PlugParams>, sh: Arc<Shared>, job: Arc<Mutex<Option<(Params, Arc<Sound>)>>>) {
    let mut stem_eng: Vec<Engine> = (0..LAYERS.len() + 2).map(|_| Engine::new()).collect();
    let mut loaded: [String; 3] = Default::default();
    while !sh.quit.load(Ordering::Relaxed) {
        let want = params.samples.read().map(|s| s.clone()).unwrap_or_default();
        if want != loaded {
            for k in 0..3 {
                if want[k] == loaded[k] { continue; }
                let audio = if want[k].is_empty() { None } else { decode(&want[k]).ok() };
                for e in stem_eng.iter_mut() { e.set_sample(k + 1, audio.as_ref().map(|(l, r, sr)| (&l[..], &r[..], *sr))); }
            }
            loaded = want;
        }
        let next = job.lock().unwrap().take();
        if let Some((full, mix)) = next {
            let v = build_view(&full, &mix, &mut stem_eng, sh.view_version.load(Ordering::Relaxed) + 1);
            *sh.view.lock().unwrap() = Some(Arc::new(v));
            sh.view_version.fetch_add(1, Ordering::Release);
            continue;
        }
        // a zoomed-in window gets its own spectrograms once it has held still for a moment
        let want = *sh.zoom_req.lock().unwrap();
        if let Some((t0, span)) = want {
            let have_z = sh.zoom.lock().unwrap().clone();
            let view = sh.view.lock().unwrap().clone();
            if let Some(view) = view {
                let fresh = have_z.as_ref().is_some_and(|z| z.view == view.version && (z.t0 - t0).abs() < 1e-6 && (z.span - span).abs() < 1e-6);
                if !fresh {
                    std::thread::sleep(Duration::from_millis(60));
                    if *sh.zoom_req.lock().unwrap() == want {
                        let sr = tearout_dsp::SR;
                        let (s0, n) = ((t0 * sr / 1000.0) as usize, (span * sr / 1000.0) as usize);
                        let rgba: Vec<Vec<u8>> = std::thread::scope(|sc| {
                            let hs: Vec<_> = view.lanes.iter().map(|ln| sc.spawn(move || {
                                let e = (s0 + n).min(ln.l.len());
                                let m: Vec<f32> = (s0.min(e)..e).map(|i| 0.5 * (ln.l[i] + ln.r[i])).collect();
                                colorize(&reassigned(&m, sr, n.max(1), SPEC_COLS, SPEC_ROWS, 16), ln.color)
                            })).collect();
                            hs.into_iter().map(|h| h.join().unwrap()).collect()
                        });
                        *sh.zoom.lock().unwrap() = Some(Arc::new(Zoom { view: view.version, t0, span, rgba }));
                    }
                    continue;
                }
            }
        }
        std::thread::sleep(Duration::from_millis(8));
    }
}

/// Lanes for the view: `mix` as heard, and every layer of the full patch `p` on its own.
fn build_view(p: &Params, mix: &Sound, eng: &mut [Engine], version: u64) -> View {
    let sr = tearout_dsp::SR;
    // the lanes span the whole sound, at the resolution of the default view, up to four times as wide
    let t1 = mix.l.len();
    let len_ms = t1 as f64 * 1000.0 / sr;
    let cols = ((SPEC_COLS as f64 * len_ms / VIEW_MS).round() as usize).clamp(SPEC_COLS / 2, SPEC_COLS * 4);
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
                    let g = reassigned(&m, sr, t1, cols, SPEC_ROWS, 16);
                    Lane { name, color: *col, level_db: *lvl, rgba: colorize(&g, *col), wave: wave_cols(l, r, t1, cols), l: Arc::new(l.clone()), r: Arc::new(r.clone()) }
                })
            })
            .collect();
        hs.into_iter().map(|h| h.join().unwrap()).collect()
    });
    let pk = mix.l.iter().chain(&mix.r).fold(0f32, |a, &b| a.max(b.abs())) as f64;
    let m: Vec<f32> = mix.l.iter().zip(&mix.r).map(|(a, b)| 0.5 * (a + b)).collect();
    let raw = crate::analysis::third_oct(&m, sr);
    let tob_max = raw.iter().cloned().fold(f64::MIN, f64::max);
    View { cols, len_ms, version, patch: hash(p, false), p: p.clone(), lanes, peak_db: 20.0 * pk.max(1e-9).log10(), tob: raw.map(|v| v - tob_max), tob_max }
}
