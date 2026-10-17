// Shot/burst assembly, reverb send, bus, output fades. `Engine` owns dropped samples and the IR cache.
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
    irs: HashMap<String, Arc<(Buf, Buf)>>,
}

#[derive(Default)]
struct Layers { sub: Option<Stereo>, tr: Option<Stereo>, body: Option<Stereo>, synth: Option<Stereo>, spice: Option<Stereo>, smp: [Option<Stereo>; 3] }

impl Engine {
    pub fn new() -> Self { Self::default() }

    /// Stores dropped audio for slot 1..=3 (any rate), resampled to the engine rate. `None` clears the slot.
    pub fn set_sample(&mut self, slot: usize, audio: Option<(&[f32], &[f32], f64)>) {
        let k = slot - 1;
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

    fn ir(&mut self, s: &IrSpec, sr: f64) -> Arc<(Buf, Buf)> {
        let key = s.key(sr);
        if let Some(h) = self.irs.get(&key) { return h.clone(); }
        if self.irs.len() > 64 { self.irs.clear(); }
        let h = Arc::new(make_ir(s, sr));
        self.irs.insert(key, h.clone());
        h
    }

    // Per-layer stereo buffers (post layer level, pre master).
    fn render_shot(&self, p: &Params, n: usize, sr: f64) -> Layers {
        let seed = to_i32(p[SEED]) as f64;
        let mono = |x: Buf| (x.clone(), x);
        let mut ly = Layers::default();
        if p[SUB_LVL] > -59.0 { ly.sub = Some(mono(render_sub(p, n, sr, &mut Rng::new(seed * 19.0 + 11.0)))); }
        if p[TR_LVL] > -59.0 { ly.tr = Some(mono(render_tr(p, n, sr, &mut Rng::new(seed * 13.0 + 3.0)))); }
        if p[BODY_LVL] > -59.0 { ly.body = Some(render_body(p, n, sr, seed)); }
        if p[SPICE_LVL] > -59.0 { ly.spice = Some(mono(render_spice(p, n, sr, &mut Rng::new(seed * 17.0 + 9.0)))); }
        if p[SYN_LVL] > -59.0 { ly.synth = Some(render_synth(p, n, sr, seed)); }
        for k in 0..3 {
            if p[S1_LVL + 10 * k] > -59.0 {
                if let Some(src) = &self.samples[k] { ly.smp[k] = Some(render_sample(p, src, k + 1, n, sr)); }
            }
        }
        // per-layer swell: a raised-sine ramp-in after the layer's drive, for ^-shaped (chug) attacks;
        // then per-layer onset delay (staggered layers: transient, then shell, then wash)
        for (layer, swell, delay) in [(&mut ly.sub, p[SUB_SWELL], p[SUB_DELAY]), (&mut ly.tr, p[TR_SWELL], p[TR_DELAY]), (&mut ly.body, p[BODY_SWELL], p[BODY_DELAY]), (&mut ly.synth, p[SYN_SWELL], p[SYN_DELAY])] {
            let Some((a, b)) = layer else { continue };
            let s = (swell * sr / 1000.0).floor() as usize;
            if s > 1 {
                for x in [&mut *a, &mut *b] {
                    for i in 0..s.min(n) { x[i] = (x[i] as f64 * (0.5 - 0.5 * (PI * i as f64 / s as f64).cos())) as f32; }
                }
            }
            let d = (delay * sr / 1000.0).floor() as usize;
            if d > 0 {
                for x in [&mut *a, &mut *b] {
                    let mut y = vec![0f32; n];
                    if d < n { y[d..].copy_from_slice(&x[..n - d]); }
                    *x = y;
                }
            }
        }
        for (layer, lvl) in [(&mut ly.sub, p[SUB_LVL]), (&mut ly.tr, p[TR_LVL]), (&mut ly.body, p[BODY_LVL]), (&mut ly.spice, p[SPICE_LVL]), (&mut ly.synth, p[SYN_LVL])] {
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
        let shot_len = (p[LEN] * sr / 1000.0).floor() as usize;
        let mut onsets = vec![0f64];
        for s in 1..shots {
            let frac = (s - 1) as f64 / ((shots - 1).max(1)) as f64;
            let iv = p[B_RATE] * 2f64.powf(-p[B_ACCEL] * frac * 1.5);
            onsets.push(onsets[s - 1] + iv.max(4.0) * sr / 1000.0);
        }
        let total = onsets[shots - 1].floor() as usize + shot_len;
        let z = || (vec![0f32; total], vec![0f32; total]);
        // sub, tr, body, synth, spice, smp1..3
        let mut acc: Vec<Stereo> = (0..8).map(|_| z()).collect();
        let mut rr = Rng::new(to_i32(p[SEED]) as f64 * 7.0 + 77.0);
        for s in 0..shots {
            let q = if shots > 1 { Self::shot_params(p, s, &mut rr) } else { p.clone() };
            let gl = if shots > 1 && s < shots - 1 { ((onsets[s + 1] - onsets[s]) * p[B_GATE] + 0.003 * sr).floor() } else { shot_len as f64 };
            let n = shot_len.min(gl.max(64.0) as usize);
            let ly = self.render_shot(&q, n, sr);
            let fade = n.min((0.002 * sr).floor() as usize);
            let o = onsets[s].floor() as usize;
            let all = [&ly.sub, &ly.tr, &ly.body, &ly.synth, &ly.spice, &ly.smp[0], &ly.smp[1], &ly.smp[2]];
            for (k, lay) in all.into_iter().enumerate() {
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
        }
        // tops = tr + body + synth + spice (+ samples routed to mids), through the crunch chain
        let (mut tl, mut tr) = z();
        let mut any_top = false;
        for (k, lvl) in [(1, p[TR_LVL]), (2, p[BODY_LVL]), (3, p[SYN_LVL]), (4, p[SPICE_LVL])] {
            if lvl > -59.0 { add_scaled(&mut tl, &acc[k].0, 1.0); add_scaled(&mut tr, &acc[k].1, 1.0); any_top = true; }
        }
        for k in 0..3 {
            if p[S1_LVL + 10 * k] <= -59.0 || self.samples[k].is_none() { continue; }
            let sm = acc[5 + k].clone();
            if js_round(p[S1_ROUTE + 10 * k]) == 1.0 { // clean: alongside the sub
                add_scaled(&mut acc[0].0, &sm.0, 1.0); add_scaled(&mut acc[0].1, &sm.1, 1.0);
            } else {
                add_scaled(&mut tl, &sm.0, 1.0); add_scaled(&mut tr, &sm.1, 1.0); any_top = true;
            }
        }
        if any_top { crunch(&mut tl, &mut tr, p, sr, &mut |s| self.ir(s, sr)); }
        let tops = if any_top { Some((tl, tr)) } else { None };
        // reverb send from the tops; level relative to the send's RMS
        let rev = if p[REV_LVL] <= -59.0 { None } else {
            let send: Buf = match &tops { Some((a, b)) => (0..total).map(|i| (0.5 * (a[i] as f64 + b[i] as f64)) as f32).collect(), None => vec![0f32; total] };
            let h = self.ir(&IrSpec { ty: p[REV_TYPE], size: p[REV_SIZE], ch: p[REV_CHAR], tone: p[REV_TONE], pre: p[REV_PRE], seed: p[SEED] }, sr);
            let (mut a, mut b) = convolve_stereo(&send, &h.0, &h.1, total);
            for x in [&mut a, &mut b] { biquad(x, Bq::Hp, p[REV_HP], 0.707, 0.0, sr); biquad(x, Bq::Hp, p[REV_HP], 0.707, 0.0, sr); }
            let (rs, rv) = (rms(&send), rms2(&a, &b));
            let g = if rv > 1e-9 { (rs / rv) * db_to_lin(p[REV_LVL]) } else { 0.0 };
            scale(&mut a, g); scale(&mut b, g);
            Some((a, b))
        };
        bus(&acc[0], tops.as_ref(), rev.as_ref(), p, sr)
    }
}
