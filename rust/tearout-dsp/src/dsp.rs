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

// RBJ biquad coefficients, normalized by a0
#[derive(Clone, Copy)]
pub struct Coef { b0: f64, b1: f64, b2: f64, a1: f64, a2: f64 }

// RBJ biquad, in place
pub fn biquad(x: &mut [f32], ty: Bq, f: f64, q: f64, gain_db: f64, sr: f64) {
    cascade(&mut [x], &[coef(ty, f, q, gain_db, sr)]);
}
pub fn lr4(x: &mut [f32], ty: Bq, f: f64, sr: f64) {
    let c = coef(ty, f, FRAC_1_SQRT_2, 0.0, sr);
    cascade(&mut [x], &[c, c]);
}
pub fn lr4c(ty: Bq, f: f64, sr: f64) -> [Coef; 2] { let c = coef(ty, f, FRAC_1_SQRT_2, 0.0, sr); [c, c] }

/// Runs every buffer (one or two) through the same chain of biquad stages. Each stage's output is rounded to f32
/// before the next, so the result equals running the stages one after another over whole buffers. Stages go
/// two at a time and channels side by side, so independent recursions overlap in the CPU.
pub fn cascade(bufs: &mut [&mut [f32]], st: &[Coef]) {
    match bufs {
        [x] => { for c in st.chunks(4) { match c.len() { 4 => bq1x4(x, c), 3 => { bq1x2(x, &c[0], &c[1]); bq1(x, &c[2]) } 2 => bq1x2(x, &c[0], &c[1]), _ => bq1(x, &c[0]) } } }
        [l, r] if st.len() >= 8 => par2(l, r, |x| cascade(&mut [x], st)),
        [l, r] => { for c in st.chunks(2) { if c.len() == 2 { bq2x2(l, r, &c[0], &c[1]) } else { bq2(l, r, &c[0]) } } }
        _ => { for x in bufs.iter_mut() { cascade(&mut [&mut **x], st); } }
    }
}
#[inline(always)]
fn tick(c: &Coef, xi: f64, z: &mut (f64, f64, f64, f64)) -> f32 {
    let y = c.b0 * xi + c.b1 * z.0 + c.b2 * z.1 - c.a1 * z.2 - c.a2 * z.3;
    z.1 = z.0; z.0 = xi; z.3 = z.2; z.2 = y;
    y as f32
}
fn bq1(x: &mut [f32], c: &Coef) {
    let mut z = (0.0, 0.0, 0.0, 0.0);
    for v in x.iter_mut() { *v = tick(c, *v as f64, &mut z); }
}
fn bq1x2(x: &mut [f32], c1: &Coef, c2: &Coef) {
    let (mut z1, mut z2) = ((0.0, 0.0, 0.0, 0.0), (0.0, 0.0, 0.0, 0.0));
    for v in x.iter_mut() { let a = tick(c1, *v as f64, &mut z1); *v = tick(c2, a as f64, &mut z2); }
}
fn bq1x4(x: &mut [f32], c: &[Coef]) {
    let z0 = (0.0, 0.0, 0.0, 0.0);
    let mut z = [z0; 4];
    for v in x.iter_mut() {
        let a = tick(&c[0], *v as f64, &mut z[0]);
        let b = tick(&c[1], a as f64, &mut z[1]);
        let d = tick(&c[2], b as f64, &mut z[2]);
        *v = tick(&c[3], d as f64, &mut z[3]);
    }
}
fn bq2(l: &mut [f32], r: &mut [f32], c: &Coef) {
    let (mut zl, mut zr) = ((0.0, 0.0, 0.0, 0.0), (0.0, 0.0, 0.0, 0.0));
    for (a, b) in l.iter_mut().zip(r.iter_mut()) { *a = tick(c, *a as f64, &mut zl); *b = tick(c, *b as f64, &mut zr); }
}
fn bq2x2(l: &mut [f32], r: &mut [f32], c1: &Coef, c2: &Coef) {
    let z0 = (0.0, 0.0, 0.0, 0.0);
    let (mut l1, mut l2, mut r1, mut r2) = (z0, z0, z0, z0);
    for (a, b) in l.iter_mut().zip(r.iter_mut()) {
        let la = tick(c1, *a as f64, &mut l1);
        let ra = tick(c1, *b as f64, &mut r1);
        *a = tick(c2, la as f64, &mut l2);
        *b = tick(c2, ra as f64, &mut r2);
    }
}

pub fn coef(ty: Bq, f: f64, q: f64, gain_db: f64, sr: f64) -> Coef {
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
    Coef { b0, b1, b2, a1, a2 }
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
    // logcosh/hc_f of the previous sample, carried over when the last step computed them
    let mut prev: Option<(f64, f64)> = None;
    for s in x.iter_mut() {
        let mut v = *s as f64 * g;
        if fold > 0.0 {
            let k = 1.0 + 3.0 * fold;
            v = (1.0 - fold) * v + fold * (0.5 * PI * v * k).sin() * 1.2;
        }
        let d = v - xp;
        let (yt, yh);
        if d.abs() > 1e-5 {
            let (lp, hp) = prev.unwrap_or_else(|| (logcosh(xp), hc_f(xp)));
            let (lv, hv) = (logcosh(v), hc_f(v));
            yt = (lv - lp) / d; yh = (hv - hp) / d;
            prev = Some((lv, hv));
        } else {
            let m = 0.5 * (v + xp); yt = m.tanh(); yh = hc(m);
            prev = None;
        }
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

// Twiddles for every stage of an n-point FFT, concatenated (stage len: len/2 entries at offset len/2 - 1). Generated
// by the rotation recurrence cr' = cr·wr − ci·wi, ci' = cr·wi + ci·wr from (1, 0), so each value is exactly the one
// the recurrence form of the butterfly loop produces.
fn twiddles(n: usize, inv: bool) -> std::sync::Arc<Vec<(f64, f64)>> {
    use std::sync::{Arc, Mutex, OnceLock};
    static CACHE: OnceLock<Mutex<std::collections::HashMap<(usize, bool), Arc<Vec<(f64, f64)>>>>> = OnceLock::new();
    let mut c = CACHE.get_or_init(Default::default).lock().unwrap();
    c.entry((n, inv)).or_insert_with(|| {
        let mut t = Vec::with_capacity(n);
        let mut len = 2;
        while len <= n {
            let ang = (if inv { 2.0 } else { -2.0 }) * PI / len as f64;
            let (wr, wi) = (ang.cos(), ang.sin());
            let (mut cr, mut ci) = (1f64, 0f64);
            for _ in 0..len / 2 {
                t.push((cr, ci));
                let tt = cr * wr - ci * wi; ci = cr * wi + ci * wr; cr = tt;
            }
            len <<= 1;
        }
        Arc::new(t)
    }).clone()
}

// butterflies of one stage (block size len) for j in [j0, j1) of every block
#[inline(always)]
fn stage(re: *mut f64, im: *mut f64, n: usize, len: usize, w: &[(f64, f64)], j0: usize, j1: usize) {
    let half = len / 2;
    let mut i = 0;
    while i < n {
        for j in j0..j1 {
            let (cr, ci) = w[j];
            // SAFETY: a = i + j and b = a + half are < n; callers give disjoint (block, j) ranges to each thread
            unsafe {
                let (a, b) = (i + j, i + j + half);
                let (rb, ib) = (*re.add(b), *im.add(b));
                let tr = rb * cr - ib * ci;
                let ti = rb * ci + ib * cr;
                let (ra, ia) = (*re.add(a), *im.add(a));
                *re.add(b) = ra - tr; *im.add(b) = ia - ti; *re.add(a) = ra + tr; *im.add(a) = ia + ti;
            }
        }
        i += len;
    }
}
#[derive(Clone, Copy)]
struct Ptr(*mut f64);
unsafe impl Send for Ptr {}
unsafe impl Sync for Ptr {}

// radix-2 complex FFT, in place; inverse scales by 1/n. Large sizes run on 4 threads: the first stages as four
// independent quarter-size transforms, the last two with each stage's butterflies split four ways. Every butterfly
// computes the same values as in a single-threaded pass.
pub fn fft(re: &mut [f64], im: &mut [f64], inv: bool) {
    let n = re.len();
    let mut j = 0usize;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 { j ^= bit; bit >>= 1; }
        j ^= bit;
        if i < j { re.swap(i, j); im.swap(i, j); }
    }
    let tw = twiddles(n, inv);
    let w = |len: usize| &tw[len / 2 - 1..len - 1];
    const T: usize = 4;
    let (pr, pi) = (Ptr(re.as_mut_ptr()), Ptr(im.as_mut_ptr()));
    if n >= 1 << 15 {
        let m = n / T;
        std::thread::scope(|sc| {
            for t in 1..T {
                let w = &w;
                sc.spawn(move || { let (pr, pi) = (pr, pi); let mut len = 2; while len <= m { stage(unsafe { pr.0.add(t * m) }, unsafe { pi.0.add(t * m) }, m, len, w(len), 0, len / 2); len <<= 1; } });
            }
            let mut len = 2; while len <= m { stage(pr.0, pi.0, m, len, w(len), 0, len / 2); len <<= 1; }
        });
        let mut len = 2 * m;
        while len <= n {
            let (half, wl) = (len / 2, w(len));
            std::thread::scope(|sc| {
                for t in 1..T { sc.spawn(move || { let (pr, pi) = (pr, pi); stage(pr.0, pi.0, n, len, wl, t * half / T, (t + 1) * half / T) }); }
                stage(pr.0, pi.0, n, len, wl, 0, half / T);
            });
            len <<= 1;
        }
    } else {
        let mut len = 2; while len <= n { stage(pr.0, pi.0, n, len, w(len), 0, len / 2); len <<= 1; }
    }
    if inv { let nf = n as f64; for i in 0..n { re[i] /= nf; im[i] /= nf; } }
}

// FFT size for convolving x_len samples with an h_len IR
pub fn conv_size(x_len: usize, h_len: usize) -> usize { let mut n = 1; while n < x_len + h_len { n <<= 1; } n }
// spectrum of a stereo IR packed as one complex signal (hl real, hr imaginary), at FFT size n
pub fn ir_spectrum(hl: &[f32], hr: &[f32], n: usize) -> (Vec<f64>, Vec<f64>) {
    let (mut h_r, mut h_i) = (vec![0f64; n], vec![0f64; n]);
    for (d, &s) in h_r.iter_mut().zip(hl) { *d = s as f64; }
    for (d, &s) in h_i.iter_mut().zip(hr) { *d = s as f64; }
    fft(&mut h_r, &mut h_i, false);
    (h_r, h_i)
}
// mono x convolved with a stereo IR given by its packed spectrum: (left, right)
pub fn convolve_spec(x: &[f32], h: &(Vec<f64>, Vec<f64>), out_len: usize) -> (Buf, Buf) {
    let n = h.0.len();
    let (mut xr, mut xi) = (vec![0f64; n], vec![0f64; n]);
    for (d, &s) in xr.iter_mut().zip(x) { *d = s as f64; }
    fft(&mut xr, &mut xi, false);
    for k in 0..n {
        let (a, b, c, d) = (xr[k], xi[k], h.0[k], h.1[k]);
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
    let n = ((p.size * 1.3 + p.pre + 5.0) * sr / 1000.0).ceil() as usize;
    let pre = (p.pre * sr / 1000.0).floor() as usize;
    let mut out = [vec![0f32; n], vec![0f32; n]];
    let (o0, o1) = out.split_at_mut(1);
    std::thread::scope(|sc| {
        let h1 = &mut o1[0];
        sc.spawn(move || ir_channel(p, sr, 1, n, pre, h1));
        ir_channel(p, sr, 0, n, pre, &mut o0[0]);
    });
    // unit energy (per channel average)
    let (a, b) = (rms(&out[0]), rms(&out[1]));
    let mut e = ((a * a + b * b) / 2.0 * n as f64).sqrt();
    if e == 0.0 || e.is_nan() { e = 1.0; }
    for h in out.iter_mut() { for v in h.iter_mut() { *v = (*v as f64 / e) as f32; } }
    let [l, r] = out;
    (l, r)
}
fn ir_channel(p: &IrSpec, sr: f64, c: usize, n: usize, pre: usize, h: &mut Buf) {
    let ty = js_round(p.ty) as i64;
    let (size, ch) = (p.size, p.ch);
    {
        let mut r = Rng::new(to_i32(p.seed) as f64 * 7919.0 + 17.0 + c as f64 * 104729.0);
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
}

/// Applies f to both channels on two threads.
pub fn par2(l: &mut [f32], r: &mut [f32], f: impl Fn(&mut [f32]) + Sync) {
    std::thread::scope(|sc| { let h = sc.spawn(|| f(r)); f(l); h.join().unwrap(); });
}
