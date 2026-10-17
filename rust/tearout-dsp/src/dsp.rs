// Shared DSP: RNG, envelopes, filters, waveshaper, FFT convolution, IR synthesis.
// Math runs in f64; buffers are f32, matching the JS engine (Float32Array storage, double arithmetic).
pub use std::f64::consts::{PI, TAU};
use std::f64::consts::{FRAC_1_SQRT_2, LN_2};

pub type Buf = Vec<f32>;

// ECMAScript ToUint32 / ToInt32 of a double
pub fn to_u32(x: f64) -> u32 {
    if !x.is_finite() { return 0; }
    (x.trunc() as i64).rem_euclid(1 << 32) as u32
}
pub fn to_i32(x: f64) -> i32 { to_u32(x) as i32 }
// ECMAScript Math.round: halves round toward +inf
pub fn js_round(x: f64) -> f64 {
    let f = x.floor();
    if x - f >= 0.5 { f + 1.0 } else { f }
}

// mulberry32, bit-identical to the JS rng()
pub struct Rng(u32);
impl Rng {
    pub fn new(seed: f64) -> Self {
        let a = to_u32(seed);
        Rng(if a == 0 { 1 } else { a })
    }
    pub fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_add(0x6d2b79f5);
        let a = self.0;
        let mut t = (a ^ (a >> 15)).wrapping_mul(1 | a);
        t = t.wrapping_add((t ^ (t >> 7)).wrapping_mul(61 | t)) ^ t;
        ((t ^ (t >> 14)) as f64) / 4294967296.0
    }
}

pub fn db_to_lin(d: f64) -> f64 { 10f64.powf(d / 20.0) }
pub fn midi_hz(m: f64) -> f64 { 440.0 * 2f64.powf((m - 69.0) / 12.0) }

// attack (linear) -> exp decay while holding -> raised-cosine gate. Times in ms.
// shape 1 (spike): drops to 35% over the first 10% of hold, then a straight ramp to 0 at hold end.
// shape 2 (dome): quarter-cosine from 1 to 0 across the hold.
pub fn envelope(n: usize, sr: f64, att: f64, dec: f64, hold: f64, rel: f64, shape: i64) -> Buf {
    let mut e = vec![0f32; n];
    let a = att * sr / 1000.0;
    let h = hold * sr / 1000.0;
    let r = (rel * sr / 1000.0).max(1.0);
    let k = 1000.0 / (dec * sr);
    let span = (h - a).max(1.0);
    for i in 0..n {
        let fi = i as f64;
        let mut v = if fi < a { fi / a } else { 1.0 };
        if shape == 0 {
            v *= (-(fi - fi.min(a)) * k).exp();
        } else if fi >= a {
            let x = ((fi - a) / span).min(1.0);
            v = if shape == 1 {
                if x < 0.1 { 1.0 - 6.5 * x } else { 0.35 * (1.0 - (x - 0.1) / 0.9) }
            } else {
                (0.5 * PI * x).cos()
            };
        }
        if fi > h {
            let x = (fi - h) / r;
            v *= if x >= 1.0 { 0.0 } else { 0.5 + 0.5 * (PI * x).cos() };
        }
        e[i] = v as f32;
    }
    e
}
pub fn active_len(sr: f64, att: f64, hold: f64, rel: f64) -> usize { (((att + hold + rel) * sr) / 1000.0).ceil() as usize }

#[derive(Clone, Copy, PartialEq)]
pub enum Bq { Lp, Hp, Bp, Peak, Ls, Hs, Ap }

// RBJ biquad, in place
pub fn biquad(x: &mut [f32], ty: Bq, f: f64, q: f64, gain_db: f64, sr: f64) {
    let w = TAU * f.min(sr * 0.49) / sr;
    let (cw, sw) = (w.cos(), w.sin());
    let al = sw / (2.0 * q);
    let a = 10f64.powf(gain_db / 40.0);
    let (mut b0, mut b1, mut b2, a0, mut a1, mut a2);
    match ty {
        Bq::Lp => { b0 = (1.0 - cw) / 2.0; b1 = 1.0 - cw; b2 = b0; a0 = 1.0 + al; a1 = -2.0 * cw; a2 = 1.0 - al; }
        Bq::Hp => { b0 = (1.0 + cw) / 2.0; b1 = -(1.0 + cw); b2 = b0; a0 = 1.0 + al; a1 = -2.0 * cw; a2 = 1.0 - al; }
        Bq::Bp => { b0 = al; b1 = 0.0; b2 = -al; a0 = 1.0 + al; a1 = -2.0 * cw; a2 = 1.0 - al; }
        Bq::Peak => { b0 = 1.0 + al * a; b1 = -2.0 * cw; b2 = 1.0 - al * a; a0 = 1.0 + al / a; a1 = -2.0 * cw; a2 = 1.0 - al / a; }
        Bq::Ls | Bq::Hs => {
            let s = if ty == Bq::Ls { 1.0 } else { -1.0 };
            let sq = 2.0 * a.sqrt() * al;
            b0 = a * ((a + 1.0) - s * (a - 1.0) * cw + sq);
            b1 = s * 2.0 * a * ((a - 1.0) - s * (a + 1.0) * cw);
            b2 = a * ((a + 1.0) - s * (a - 1.0) * cw - sq);
            a0 = (a + 1.0) + s * (a - 1.0) * cw + sq;
            a1 = -s * 2.0 * ((a - 1.0) + s * (a + 1.0) * cw);
            a2 = (a + 1.0) + s * (a - 1.0) * cw - sq;
        }
        Bq::Ap => { b0 = 1.0 - al; b1 = -2.0 * cw; b2 = 1.0 + al; a0 = 1.0 + al; a1 = -2.0 * cw; a2 = 1.0 - al; }
    }
    b0 /= a0; b1 /= a0; b2 /= a0; a1 /= a0; a2 /= a0;
    let (mut x1, mut x2, mut y1, mut y2) = (0f64, 0f64, 0f64, 0f64);
    for v in x.iter_mut() {
        let xi = *v as f64;
        let y = b0 * xi + b1 * x1 + b2 * x2 - a1 * y1 - a2 * y2;
        x2 = x1; x1 = xi; y2 = y1; y1 = y;
        *v = y as f32;
    }
}
pub fn lr4(x: &mut [f32], ty: Bq, f: f64, sr: f64) {
    biquad(x, ty, f, FRAC_1_SQRT_2, 0.0, sr);
    biquad(x, ty, f, FRAC_1_SQRT_2, 0.0, sr);
}

// one-pole tilt: color<0 darkens (LP 20k->250 Hz), color>0 brightens (HP 20->5k Hz)
pub fn color_noise(x: &mut [f32], color: f64, sr: f64) {
    if color.abs() < 1e-3 { return; }
    if color < 0.0 {
        let fc = 20000.0 * (250.0f64 / 20000.0).powf(-color);
        let a = (-TAU * fc / sr).exp();
        let mut y = 0f64;
        for v in x.iter_mut() { y = (1.0 - a) * (*v as f64) + a * y; *v = y as f32; }
    } else {
        let fc = 20.0 * (5000.0f64 / 20.0).powf(color);
        let a = (-TAU * fc / sr).exp();
        let (mut y, mut xp) = (0f64, 0f64);
        for v in x.iter_mut() { let xi = *v as f64; y = a * (y + xi - xp); xp = xi; *v = y as f32; }
    }
}

// ADAA1 waveshaper: blend of tanh and hard clip, plus sine fold. In place.
fn logcosh(x: f64) -> f64 { let a = x.abs(); a + (-2.0 * a).exp().ln_1p() - LN_2 }
fn hc_f(x: f64) -> f64 { if x.abs() <= 1.0 { 0.5 * x * x } else { x.abs() - 0.5 } }
fn hc(x: f64) -> f64 { if x > 1.0 { 1.0 } else if x < -1.0 { -1.0 } else { x } }
pub fn shape(x: &mut [f32], drive_db: f64, hard: f64, fold: f64) {
    if drive_db <= 0.01 && fold <= 0.001 { return; }
    let g = db_to_lin(drive_db);
    let mut xp = 0f64;
    for s in x.iter_mut() {
        let mut v = *s as f64 * g;
        if fold > 0.0 {
            let k = 1.0 + 3.0 * fold;
            v = (1.0 - fold) * v + fold * (0.5 * PI * v * k).sin() * 1.2;
        }
        let d = v - xp;
        let (yt, yh);
        if d.abs() > 1e-5 { yt = (logcosh(v) - logcosh(xp)) / d; yh = (hc_f(v) - hc_f(xp)) / d; }
        else { let m = 0.5 * (v + xp); yt = m.tanh(); yh = hc(m); }
        xp = v;
        *s = ((1.0 - hard) * yt + hard * yh) as f32;
    }
}

pub fn rms_of(x: &[f32], n: usize) -> f64 {
    let n = n.min(x.len());
    let mut s = 0f64;
    for &v in &x[..n] { let v = v as f64; s += v * v; }
    (s / (n.max(1) as f64)).sqrt()
}
pub fn rms(x: &[f32]) -> f64 { rms_of(x, x.len()) }
pub fn norm_rms(x: &mut [f32], target: f64, n: usize) {
    let r = rms_of(x, n);
    if r > 1e-9 { let g = target / r; for v in x.iter_mut() { *v = (*v as f64 * g) as f32; } }
}
pub fn add_scaled(dst: &mut [f32], src: &[f32], g: f64) {
    for (d, &s) in dst.iter_mut().zip(src) { *d = (*d as f64 + s as f64 * g) as f32; }
}
pub fn mul_env(x: &mut [f32], e: &[f32]) { for (v, &g) in x.iter_mut().zip(e) { *v = (*v as f64 * g as f64) as f32; } }
pub fn white_noise(n: usize, r: &mut Rng) -> Buf { (0..n).map(|_| (r.next() * 2.0 - 1.0) as f32).collect() }
pub fn scale(x: &mut [f32], g: f64) { for v in x.iter_mut() { *v = (*v as f64 * g) as f32; } }
pub fn rms2(l: &[f32], r: &[f32]) -> f64 { let (a, b) = (rms(l), rms(r)); ((a * a + b * b) / 2.0).sqrt() }
pub fn peak2(l: &[f32], r: &[f32]) -> f64 {
    let mut pk = 0f64;
    for i in 0..l.len() { pk = pk.max((l[i] as f64).abs()).max((r[i] as f64).abs()); }
    pk
}

// radix-2 complex FFT, in place; inverse scales by 1/n
pub fn fft(re: &mut [f64], im: &mut [f64], inv: bool) {
    let n = re.len();
    let mut j = 0usize;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 { j ^= bit; bit >>= 1; }
        j ^= bit;
        if i < j { re.swap(i, j); im.swap(i, j); }
    }
    let mut len = 2;
    while len <= n {
        let ang = (if inv { 2.0 } else { -2.0 }) * PI / len as f64;
        let (wr, wi) = (ang.cos(), ang.sin());
        let half = len / 2;
        let mut i = 0;
        while i < n {
            let (mut cr, mut ci) = (1f64, 0f64);
            for j in 0..half {
                let a = i + j;
                let b = a + half;
                let tr = re[b] * cr - im[b] * ci;
                let ti = re[b] * ci + im[b] * cr;
                re[b] = re[a] - tr; im[b] = im[a] - ti; re[a] += tr; im[a] += ti;
                let t = cr * wr - ci * wi; ci = cr * wi + ci * wr; cr = t;
            }
            i += len;
        }
        len <<= 1;
    }
    if inv { let nf = n as f64; for i in 0..n { re[i] /= nf; im[i] /= nf; } }
}

// mono x convolved with stereo IR (hl, hr) via one complex FFT (IR packed as re/im)
pub fn convolve_stereo(x: &[f32], hl: &[f32], hr: &[f32], out_len: usize) -> (Buf, Buf) {
    let mut n = 1;
    while n < x.len() + hl.len() { n <<= 1; }
    let (mut xr, mut xi) = (vec![0f64; n], vec![0f64; n]);
    for (d, &s) in xr.iter_mut().zip(x) { *d = s as f64; }
    fft(&mut xr, &mut xi, false);
    let (mut h_r, mut h_i) = (vec![0f64; n], vec![0f64; n]);
    for (d, &s) in h_r.iter_mut().zip(hl) { *d = s as f64; }
    for (d, &s) in h_i.iter_mut().zip(hr) { *d = s as f64; }
    fft(&mut h_r, &mut h_i, false);
    for k in 0..n {
        let (a, b, c, d) = (xr[k], xi[k], h_r[k], h_i[k]);
        xr[k] = a * c - b * d; xi[k] = a * d + b * c;
    }
    fft(&mut xr, &mut xi, true);
    (xr[..out_len].iter().map(|&v| v as f32).collect(), xi[..out_len].iter().map(|&v| v as f32).collect())
}

// ---------------------------------------------------------------- IR synthesis
#[derive(Clone, Copy)]
pub struct IrSpec { pub ty: f64, pub size: f64, pub ch: f64, pub tone: f64, pub pre: f64, pub seed: f64 }
impl IrSpec {
    pub fn key(&self, sr: f64) -> String {
        format!("{:.3}|{:.3}|{:.3}|{:.3}|{:.3}|{:.3}|{}", self.ty, self.size, self.ch, self.tone, self.pre, self.seed, sr)
    }
}
pub fn make_ir(p: &IrSpec, sr: f64) -> (Buf, Buf) {
    let ty = js_round(p.ty) as i64;
    let (size, ch) = (p.size, p.ch);
    let n = ((size * 1.3 + p.pre + 5.0) * sr / 1000.0).ceil() as usize;
    let pre = (p.pre * sr / 1000.0).floor() as usize;
    let mut out = [vec![0f32; n], vec![0f32; n]];
    for c in 0..2 {
        let mut r = Rng::new(to_i32(p.seed) as f64 * 7919.0 + 17.0 + c as f64 * 104729.0);
        let h = &mut out[c];
        let t60 = size / 1000.0;
        let dk = 6.9078 / (t60 * sr);
        let noise_tail = |h: &mut Buf, r: &mut Rng, g: f64, dkm: f64| {
            for i in pre..n { h[i] = (h[i] as f64 + g * (r.next() * 2.0 - 1.0) * (-((i - pre) as f64) * dk * dkm).exp()) as f32; }
        };
        if ty == 0 || ty == 4 {
            // plate / room: decaying noise whose lowpass closes over time
            for i in pre..n { h[i] = ((r.next() * 2.0 - 1.0) * (-((i - pre) as f64) * dk).exp()) as f32; }
            let mut y = 0f64;
            for i in pre..n {
                let t = (i - pre) as f64 / (n - pre) as f64;
                let fc = p.tone * (0.12 + 0.88 * (1.0 - ch)).powf(t * 3.0);
                let a = (-TAU * fc.min(sr * 0.45) / sr).exp();
                y = (1.0 - a) * h[i] as f64 + a * y;
                h[i] = y as f32;
            }
            if ty == 4 { // early reflections
                let taps = 6 + (ch * 18.0).floor() as usize;
                for k in 0..taps {
                    let at = pre + (r.next() * 0.035 * sr).floor() as usize;
                    if at < n {
                        let sgn = if r.next() < 0.5 { -1.0 } else { 1.0 };
                        h[at] = (h[at] as f64 + sgn * (0.6 + 0.6 * r.next()) * (1.0 - k as f64 / taps as f64)) as f32;
                    }
                }
            }
        } else if ty == 1 {
            // metal: dense inharmonic decaying partials
            let np = 24 + (ch * 60.0).floor() as usize;
            let (lo, hi) = (250.0, p.tone.min(16000.0));
            for _ in 0..np {
                let f = lo * (hi / lo as f64).powf(r.next().powf(0.8));
                let ph = r.next() * TAU;
                let dd = dk * (0.4 + 1.6 * r.next()) * (f / 2000.0).powf(0.3);
                let amp = 1.0 / (np as f64).sqrt();
                let w = TAU * f / sr;
                for i in pre..n {
                    let t = (i - pre) as f64;
                    h[i] = (h[i] as f64 + amp * (w * t + ph).sin() * (-t * dd).exp()) as f32;
                }
            }
            noise_tail(h, &mut r, 0.15, 3.0);
        } else if ty == 2 {
            // whip: downward exponential chirp (dispersion) + faint tail
            let fhi = (p.tone * 1.2).min(18000.0);
            let flo = 60.0 + 400.0 * (1.0 - ch);
            let tc = (t60 * (0.25 + 0.6 * ch)).max(0.01);
            let nc = (tc * sr).floor() as usize;
            let mut ph = c as f64 * 0.7;
            let mut i = 0;
            while i < nc && pre + i < n {
                let t = i as f64 / sr;
                let f = fhi * (flo / fhi).powf(t / tc);
                ph += TAU * f / sr;
                h[pre + i] = (h[pre + i] as f64 + ph.sin() * (1.0 - i as f64 / nc as f64).powf(1.5) * (f / fhi + 0.05).sqrt()) as f32;
                i += 1;
            }
            noise_tail(h, &mut r, 0.08, 1.0);
        } else if ty == 3 {
            // spring: repeating dispersive chirps
            let sp = ((0.012 + 0.05 * ch) * (1.0 + 0.03 * c as f64)).max(0.004);
            let nc = (0.018 * sr).floor() as usize;
            let mut e = 0usize;
            while (pre as f64) + e as f64 * sp * sr < n as f64 {
                let st = pre + (e as f64 * sp * sr).floor() as usize;
                let g = (-((st - pre) as f64) * dk).exp() * if e % 2 == 1 { -1.0 } else { 1.0 };
                let mut ph = 0f64;
                let mut i = 0;
                while i < nc && st + i < n {
                    let f = 5000.0 * (150.0f64 / 5000.0).powf(i as f64 / nc as f64);
                    ph += TAU * f / sr;
                    h[st + i] = (h[st + i] as f64 + g * ph.sin() * (1.0 - i as f64 / nc as f64)) as f32;
                    i += 1;
                }
                e += 1;
            }
            noise_tail(h, &mut r, 0.1, 1.0);
        }
    }
    // unit energy (per channel average)
    let (a, b) = (rms(&out[0]), rms(&out[1]));
    let mut e = ((a * a + b * b) / 2.0 * n as f64).sqrt();
    if e == 0.0 || e.is_nan() { e = 1.0; }
    for h in out.iter_mut() { for v in h.iter_mut() { *v = (*v as f64 / e) as f32; } }
    let [l, r] = out;
    (l, r)
}
