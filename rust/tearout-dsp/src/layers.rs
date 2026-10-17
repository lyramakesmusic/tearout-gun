// Sound layers: sub, transient, body, synth, spice, dropped samples.
use crate::dsp::*;
use crate::id::*;
use crate::Params;

pub type Stereo = (Buf, Buf);

pub fn render_sub(p: &Params, n: usize, sr: f64, r: &mut Rng) -> Buf {
    let fe = midi_hz(p[NOTE] + p[SUB_TUNE]);
    let d = p[SUB_DIVE] / 12.0;
    let tau = p[SUB_TAU] / 1000.0;
    let k = p[SUB_K];
    let c = p[SUB_CLICK] / 12.0;
    let tc = p[SUB_CLICK_MS] / 1000.0;
    let mut x = vec![0f32; n];
    let mut ph = 0f64;
    for i in 0..n {
        let t = i as f64 / sr;
        let f = fe * 2f64.powf(d * (-(t / tau).powf(k)).exp() + c * (-t / tc).exp());
        ph += TAU * f.min(sr * 0.45) / sr;
        x[i] = ph.sin() as f32;
    }
    if p[SUB_NOISE] > -59.0 {
        // noise crunched together with the sine through the same drive
        let mut nz = white_noise(n, r);
        biquad(&mut nz, Bq::Hp, p[SUB_NOISE_HP], 0.707, 0.0, sr);
        norm_rms(&mut nz, 0.7071, n);
        let g = db_to_lin(p[SUB_NOISE]);
        for i in 0..n { x[i] = (x[i] as f64 + g * nz[i] as f64) as f32; }
    }
    let e = envelope(n, sr, p[SUB_ATT], p[SUB_DEC], p[SUB_HOLD], p[SUB_REL], js_round(p[SUB_SHAPE]) as i64);
    mul_env(&mut x, &e);
    shape(&mut x, p[SUB_DRIVE], p[SUB_HARD], 0.0);
    norm_rms(&mut x, 0.25, active_len(sr, p[SUB_ATT], p[SUB_HOLD], p[SUB_REL]));
    x
}

// polyBLEP residual for a discontinuity at phase 0 (t in [0,1), dt = phase increment)
pub fn blep(t: f64, dt: f64) -> f64 {
    if t < dt { let t = t / dt; t + t - t * t - 1.0 }
    else if t > 1.0 - dt { let t = (t - 1.0) / dt; t * t + t + t + 1.0 }
    else { 0.0 }
}

// Cytomic TPT state-variable lowpass whose cutoff glides exponentially from f0 to f1 (coefficients every 16 samples)
fn svf_sweep_lp(x: &mut [f32], l0: f64, l1: f64, tau: f64, k: f64, sr: f64) {
    let (mut ic1, mut ic2, mut a1, mut a2, mut a3) = (0f64, 0f64, 0f64, 0f64, 0f64);
    for i in 0..x.len() {
        if i & 15 == 0 {
            let f = (l1 + (l0 - l1) * (-(i as f64 / sr) / tau).exp()).exp();
            let g = (PI * f.min(sr * 0.45) / sr).tan();
            a1 = 1.0 / (1.0 + g * (g + k)); a2 = g * a1; a3 = g * a2;
        }
        let v3 = x[i] as f64 - ic2;
        let v1 = a1 * ic1 + a2 * v3;
        let v2 = ic2 + a2 * ic1 + a3 * v3;
        ic1 = 2.0 * v1 - ic1; ic2 = 2.0 * v2 - ic2;
        x[i] = v2 as f32;
    }
}

pub fn render_tr(p: &Params, n: usize, sr: f64, r: &mut Rng) -> Buf {
    let mut x = white_noise(n, r);
    color_noise(&mut x, p[TR_COLOR], sr);
    norm_rms(&mut x, 0.25, n);
    if p[TR_CLICK] > -59.0 {
        let g = db_to_lin(p[TR_CLICK]);
        let tau = p[TR_CLICK_TAU] / 1000.0;
        let mut ph = 0f64;
        for i in 0..n {
            let t = i as f64 / sr;
            let f = 40.0 + (p[TR_CLICK_F0] - 40.0) * (-t / tau).exp();
            ph += TAU * f / sr;
            x[i] = (x[i] as f64 + g * 0.35 * ph.sin() * (-t / (tau * 4.0)).exp()) as f32;
        }
    }
    if p[TR_ZAP] > -59.0 { // harmonic (saw) sweep, exponential from zap-from to zap-to
        let g = db_to_lin(p[TR_ZAP]) * 0.5;
        let (l0, l1) = (p[TR_ZAP_F0].ln(), p[TR_ZAP_F1].ln());
        let zt = p[TR_ZAP_MS] / 1000.0;
        let mut ph = 0f64;
        for i in 0..n {
            let f = (l1 + (l0 - l1) * (-(i as f64 / sr) / zt).exp()).exp();
            let dt = f.min(sr * 0.45) / sr;
            ph += dt; if ph >= 1.0 { ph -= 1.0; }
            x[i] = (x[i] as f64 + g * (2.0 * ph - 1.0 - blep(ph, dt))) as f32;
        }
    }
    let hp = coef(Bq::Hp, p[TR_HP], 0.707, 0.0, sr);
    cascade(&mut [&mut x], &[hp, hp, coef(Bq::Lp, p[TR_LP], 0.707, 0.0, sr)]);
    if p[TR_CLOSE] < 19900.0 { // lowpass closing from the lowpass setting to close-to: highs end first
        svf_sweep_lp(&mut x, p[TR_LP].ln(), p[TR_CLOSE].min(p[TR_LP]).ln(), p[TR_CLOSE_MS] / 1000.0, 1.2, sr);
    }
    biquad(&mut x, Bq::Peak, p[TR_PK_F], p[TR_PK_Q], p[TR_PK_G], sr);
    { // envelope, optionally retriggered (stutter) with each repeat a little quieter
        let e = envelope(n, sr, p[TR_ATT], p[TR_DEC], p[TR_HOLD], p[TR_REL], js_round(p[TR_SHAPE]) as i64);
        let s = js_round(p[TR_STUTTER]) as i64;
        if s > 1 {
            let gap = (p[TR_STUT_MS] * sr / 1000.0).floor() as usize;
            let mut sum = vec![0f32; n];
            for k in 0..s {
                let g = 0.8f64.powf(k as f64);
                let o = k as usize * gap;
                let mut i = 0;
                while i + o < n {
                    let gate = if k < s - 1 && i > gap { 0.0 } else { 1.0 };
                    sum[i + o] = (sum[i + o] as f64).max(g * e[i] as f64 * gate) as f32;
                    i += 1;
                }
            }
            mul_env(&mut x, &sum);
        } else {
            mul_env(&mut x, &e);
        }
    }
    shape(&mut x, p[TR_DRIVE], 0.4, 0.0);
    biquad(&mut x, Bq::Hp, p[TR_HP] * 0.7, 0.707, 0.0, sr);
    norm_rms(&mut x, 0.25, active_len(sr, p[TR_ATT], p[TR_HOLD], p[TR_REL]));
    x
}

// time-varying multiplier for laser drop
fn drop_mul(p: &Params, t: f64) -> f64 {
    if p[BODY_DROP] > 0.01 { 2f64.powf((p[BODY_DROP] / 12.0) * (-t / (p[BODY_DROP_TAU] / 1000.0)).exp()) } else { 1.0 }
}
// key tracking: snap a frequency to the nearest harmonic of the note
fn keyed(p: &Params, f: f64) -> f64 {
    if js_round(p[BODY_KEY]) == 0.0 { return f; }
    let h = midi_hz(p[NOTE]);
    h * js_round(f / h).max(1.0)
}
// vowel formants (F1, F2, F3), a e i o u
const VOWELS: [[f64; 3]; 5] = [[800.0, 1150.0, 2900.0], [400.0, 1600.0, 2700.0], [300.0, 2250.0, 3000.0], [450.0, 800.0, 2830.0], [325.0, 700.0, 2530.0]];
fn formant(x: &mut [f32], pos: f64, mix: f64, sr: f64) {
    if mix < 0.001 { return; }
    let i = (pos.floor() as i64).min(3).max(0) as usize;
    let t = (pos - i as f64).min(1.0);
    let mut y = vec![0f32; x.len()];
    for k in 0..3 {
        let f = VOWELS[i][k] * (VOWELS[i + 1][k] / VOWELS[i][k]).powf(t);
        let mut b = x.to_vec();
        biquad(&mut b, Bq::Bp, f, 7.0, 0.0, sr);
        let g = [1.0, 0.7, 0.45][k];
        for j in 0..y.len() { y[j] = (y[j] as f64 + g * b[j] as f64) as f32; }
    }
    let rx = rms(x);
    let mut ry = rms(&y);
    if ry == 0.0 { ry = 1.0; }
    for j in 0..x.len() { x[j] = ((1.0 - mix) * x[j] as f64 + mix * y[j] as f64 * (rx / ry)) as f32; }
}

// modal bank (2-pole resonators with time-varying frequency), K modes stepped together; m[i] accumulates in mode order
fn modal_chunk<const K: usize>(p: &Params, sr: f64, exc: &[f32], ms: &[(f64, f64)], m: &mut [f32]) {
    let q = p[BODY_Q];
    let f0: [f64; K] = std::array::from_fn(|j| keyed(p, p[BODY_MODE_F]) * ms[j].0);
    let (mut y1, mut y2, mut c1, mut c2, mut gn) = ([0f64; K], [0f64; K], [0f64; K], [0f64; K], [0f64; K]);
    for i in 0..m.len() {
        if i & 15 == 0 {
            let dm = drop_mul(p, i as f64 / sr);
            for j in 0..K {
                let f = (f0[j] * dm).min(sr * 0.45);
                let rr = (-PI * (f / q) / sr).exp();
                c1[j] = 2.0 * rr * (TAU * f / sr).cos(); c2[j] = -rr * rr; gn[j] = (1.0 - rr * rr) * 0.5;
            }
        }
        let e = exc[i] as f64;
        let mut acc = m[i];
        for j in 0..K {
            let y = gn[j] * e + c1[j] * y1[j] + c2[j] * y2[j];
            y2[j] = y1[j]; y1[j] = y;
            acc = (acc as f64 + ms[j].1 * y) as f32;
        }
        m[i] = acc;
    }
}
fn body_modes(p: &Params, n: usize, sr: f64, exc: &[f32], modes: &[(f64, f64)]) -> Buf {
    let mut m = vec![0f32; n];
    for ch in modes.chunks(4) {
        match ch.len() { 4 => modal_chunk::<4>(p, sr, exc, ch, &mut m), 3 => modal_chunk::<3>(p, sr, exc, ch, &mut m), 2 => modal_chunk::<2>(p, sr, exc, ch, &mut m), _ => modal_chunk::<1>(p, sr, exc, ch, &mut m) }
    }
    norm_rms(&mut m, 0.25, n);
    m
}
// comb (feedback delay, cubic interp, damped, soft-limited loop)
fn body_comb(p: &Params, n: usize, sr: f64, exc: &[f32], comb_f: f64) -> Buf {
    const NB: usize = 1 << 14;
    let mut c = vec![0f32; n];
    let mut buf = vec![0f32; NB];
    let damp = p[BODY_COMB_DAMP] * 0.9;
    let (mut w, mut lp) = (0usize, 0f64);
    let at = |buf: &Vec<f32>, k: i64| buf[(k & (NB as i64 - 1)) as usize] as f64;
    for i in 0..n {
        let f = (comb_f * drop_mul(p, i as f64 / sr)).min(sr * 0.4);
        let d = ((NB - 4) as f64).min((sr / f).max(2.0));
        let rp = w as f64 - d + (NB * 2) as f64;
        let i0 = rp.floor();
        let fr = rp - i0;
        let i0 = i0 as i64;
        let (xm1, x0, x1, x2) = (at(&buf, i0 - 1), at(&buf, i0), at(&buf, i0 + 1), at(&buf, i0 + 2));
        let del = x0 + 0.5 * fr * (x1 - xm1 + fr * (2.0 * xm1 - 5.0 * x0 + 4.0 * x1 - x2 + fr * (3.0 * (x0 - x1) + x2 - xm1)));
        lp = (1.0 - damp) * del + damp * lp;
        let y = exc[i] as f64 * 0.3 + p[BODY_COMB_FB] * lp.tanh();
        buf[w & (NB - 1)] = y as f32; w += 1; c[i] = y as f32;
    }
    norm_rms(&mut c, 0.25, n);
    c
}
// FM tone
fn body_fm(p: &Params, n: usize, sr: f64) -> Buf {
    let mut fm = vec![0f32; n];
    let base = midi_hz(p[NOTE]) * p[BODY_FM_RATIO];
    let (mut pc, mut pm) = (0f64, 0f64);
    for i in 0..n {
        let t = i as f64 / sr;
        let fc = (base * drop_mul(p, t)).min(sr * 0.45);
        pc += TAU * fc / sr;
        pm += TAU * fc * p[BODY_FM_MOD] / sr;
        fm[i] = (pc + p[BODY_FM_IDX] * (-t / (p[BODY_FM_TAU] / 1000.0)).exp() * pm.sin()).sin() as f32;
    }
    norm_rms(&mut fm, 0.25, n);
    fm
}

fn render_body_channel(p: &Params, n: usize, sr: f64, r: &mut Rng, modes: &[(f64, f64)]) -> Buf {
    let mut exc = white_noise(n, r);
    color_noise(&mut exc, p[BODY_COLOR], sr);
    norm_rms(&mut exc, 0.25, n);
    // modal bank, comb and FM tone read only the excitation: each renders on its own thread, summed in order
    let (m, c, fm) = std::thread::scope(|sc| {
        let exc = &exc;
        let m = (p[BODY_MODES] > -59.0).then(|| sc.spawn(move || body_modes(p, n, sr, exc, modes)));
        let c = (p[BODY_COMB] > -59.0).then(|| sc.spawn(move || body_comb(p, n, sr, exc, keyed(p, p[BODY_COMB_F]))));
        let fm = (p[BODY_FM] > -59.0).then(|| body_fm(p, n, sr));
        (m.map(|h| h.join().unwrap()), c.map(|h| h.join().unwrap()), fm)
    });
    let mut out = vec![0f32; n];
    if p[BODY_NOISE] > -59.0 { add_scaled(&mut out, &exc, db_to_lin(p[BODY_NOISE])); }
    if let Some(m) = m { add_scaled(&mut out, &m, db_to_lin(p[BODY_MODES])); }
    if let Some(c) = c { add_scaled(&mut out, &c, db_to_lin(p[BODY_COMB])); }
    if let Some(fm) = fm { add_scaled(&mut out, &fm, db_to_lin(p[BODY_FM])); }
    // open-close SVF (Cytomic TPT), blended with dry
    if p[BODY_FLT_MIX] > 0.001 {
        let k = 2.0 - 2.0 * p[BODY_FLT_RES];
        let ty = js_round(p[BODY_FLT_TYPE]) as i64;
        let mix = p[BODY_FLT_MIX];
        let (mut ic1, mut ic2, mut a1, mut a2, mut a3) = (0f64, 0f64, 0f64, 0f64, 0f64);
        let (lf0, lf1, ft) = (p[BODY_FLT_F0].ln(), p[BODY_FLT_F1].ln(), p[BODY_FLT_TAU] / 1000.0);
        for i in 0..n {
            if i & 15 == 0 {
                let f = (lf1 + (lf0 - lf1) * (-(i as f64 / sr) / ft).exp()).exp();
                let g = (PI * f.min(sr * 0.45) / sr).tan();
                a1 = 1.0 / (1.0 + g * (g + k)); a2 = g * a1; a3 = g * a2;
            }
            let v0 = out[i] as f64;
            let v3 = v0 - ic2;
            let v1 = a1 * ic1 + a2 * v3;
            let v2 = ic2 + a2 * ic1 + a3 * v3;
            ic1 = 2.0 * v1 - ic1; ic2 = 2.0 * v2 - ic2;
            let hp = v0 - k * v1 - v2;
            let y = match ty { 0 => v2, 1 => v1 * k, 2 => hp, _ => v2 + hp };
            out[i] = ((1.0 - mix) * v0 + mix * y) as f32;
        }
    }
    formant(&mut out, p[BODY_VOWEL], p[BODY_VOWEL_MIX], sr);
    let e = envelope(n, sr, p[BODY_ATT], p[BODY_DEC], p[BODY_HOLD], p[BODY_REL], js_round(p[BODY_SHAPE]) as i64);
    mul_env(&mut out, &e);
    norm_rms(&mut out, 0.25, active_len(sr, p[BODY_ATT], p[BODY_HOLD], p[BODY_REL]));
    shape(&mut out, p[BODY_DRIVE], 0.3, p[BODY_FOLD]);
    let hp = coef(Bq::Hp, p[BODY_HP], 0.707, 0.0, sr);
    cascade(&mut [&mut out], &[hp, hp]);
    out
}

// Partial ratio tables: membrane / free bar / clamped plate / cymbal / bell (re prime) / TR-808 metal oscillators.
const MODE_SETS: [&[f64]; 6] = [
    &[1.0, 1.593, 2.136, 2.295, 2.653, 2.917, 3.155, 3.501, 3.598, 3.647, 4.06, 4.23],
    &[1.0, 2.757, 5.404, 8.933, 13.344, 18.64],
    &[1.0, 2.081, 3.414, 3.893, 5.2, 6.1],
    &[1.0, 3.3, 8.9, 12.6],
    &[0.5, 1.0, 1.183, 1.506, 2.0, 2.514, 3.011, 4.166],
    &[1.0, 1.483, 1.8, 2.546, 2.63, 3.897],
];
fn mode_table(p: &Params, r: &mut Rng) -> Vec<(f64, f64)> {
    let si = js_round(p[BODY_MODE_SET]) as i64;
    let set: Option<&[f64]> = if si >= 1 && si <= 6 { Some(MODE_SETS[(si - 1) as usize]) } else { None };
    let b = 0.6 * p[BODY_METAL] * p[BODY_METAL];
    let jit = 0.35 * p[BODY_METAL];
    let mut out = vec![];
    let mut i = 1usize;
    while i as f64 <= p[BODY_NMODES] {
        let fi = i as f64;
        let base = match set {
            None => fi * (1.0 + b * fi * fi).sqrt(),
            Some(s) => if i <= s.len() { s[i - 1] } else { s[s.len() - 1] * (fi / s.len() as f64).powf(1.5) },
        };
        let ratio = base * (1.0 + (if set.is_some() { 0.5 } else { 1.0 }) * jit * (r.next() - 0.5) * (if i > 1 { 1.0 } else { 0.0 }));
        out.push((ratio, 10f64.powf((p[BODY_MODE_TILT] * ratio.log2()) / 20.0)));
        i += 1;
    }
    out
}

pub fn render_body(p: &Params, n: usize, sr: f64, seed: f64) -> Stereo {
    let modes = mode_table(p, &mut Rng::new(12345.0));
    if p[BODY_WIDTH] < 0.01 { let mid = render_body_channel(p, n, sr, &mut Rng::new(seed * 101.0 + 1.0), &modes); return (mid.clone(), mid); }
    let (mid, side) = std::thread::scope(|sc| {
        let side = sc.spawn(|| render_body_channel(p, n, sr, &mut Rng::new(seed * 101.0 + 2.0), &modes));
        (render_body_channel(p, n, sr, &mut Rng::new(seed * 101.0 + 1.0), &modes), side.join().unwrap())
    });
    let w = p[BODY_WIDTH];
    let gm = 1.0 / (1.0 + w * w).sqrt();
    let (mut l, mut rr) = (vec![0f32; n], vec![0f32; n]);
    for i in 0..n {
        let (m, s) = (mid[i] as f64, side[i] as f64);
        l[i] = (gm * (m + w * s)) as f32;
        rr[i] = (gm * (m - w * s)) as f32;
    }
    (l, rr)
}

fn synth_voices(p: &Params, n: usize, sr: f64, r: &mut Rng, side: bool) -> Buf {
    let wave = js_round(p[SYN_WAVE]) as i64;
    let vv = if wave == 2 { js_round(p[SYN_VOICES]).max(3.0) } else { js_round(p[SYN_VOICES]) } as i64;
    let root = midi_hz(p[NOTE] + 12.0 * js_round(p[SYN_OCT]) + p[SYN_TUNE]);
    let d = p[SYN_PITCH] / 12.0;
    let pt = p[SYN_PITCH_MS] / 1000.0;
    let mut x = vec![0f32; n];
    // pitch dive multiplier, the same for every voice
    let curve: Vec<f64> = (0..n).map(|i| 2f64.powf(d * (-(i as f64) / sr / pt).exp())).collect();
    let mut stacks = vec![(root, 1.0)];
    if p[SYN_INT_MIX] > 0.001 { stacks.push((root * 2f64.powf(js_round(p[SYN_INT]) / 12.0), p[SYN_INT_MIX])); }
    for &(base, sg) in &stacks {
        for v in 0..vv {
            let spread = if vv > 1 { (v as f64 / (vv - 1) as f64) * 2.0 - 1.0 } else { 0.0 };
            let cents = spread * p[SYN_DETUNE] * 50.0 * if side { -1.0 } else { 1.0 };
            let mul = 2f64.powf(cents / 1200.0);
            let w = if vv > 1 && wave == 2 { if v == (vv >> 1) { 1.0 } else { 0.7 } } else { 1.0 };
            let amp = sg * w / (vv as f64).sqrt();
            let mut ph = r.next();
            let bm = base * mul;
            for i in 0..n {
                let f = (bm * curve[i]).min(sr * 0.45);
                let dt = f / sr;
                ph += dt; if ph >= 1.0 { ph -= 1.0; }
                let y = if wave == 0 || wave == 2 {
                    2.0 * ph - 1.0 - blep(ph, dt)
                } else {
                    let pw = if wave == 1 { 0.5 } else { 0.18 };
                    let ph2 = ph - pw + if ph < pw { 1.0 } else { 0.0 };
                    (if ph < pw { 1.0 } else { -1.0 }) + blep(ph, dt) - blep(ph2, dt)
                };
                x[i] = (x[i] as f64 + amp * y) as f32;
            }
        }
    }
    // filter sweep (SVF lowpass)
    svf_sweep_lp(&mut x, p[SYN_FLT_F0].ln(), p[SYN_FLT_F1].ln(), p[SYN_FLT_TAU] / 1000.0, 2.0 - 2.0 * p[SYN_FLT_RES], sr);
    formant(&mut x, p[SYN_VOWEL], p[SYN_VOWEL_MIX], sr);
    let e = envelope(n, sr, p[SYN_ATT], p[SYN_DEC], p[SYN_HOLD], p[SYN_REL], js_round(p[SYN_SHAPE]) as i64);
    mul_env(&mut x, &e);
    norm_rms(&mut x, 0.25, active_len(sr, p[SYN_ATT], p[SYN_HOLD], p[SYN_REL]));
    shape(&mut x, p[SYN_DRIVE], 0.4, 0.0);
    biquad(&mut x, Bq::Hp, 120.0, 0.707, 0.0, sr);
    x
}
pub fn render_synth(p: &Params, n: usize, sr: f64, seed: f64) -> Stereo {
    if p[SYN_WIDTH] < 0.01 { let a = synth_voices(p, n, sr, &mut Rng::new(seed * 23.0 + 1.0), false); return (a.clone(), a); }
    let (a, b) = std::thread::scope(|sc| {
        let b = sc.spawn(|| synth_voices(p, n, sr, &mut Rng::new(seed * 23.0 + 2.0), true));
        (synth_voices(p, n, sr, &mut Rng::new(seed * 23.0 + 1.0), false), b.join().unwrap())
    });
    let w = p[SYN_WIDTH];
    let (mut l, mut r) = (vec![0f32; n], vec![0f32; n]);
    for i in 0..n {
        let (x, y) = (a[i] as f64, b[i] as f64);
        let (m, d) = (0.5 * (x + y), 0.5 * (x - y));
        l[i] = (m + w * d) as f32; r[i] = (m - w * d) as f32;
    }
    (l, r)
}

pub fn render_spice(p: &Params, n: usize, sr: f64, r: &mut Rng) -> Buf {
    let mut x = vec![0f32; n];
    let ty = js_round(p[SPICE_TYPE]) as i64;
    let len = (p[SPICE_LEN] * sr / 1000.0).floor() as usize;
    let lenf = len as f64;
    if ty == 0 { // comb ping
        const NB: usize = 1 << 14;
        let d = (sr / p[SPICE_F]).max(2.0);
        let fb = 0.001f64.powf(d / lenf.max(1.0));
        let mut buf = vec![0f32; NB];
        let mut w = 0usize;
        for i in 0..n {
            let rp = w as f64 - d + (NB * 2) as f64;
            let i0 = rp.floor();
            let fr = rp - i0;
            let i0 = i0 as usize;
            let del = buf[i0 & (NB - 1)] as f64 * (1.0 - fr) + buf[(i0 + 1) & (NB - 1)] as f64 * fr;
            let ex = if i < 48 { r.next() * 2.0 - 1.0 } else { 0.0 };
            let y = ex + fb * del;
            buf[w & (NB - 1)] = y as f32; w += 1; x[i] = y as f32;
        }
    } else if ty == 1 { // random sine bleeps
        let count = js_round(p[SPICE_DENSITY] * p[SPICE_LEN] / 1000.0).max(1.0) as usize;
        for _ in 0..count {
            let st = (r.next().powf(1.8) * lenf * 0.6).floor() as usize;
            let bl = ((0.004 + 0.03 * r.next()) * sr).floor() as usize;
            let f = p[SPICE_F] * 2f64.powf((r.next() - 0.5) * 3.0);
            let amp = 0.4 + 0.6 * r.next();
            let mut i = 0;
            while i < bl && st + i < n {
                let fi = i as f64;
                x[st + i] = (x[st + i] as f64 + amp * (TAU * f * fi / sr).sin() * (PI * fi / bl as f64).sin()) as f32;
                i += 1;
            }
        }
    } else if ty == 2 { // crackle
        let prob = p[SPICE_DENSITY] * 4.0 / sr;
        let mut i = 0;
        while i < len && i < n {
            if r.next() < prob { x[i] = ((r.next() * 2.0 - 1.0) * (1.0 - i as f64 / lenf)) as f32; }
            i += 1;
        }
        biquad(&mut x, Bq::Bp, p[SPICE_F], 0.8, 0.0, sr);
    } else if ty == 4 { // TR-808 metal: six squares -> two bandpasses -> highpass, scaled by freq
        let sc = p[SPICE_F] / 3440.0;
        let fr: Vec<f64> = [205.3, 304.4, 369.6, 522.7, 540.0, 800.0].iter().map(|f| f * sc).collect();
        for i in 0..n {
            let mut s = 0f64;
            for &f in &fr { s += if ((i as f64 * f / sr) % 1.0) < 0.5 { 1.0 } else { -1.0 }; }
            x[i] = (s / 6.0) as f32;
        }
        let (mut a, mut b) = (x.clone(), x.clone());
        biquad(&mut a, Bq::Bp, 3440.0 * sc, 3.0, 0.0, sr);
        biquad(&mut b, Bq::Bp, 7100.0 * sc, 3.0, 0.0, sr);
        let dk = 6.9 / lenf.max(1.0);
        for i in 0..n { x[i] = ((a[i] as f64 + 0.7 * b[i] as f64) * (-(i as f64) * dk).exp()) as f32; }
    } else { // metal hit
        let mut ex = vec![0f32; n];
        for i in 0..96 { let v = r.next() * 2.0 - 1.0; if i < n { ex[i] = v as f32; } }
        for k in 1..=8 {
            let kf = k as f64;
            let f = (p[SPICE_F] * kf * (1.0 + 0.3 * kf * kf).sqrt() * (1.0 + 0.2 * (r.next() - 0.5))).min(sr * 0.45);
            let rr = 0.001f64.powf(1.0 / (lenf * (1.2 - kf / 10.0)).max(1.0));
            let (c1, c2) = (2.0 * rr * (TAU * f / sr).cos(), -rr * rr);
            let (mut y1, mut y2) = (0f64, 0f64);
            for i in 0..n {
                let y = (1.0 - rr) * ex[i] as f64 + c1 * y1 + c2 * y2;
                y2 = y1; y1 = y;
                x[i] = (x[i] as f64 + y / kf) as f32;
            }
        }
    }
    // every spice type lives inside its window and fades across it
    for i in 0..n {
        let g = if i < len { (1.0 - i as f64 / lenf).powf(1.5) } else { 0.0 };
        x[i] = (x[i] as f64 * g) as f32;
    }
    biquad(&mut x, Bq::Hp, p[SPICE_HP], 0.707, 0.0, sr);
    norm_rms(&mut x, 0.25, len.max(480));
    x
}

// slot k (1..3): dropped audio, pitched, gated, filtered, driven
pub fn render_sample(p: &Params, src: &Stereo, k: usize, n: usize, sr: f64) -> Stereo {
    let b = S1_LVL + 10 * (k - 1);
    let (lvl, pitch, key, start, slen, rel, hp, lp, drive) = (p[b], p[b + 1], p[b + 2], p[b + 3], p[b + 4], p[b + 5], p[b + 6], p[b + 7], p[b + 8]);
    let st = pitch + if js_round(key) != 0.0 { p[NOTE] - 24.0 } else { 0.0 };
    let rate = 2f64.powf(st / 12.0);
    let o0 = start * sr / 1000.0;
    let len = (slen * sr / 1000.0).floor();
    let rl = (rel * sr / 1000.0).max(1.0);
    let mut out = (vec![0f32; n], vec![0f32; n]);
    for c in 0..2 {
        let s = if c == 0 { &src.0 } else { &src.1 };
        let y = if c == 0 { &mut out.0 } else { &mut out.1 };
        for i in 0..n {
            let fi = i as f64;
            let t = o0 + fi * rate;
            let j = t.floor();
            if j + 1.0 >= s.len() as f64 { break; }
            let g = if fi < len { 1.0 } else if fi < len + rl { 0.5 + 0.5 * (PI * (fi - len) / rl).cos() } else { 0.0 };
            if g == 0.0 { break; }
            let ju = j as usize;
            y[i] = (g * (s[ju] as f64 + (t - j) * (s[ju + 1] as f64 - s[ju] as f64))) as f32;
        }
        if hp > 21.0 { biquad(y, Bq::Hp, hp, 0.707, 0.0, sr); biquad(y, Bq::Hp, hp, 0.707, 0.0, sr); }
        if lp < 19900.0 { biquad(y, Bq::Lp, lp, 0.707, 0.0, sr); }
        if drive > 0.05 { shape(y, drive, 0.4, 0.0); }
    }
    let g = db_to_lin(lvl);
    scale(&mut out.0, g);
    scale(&mut out.1, g);
    out
}
