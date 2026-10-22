// Shot/burst assembly, reverb send, bus, output fades. `Engine` owns dropped samples, the IR cache and the last
// render's stage outputs: a stage whose inputs and params are unchanged is reused, so a master or reverb tweak
// re-runs only what follows it.
use std::collections::HashMap;
use std::sync::Arc;

use crate::chain::{bus, crunch};
use crate::dsp::*;
use crate::id::*;
use crate::layers::*;
use crate::{from_norm, to_norm, Params, SPEC};

pub struct Out { pub l: Buf, pub r: Buf, pub sr: f64 }

#[derive(Default)]
pub struct Engine {
    samples: [Option<Stereo>; 3],
    // bumped whenever a slot's audio changes; part of that sample layer's cache key
    sample_ver: [u64; 3],
    irs: HashMap<String, Arc<(Buf, Buf)>>,
    specs: HashMap<(String, usize), Arc<(Vec<f64>, Vec<f64>)>>,
    // last render's stage outputs, each with the hash of everything it was computed from
    layer_memo: [Option<(u64, Stereo)>; 8],
    crunch_memo: Option<(u64, Option<Stereo>)>,
    // convolved, filtered send (un-scaled) with its RMS and the send's RMS
    rev_memo: Option<(u64, Option<(Buf, Buf, f64, f64)>)>,
}

// layer slots: sub, tr, body, synth, spice, smp1..3, with the SPEC group each reads (besides global and burst)
const LAYER_GROUPS: [&str; 8] = ["sub", "tr", "body", "synth", "spice", "sample1", "sample2", "sample3"];
type Layers = [Option<Stereo>; 8];

fn key_of(p: &Params, groups: &[&str], extra: &[u64]) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    for (i, s) in SPEC.iter().enumerate() { if groups.contains(&s.group) { p[i].to_bits().hash(&mut h); } }
    extra.hash(&mut h);
    h.finish()
}

impl Engine {
    pub fn new() -> Self { Self::default() }

    /// Stores dropped audio for slot 1..=3 (any rate), resampled to the engine rate. `None` clears the slot.
    pub fn set_sample(&mut self, slot: usize, audio: Option<(&[f32], &[f32], f64)>) {
        let k = slot - 1;
        self.sample_ver[k] += 1;
        let Some((l, r, rate)) = audio else { self.samples[k] = None; return; };
        let ratio = rate / crate::SR;
        let n = (l.len() as f64 / ratio).floor() as usize;
        let lerp = |x: &[f32], i: usize| -> f32 {
            let t = i as f64 * ratio;
            let j = t.floor() as usize;
            let f = t - j as f64;
            let a = x[j] as f64;
            let b = if j + 1 < x.len() { x[j + 1] as f64 } else { a };
            (a + f * (b - a)) as f32
        };
        self.samples[k] = Some(((0..n).map(|i| lerp(l, i)).collect(), (0..n).map(|i| lerp(r, i)).collect()));
    }
    pub fn has_sample(&self, slot: usize) -> bool { self.samples[slot - 1].is_some() }

    /// Samples that sound longer than the patch's length extend the render: start crop, length, then the fade.
    fn sample_end(&self, p: &Params, sr: f64) -> usize {
        let mut end = 0usize;
        for k in 0..3 {
            let b = S1_LVL + 10 * k;
            let Some(src) = &self.samples[k] else { continue };
            if p[b] <= -59.0 { continue; }
            let st = p[b + 1] + if js_round(p[b + 2]) != 0.0 { p[NOTE] - 24.0 } else { 0.0 };
            let rate = 2f64.powf(st / 12.0);
            let avail = (src.0.len() as f64 - p[b + 3] * sr / 1000.0) / rate;
            end = end.max(((p[b + 4] * sr / 1000.0).min(avail) + p[b + 5] * sr / 1000.0).floor() as usize);
        }
        end
    }

    // x convolved with the IR `s` describes; IRs and their spectra (per FFT size) are cached
    fn conv(&mut self, s: &IrSpec, x: &[f32], out_len: usize, sr: f64) -> (Buf, Buf) {
        let key = s.key(sr);
        if self.irs.len() > 512 { self.irs.clear(); self.specs.clear(); }
        let h = self.irs.entry(key.clone()).or_insert_with(|| Arc::new(make_ir(s, sr))).clone();
        let n = conv_size(x.len(), h.0.len());
        let spec = self.specs.entry((key, n)).or_insert_with(|| Arc::new(ir_spectrum(&h.0, &h.1, n))).clone();
        convolve_spec(x, &spec, out_len)
    }

    // Per-layer stereo buffers (post layer level, pre master), for the layers `need` asks for.
    fn render_shot(&self, p: &Params, n: usize, sr: f64, need: &[bool; 8]) -> Layers {
        let seed = to_i32(p[SEED]) as f64;
        let mono = |x: Buf| (x.clone(), x);
        let mut ly: Layers = Default::default();
        let on = |k: usize, lvl: f64| need[k] && lvl > -59.0;
        // layers are independent: each renders on its own thread with its own RNG
        std::thread::scope(|sc| {
            let sub = on(0, p[SUB_LVL]).then(|| sc.spawn(|| mono(render_sub(p, n, sr, &mut Rng::new(seed * 19.0 + 11.0)))));
            let tr = on(1, p[TR_LVL]).then(|| sc.spawn(|| mono(render_tr(p, n, sr, &mut Rng::new(seed * 13.0 + 3.0)))));
            let spice = on(4, p[SPICE_LVL]).then(|| sc.spawn(|| mono(render_spice(p, n, sr, &mut Rng::new(seed * 17.0 + 9.0)))));
            let synth = on(3, p[SYN_LVL]).then(|| sc.spawn(|| render_synth(p, n, sr, seed)));
            if on(2, p[BODY_LVL]) { ly[2] = Some(render_body(p, n, sr, seed)); }
            for k in 0..3 {
                if on(5 + k, p[S1_LVL + 10 * k]) {
                    if let Some(src) = &self.samples[k] { ly[5 + k] = Some(render_sample(p, src, k + 1, n, sr)); }
                }
            }
            ly[0] = sub.map(|h| h.join().unwrap());
            ly[1] = tr.map(|h| h.join().unwrap());
            ly[4] = spice.map(|h| h.join().unwrap());
            ly[3] = synth.map(|h| h.join().unwrap());
        });
        // per-layer swell: a raised-sine ramp-in after the layer's drive, for ^-shaped (chug) attacks;
        // then per-layer onset delay (staggered layers: transient, then shell, then wash)
        let [l0, l1, l2, l3, l4, ..] = &mut ly;
        for (layer, swell, delay) in [(&mut *l0, p[SUB_SWELL], p[SUB_DELAY]), (&mut *l1, p[TR_SWELL], p[TR_DELAY]), (&mut *l2, p[BODY_SWELL], p[BODY_DELAY]), (&mut *l3, p[SYN_SWELL], p[SYN_DELAY]), (&mut *l4, 0.0, p[SPICE_DELAY])] {
            let Some((a, b)) = layer else { continue };
            let s = (swell * sr / 1000.0).floor() as usize;
            if s > 1 {
                for x in [&mut *a, &mut *b] {
                    for i in 0..s.min(n) { x[i] = (x[i] as f64 * (0.5 - 0.5 * (PI * i as f64 / s as f64).cos())) as f32; }
                }
            }
            let d = (delay * sr / 1000.0).floor() as i64;
            if d > 0 {
                let d = d as usize;
                for x in [&mut *a, &mut *b] {
                    let mut y = vec![0f32; n];
                    if d < n { y[d..].copy_from_slice(&x[..n - d]); }
                    *x = y;
                }
            } else if d < 0 {
                // a negative delay crops the layer's start
                let d = (-d) as usize;
                for x in [&mut *a, &mut *b] {
                    let mut y = vec![0f32; n];
                    if d < n { y[..n - d].copy_from_slice(&x[d..]); }
                    *x = y;
                }
            }
        }
        for (layer, lvl) in [(l0, p[SUB_LVL]), (l1, p[TR_LVL]), (l2, p[BODY_LVL]), (l4, p[SPICE_LVL]), (l3, p[SYN_LVL])] {
            if let Some((a, b)) = layer { let g = db_to_lin(lvl); scale(a, g); scale(b, g); }
        }
        ly
    }

    // Per-shot variation for bursts: round-robin seed, drift, jitter on timbral params.
    fn shot_params(p: &Params, s: usize, r: &mut Rng) -> Params {
        let mut q = p.clone();
        q[SEED] = to_i32(p[SEED]) as f64 * 1000.0 + s as f64 + 1.0;
        q[NOTE] = p[NOTE] + p[B_DRIFT] * s as f64;
        let j = p[B_JITTER];
        if j > 0.0 {
            for (id, amt) in [(SUB_DIVE, 0.15), (SUB_TAU, 0.15), (BODY_FLT_F0, 0.2), (BODY_FLT_F1, 0.2), (BODY_MODE_F, 0.08), (BODY_COMB_F, 0.08), (TR_PK_F, 0.2), (BODY_FM_IDX, 0.2)] {
                let sp = &SPEC[id];
                q[id] = from_norm(sp, to_norm(sp, q[id]) + (r.next() - 0.5) * amt * j);
            }
            q[NOTE] = js_round(q[NOTE]);
        }
        q
    }

    /// Renders a patch: stereo at 48 kHz, with a 1 ms fade-in and a 10 ms raised-cosine fade-out.
    pub fn render(&mut self, p: &Params) -> Out {
        let sr = crate::SR;
        let (mut l, mut r) = self.render_raw(p, sr);
        let n = l.len();
        let a = n.min((0.001 * sr).floor() as usize);
        let f = n.min((0.01 * sr).floor() as usize);
        for i in 0..a { let g = i as f64 / a as f64; l[i] = (l[i] as f64 * g) as f32; r[i] = (r[i] as f64 * g) as f32; }
        for i in 0..f {
            let g = 0.5 - 0.5 * (PI * i as f64 / f as f64).cos();
            let k = n - 1 - i;
            l[k] = (l[k] as f64 * g) as f32; r[k] = (r[k] as f64 * g) as f32;
        }
        Out { l, r, sr }
    }

    fn render_raw(&mut self, p: &Params, sr: f64) -> (Buf, Buf) {
        let shots = js_round(p[B_SHOTS]).max(1.0) as usize;
        let shot_len = ((p[LEN] * sr / 1000.0).floor() as usize).max(self.sample_end(p, sr));
        let mut onsets = vec![0f64];
        for s in 1..shots {
            let frac = (s - 1) as f64 / ((shots - 1).max(1)) as f64;
            let iv = p[B_RATE] * 2f64.powf(-p[B_ACCEL] * frac * 1.5);
            onsets.push(onsets[s - 1] + iv.max(4.0) * sr / 1000.0);
        }
        let total = onsets[shots - 1].floor() as usize + shot_len;
        let z = || (vec![0f32; total], vec![0f32; total]);
        // per-layer accumulations across shots, reused from the last render when nothing they read changed
        // the render length is part of every layer's key: a sample can stretch it
        let keys: [u64; 8] = std::array::from_fn(|k| key_of(p, &["global", "burst", LAYER_GROUPS[k]], &[if k >= 5 { self.sample_ver[k - 5] } else { 0 }, shot_len as u64]));
        let need: [bool; 8] = std::array::from_fn(|k| self.layer_memo[k].as_ref().map_or(true, |m| m.0 != keys[k]));
        let mut acc: Vec<Stereo> = (0..8).map(|k| if need[k] { z() } else { self.layer_memo[k].as_ref().unwrap().1.clone() }).collect();
        if need.iter().any(|&x| x) {
            let mut rr = Rng::new(to_i32(p[SEED]) as f64 * 7.0 + 77.0);
            for s in 0..shots {
                let q = if shots > 1 { Self::shot_params(p, s, &mut rr) } else { p.clone() };
                let gl = if shots > 1 && s < shots - 1 { ((onsets[s + 1] - onsets[s]) * p[B_GATE] + 0.003 * sr).floor() } else { shot_len as f64 };
                let n = shot_len.min(gl.max(64.0) as usize);
                let t0 = std::time::Instant::now();
                let ly = self.render_shot(&q, n, sr, &need);
                crate::timing::add(0, t0);
                let t0 = std::time::Instant::now();
                let fade = n.min((0.002 * sr).floor() as usize);
                let o = onsets[s].floor() as usize;
                for (k, lay) in ly.iter().enumerate() {
                    let Some(src) = lay else { continue };
                    for c in 0..2 {
                        let (sx, dst) = if c == 0 { (&src.0, &mut acc[k].0) } else { (&src.1, &mut acc[k].1) };
                        for i in 0..n {
                            if o + i >= total { break; }
                            let g = if i + fade > n { (n - i) as f64 / fade as f64 } else { 1.0 };
                            dst[o + i] = (dst[o + i] as f64 + sx[i] as f64 * g) as f32;
                        }
                    }
                }
                crate::timing::add(1, t0);
            }
            for k in 0..8 { if need[k] { self.layer_memo[k] = Some((keys[k], acc[k].clone())); } }
        }
        // tops = tr + body + synth + spice (+ samples routed to mids), through the crunch chain
        // crunch at zero mix passes the tops through, whatever its other settings
        let crunch_key = if p[CR_MIX] < 0.001 { key_of(p, &[], &keys[1..]) } else { key_of(p, &["crunch"], &keys[1..]) };
        let mut sub = acc[0].clone();
        let (mut tl, mut tr) = z();
        let mut any_top = false;
        for (k, lvl) in [(1, p[TR_LVL]), (2, p[BODY_LVL]), (3, p[SYN_LVL]), (4, p[SPICE_LVL])] {
            if lvl > -59.0 { add_scaled(&mut tl, &acc[k].0, 1.0); add_scaled(&mut tr, &acc[k].1, 1.0); any_top = true; }
        }
        for k in 0..3 {
            if p[S1_LVL + 10 * k] <= -59.0 || self.samples[k].is_none() { continue; }
            let sm = &acc[5 + k];
            if js_round(p[S1_ROUTE + 10 * k]) == 1.0 { // clean: alongside the sub
                add_scaled(&mut sub.0, &sm.0, 1.0); add_scaled(&mut sub.1, &sm.1, 1.0);
            } else {
                add_scaled(&mut tl, &sm.0, 1.0); add_scaled(&mut tr, &sm.1, 1.0); any_top = true;
            }
        }
        let t0 = std::time::Instant::now();
        let tops = match &self.crunch_memo {
            Some((k, t)) if *k == crunch_key => t.clone(),
            _ => {
                if any_top { crunch(&mut tl, &mut tr, p, sr, &mut |s, x, m| self.conv(s, x, m, sr)); }
                let t = if any_top { Some((tl, tr)) } else { None };
                self.crunch_memo = Some((crunch_key, t.clone()));
                t
            }
        };
        crate::timing::add(2, t0);
        let t0 = std::time::Instant::now();
        // reverb send from the tops; level relative to the send's RMS. The convolved send is kept un-scaled, so a
        // level change only rescales it.
        let mut q = p.clone();
        q[REV_LVL] = if p[REV_LVL] <= -59.0 { -60.0 } else { 0.0 };
        let rev_key = key_of(&q, &["rev"], &[crunch_key]);
        if !matches!(&self.rev_memo, Some((k, _)) if *k == rev_key) {
            let r = if p[REV_LVL] <= -59.0 { None } else {
                let send: Buf = match &tops { Some((a, b)) => (0..total).map(|i| (0.5 * (a[i] as f64 + b[i] as f64)) as f32).collect(), None => vec![0f32; total] };
                let (mut a, mut b) = self.conv(&IrSpec { ty: p[REV_TYPE], size: p[REV_SIZE], ch: p[REV_CHAR], tone: p[REV_TONE], pre: p[REV_PRE], seed: p[SEED] }, &send, total, sr);
                let hp = coef(Bq::Hp, p[REV_HP], 0.707, 0.0, sr);
                cascade(&mut [&mut a, &mut b], &[hp, hp]);
                let (rs, rv) = (rms(&send), rms2(&a, &b));
                Some((a, b, rs, rv))
            };
            self.rev_memo = Some((rev_key, r));
        }
        let rev = self.rev_memo.as_ref().unwrap().1.as_ref().map(|(a, b, rs, rv)| {
            let g = if *rv > 1e-9 { (rs / rv) * db_to_lin(p[REV_LVL]) } else { 0.0 };
            let (mut a, mut b) = (a.clone(), b.clone());
            scale(&mut a, g); scale(&mut b, g);
            (a, b)
        });
        crate::timing::add(3, t0);
        let t0 = std::time::Instant::now();
        let out = bus(&sub, tops.as_ref(), rev.as_ref(), p, sr);
        crate::timing::add(4, t0);
        out
    }
}
