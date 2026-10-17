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
}

impl Default for Shared {
    fn default() -> Self {
        Self { bank: Mutex::new(Bank { notes: vec![None; NOTE_SLOTS], ..Default::default() }), view: Mutex::new(None), view_version: AtomicU64::new(0), quit: AtomicBool::new(false), dice_used: AtomicBool::new(true), last_note: std::sync::atomic::AtomicU8::new(36) }
    }
}

pub const LAYERS: [(&str, usize, [u8; 3]); 5] = [
    ("sub", SUB_LVL, [86, 180, 233]),
    ("transient", TR_LVL, [230, 159, 0]),
    ("body", BODY_LVL, [63, 199, 154]),
    ("synth", SYN_LVL, [204, 121, 167]),
    ("spice", SPICE_LVL, [240, 228, 66]),
];
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
    let mut stem_eng: Vec<Engine> = (0..LAYERS.len() + 1).map(|_| Engine::new()).collect();
    let mut last = 0u64;
    let mut last_view = 0u64;
    let mut rng = tearout_dsp::dsp::Rng::new(1234.0);
    let mut pending_view: Option<Params> = None;
    let mut have = vec![false; NOTE_SLOTS];
    let render_note = |eng: &mut Engine, p: &Params, slot: usize| {
        let mut q = p.clone();
        q.0[NOTE] = SPEC[NOTE].min + slot as f64;
        let o = eng.render(&q);
        Arc::new(Sound { l: o.l, r: o.r, note: q.0[NOTE] })
    };
    while !sh.quit.load(Ordering::Relaxed) {
        let p = params.gun.snapshot();
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
            let mut q = roll(&p, params.dice_amt.value() as f64, &mut rng);
            q.0[NOTE] = fold_note(sh.last_note.load(Ordering::Relaxed));
            let o = other.render(&q);
            sh.bank.lock().unwrap().dice = Some(Arc::new(Sound { l: o.l, r: o.r, note: q.0[NOTE] }));
            continue;
        }
        // the view is the slow part: build it only once the patch has settled for one poll
        if let Some(vp) = pending_view.take() {
            if hash(&params.gun.snapshot(), false) != hash(&vp, false) { continue; }
            let kv = hash(&vp, false);
            if kv == last_view { continue; }
            last_view = kv;
            let b = sh.bank.lock().unwrap().base.clone();
            let Some(mix) = b else { continue };
            let v = build_view(&vp, &mix, &mut stem_eng, sh.view_version.load(Ordering::Relaxed) + 1);
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

fn build_view(p: &Params, mix: &Sound, eng: &mut [Engine], version: u64) -> View {
    let sr = tearout_dsp::SR;
    let t1 = (VIEW_MS * sr / 1000.0) as usize;
    // stems: each layer soloed (others muted), reverb = mix minus the dry render
    let mut stems: Vec<(&'static str, [u8; 3], f64, Vec<f32>, Vec<f32>)> = Vec::new();
    let (head, last) = eng.split_at_mut(LAYERS.len());
    for (k, &(name, lvl, col)) in LAYERS.iter().enumerate() {
        if p.0[lvl] <= -59.0 { continue; }
        let mut q = p.clone();
        for &(_, o, _) in LAYERS.iter() { if o != lvl { q.0[o] = -60.0; } }
        q.0[REV_LVL] = -60.0;
        let o = head[k].render(&q);
        stems.push((name, col, p.0[lvl], o.l, o.r));
    }
    if p.0[REV_LVL] > -59.0 {
        let mut q = p.clone();
        q.0[REV_LVL] = -60.0;
        let o = last[0].render(&q);
        let n = mix.l.len().min(o.l.len());
        let l: Vec<f32> = (0..n).map(|i| mix.l[i] - o.l[i]).collect();
        let r: Vec<f32> = (0..n).map(|i| mix.r[i] - o.r[i]).collect();
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
