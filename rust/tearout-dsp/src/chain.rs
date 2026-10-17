// Crunch chain on the tops, reverb send, mids chain and final stage on the bus.
use crate::dsp::*;
use crate::id::*;
use crate::Params;

fn disperse(l: &mut [f32], r: &mut [f32], n: f64, f: f64, q: f64, sr: f64) {
    let c = coef(Bq::Ap, f, q, 0.0, sr);
    let mut st = vec![];
    while (st.len() as f64) < n { st.push(c); }
    cascade(&mut [l, r], &st);
}
// single-sideband frequency shift via FFT analytic signal; rot[i] = (cos, sin) of the shift phase at sample i
fn freq_shift(x: &[f32], rot: &[(f64, f64)]) -> Buf {
    let mut n = 1;
    while n < x.len() { n <<= 1; }
    let (mut re, mut im) = (vec![0f64; n], vec![0f64; n]);
    for (d, &s) in re.iter_mut().zip(x) { *d = s as f64; }
    fft(&mut re, &mut im, false);
    for k in 1..n / 2 { re[k] *= 2.0; im[k] *= 2.0; }
    for k in n / 2 + 1..n { re[k] = 0.0; im[k] = 0.0; }
    fft(&mut re, &mut im, true);
    (0..x.len()).map(|i| (re[i] * rot[i].0 - im[i] * rot[i].1) as f32).collect()
}
fn peak_norm(l: &mut [f32], r: &mut [f32], target: f64) {
    let pk = peak2(l, r);
    if pk > 1e-9 { let g = target / pk; scale(l, g); scale(r, g); }
}

pub fn env_follow(x: &[f32], sr: f64, att_ms: f64, rel_ms: f64) -> Buf {
    let a = (-1.0 / (att_ms * sr / 1000.0)).exp();
    let r = (-1.0 / (rel_ms * sr / 1000.0)).exp();
    let mut v = 0f64;
    x.iter().map(|&s| {
        let s = (s as f64).abs();
        v = if s > v { a * v + (1.0 - a) * s } else { r * v + (1.0 - r) * s };
        v as f32
    }).collect()
}

// 3-band upward/downward compressor (120 Hz / 2.5 kHz crossovers)
pub fn ott_core(l: &mut [f32], r: &mut [f32], depth: f64, up: f64, tm: f64, sr: f64) {
    if depth < 0.001 { return; }
    let n = l.len();
    // both channels at once; low band (allpass-compensated for the 2.5k crossover) and the rest on separate threads
    let (lp1, hp1, lp2, hp2) = (lr4c(Bq::Lp, 120.0, sr), lr4c(Bq::Hp, 120.0, sr), lr4c(Bq::Lp, 2500.0, sr), lr4c(Bq::Hp, 2500.0, sr));
    let ((la_l, la_r), ((mid_l, mid_r), (hi_l, hi_r))) = std::thread::scope(|sc| {
        let rest = sc.spawn(|| {
            let (mut hi_l, mut hi_r) = (l.to_vec(), r.to_vec()); cascade(&mut [&mut hi_l, &mut hi_r], &hp1);
            let (mut mid_l, mut mid_r) = (hi_l.clone(), hi_r.clone()); cascade(&mut [&mut mid_l, &mut mid_r], &lp2);
            cascade(&mut [&mut hi_l, &mut hi_r], &hp2);
            ((mid_l, mid_r), (hi_l, hi_r))
        });
        let (mut lo_l, mut lo_r) = (l.to_vec(), r.to_vec()); cascade(&mut [&mut lo_l, &mut lo_r], &lp1);
        let (mut la_l, mut la_r) = (lo_l.clone(), lo_r.clone()); cascade(&mut [&mut la_l, &mut la_r], &lp2);
        cascade(&mut [&mut lo_l, &mut lo_r], &hp2);
        for i in 0..n { la_l[i] = (la_l[i] as f64 + lo_l[i] as f64) as f32; la_r[i] = (la_r[i] as f64 + lo_r[i] as f64) as f32; }
        ((la_l, la_r), rest.join().unwrap())
    });
    let (bl, br) = ([la_l, mid_l, hi_l], [la_r, mid_r, hi_r]);
    let cfg = [(12.0 * tm, 120.0 * tm), (4.0 * tm, 70.0 * tm), (2.0 * tm, 40.0 * tm)];
    // per band: gain per sample, from its own envelope (bands on separate threads)
    let gains: Vec<Vec<f64>> = std::thread::scope(|sc| {
        let hs: Vec<_> = (0..3).map(|b| { let (bl, br) = (&bl[b], &br[b]); let (at, rl) = cfg[b]; sc.spawn(move || ott_gains(bl, br, depth, up, at, rl, sr)) }).collect();
        hs.into_iter().map(|h| h.join().unwrap()).collect()
    });
    // bands summed in order, as accumulating into zeroed buffers
    for i in 0..n {
        let (mut a, mut b) = (0f32, 0f32);
        for k in 0..3 {
            a = (a as f64 + bl[k][i] as f64 * gains[k][i]) as f32;
            b = (b as f64 + br[k][i] as f64 * gains[k][i]) as f32;
        }
        l[i] = a; r[i] = b;
    }
}

// one OTT band's gain per sample. Envelope levels inside (-30, -18) dB or below -75 dB give exactly 0 dB of gain:
// those are recognized from the linear envelope (with a 1e-12 margin, so the dB comparison can't disagree) and skip log/pow.
fn ott_gains(bl: &[f32], br: &[f32], depth: f64, up: f64, at: f64, rl: f64, sr: f64) -> Vec<f64> {
    let sum: Buf = (0..bl.len()).map(|i| (bl[i] as f64).abs().max((br[i] as f64).abs()) as f32).collect();
    let e = env_follow(&sum, sr, at, rl);
    let (t75, t30, t18) = (10f64.powf(-75.0 / 20.0) * (1.0 - 1e-12), 10f64.powf(-30.0 / 20.0) * (1.0 + 1e-12), 10f64.powf(-18.0 / 20.0) * (1.0 - 1e-12));
    e.iter().map(|&ev| {
        let x = ev as f64 + 1e-9;
        if x < t75 || (x > t30 && x < t18) { return 1.0; }
        let lv = 20.0 * x.log10();
        let mut gdb = 0f64;
        if lv > -18.0 { gdb -= (lv + 18.0) * (1.0 - 1.0 / 8.0); }
        if lv < -30.0 && lv > -75.0 { gdb += ((-30.0 - lv) * (1.0 - 1.0 / 3.0)).min(24.0) * up; }
        10f64.powf((depth * gdb) / 20.0)
    }).collect()
}

// `conv(ir, x, out_len)`: x convolved with the stereo IR the spec describes
pub fn crunch(l: &mut [f32], r: &mut [f32], p: &Params, sr: f64, conv: &mut dyn FnMut(&IrSpec, &[f32], usize) -> (Buf, Buf)) {
    if p[CR_MIX] < 0.001 { return; }
    let t0 = std::time::Instant::now();
    let (dl, dr) = (l.to_vec(), r.to_vec());
    let rms_in = rms2(l, r);
    let mut eq = lr4c(Bq::Hp, p[CR_HP], sr).to_vec();
    if p[CR_MUD] < -0.05 { eq.push(coef(Bq::Peak, p[CR_HP] * 1.2, 1.0, p[CR_MUD], sr)); }
    if p[CR_PK_G] > 0.05 { eq.push(coef(Bq::Peak, p[CR_PK_F], 1.4, p[CR_PK_G], sr)); }
    cascade(&mut [&mut *l, &mut *r], &eq);
    peak_norm(l, r, 0.7);
    par2(l, r, |x| shape(x, p[CR_DRIVE1], p[CR_HARD1], 0.0));
    crate::timing::add(5, t0); let t0 = std::time::Instant::now();
    if p[CR_DISP_N] > 0.0 {
        disperse(l, r, p[CR_DISP_N], p[CR_DISP_F], p[CR_DISP_Q], sr);
    }
    crate::timing::add(6, t0); let t0 = std::time::Instant::now();
    if p[CR_CONV] > 0.001 {
        let mono: Buf = (0..l.len()).map(|i| (0.5 * (l[i] as f64 + r[i] as f64)) as f32).collect();
        let (cl, cr) = conv(&IrSpec { ty: p[CR_CONV_TYPE], size: p[CR_CONV_LEN], ch: p[CR_CONV_CHAR], tone: 16000.0, pre: 0.0, seed: 3.0 }, &mono, l.len());
        let mut den = rms2(&cl, &cr);
        if den == 0.0 { den = 1.0; }
        let gc = rms2(l, r) / den;
        let w = p[CR_CONV];
        for i in 0..l.len() {
            l[i] = ((1.0 - w) * l[i] as f64 + w * gc * cl[i] as f64) as f32;
            r[i] = ((1.0 - w) * r[i] as f64 + w * gc * cr[i] as f64) as f32;
        }
    }
    crate::timing::add(7, t0); let t0 = std::time::Instant::now();
    peak_norm(l, r, 0.7);
    par2(l, r, |x| shape(x, p[CR_DRIVE2], 0.5, 0.0));
    let mut k = 0.0;
    while k < p[CR_RACK] {
        ott_core(l, r, p[CR_RACK_DEPTH], 0.8, 2.5, sr);
        peak_norm(l, r, 1.4);
        par2(l, r, |x| for v in x.iter_mut() { *v = (*v as f64).tanh() as f32; });
        k += 1.0;
    }
    crate::timing::add(8, t0); let t0 = std::time::Instant::now();
    if p[CR_PP] > 0.001 {
        const NB: usize = 1 << 13;
        let dly = (p[CR_PP_MS] * sr / 1000.0).floor() as usize;
        let dry = dly + (0.003 * sr).floor() as usize;
        let (mut bl, mut br) = (vec![0f32; NB], vec![0f32; NB]);
        let fb = p[CR_PP_FB];
        for i in 0..l.len() {
            let yl = bl[(i + NB * 4 - dly) & (NB - 1)] as f64;
            let yr = br[(i + NB * 4 - dry) & (NB - 1)] as f64;
            bl[i & (NB - 1)] = (l[i] as f64 + fb * yr.tanh()) as f32;
            br[i & (NB - 1)] = (r[i] as f64 + fb * yl.tanh()) as f32;
            l[i] = (l[i] as f64 + p[CR_PP] * yl) as f32;
            r[i] = (r[i] as f64 + p[CR_PP] * yr) as f32;
        }
    }
    crate::timing::add(9, t0); let t0 = std::time::Instant::now();
    if p[CR_FS_WET] > 0.001 && p[CR_FS].abs() > 0.05 {
        let w = TAU * p[CR_FS] / sr;
        let rot: Vec<(f64, f64)> = (0..l.len()).map(|i| { let a = w * i as f64; (a.cos(), a.sin()) }).collect();
        let (sl, sr2) = std::thread::scope(|sc| { let h = sc.spawn(|| freq_shift(r, &rot)); (freq_shift(l, &rot), h.join().unwrap()) });
        let w = p[CR_FS_WET];
        for i in 0..l.len() {
            l[i] = ((1.0 - w) * l[i] as f64 + w * sl[i] as f64) as f32;
            r[i] = ((1.0 - w) * r[i] as f64 + w * sr2[i] as f64) as f32;
        }
    }
    cascade(&mut [&mut *l, &mut *r], &lr4c(Bq::Hp, p[CR_HP] * 0.8, sr));
    // keep the tops' loudness relationship with the sub: output RMS = input RMS
    let rms_out = rms2(l, r);
    let g = if rms_out > 1e-9 { rms_in / rms_out } else { 0.0 };
    let m = p[CR_MIX];
    crate::timing::add(10, t0);
    for i in 0..l.len() {
        l[i] = ((1.0 - m) * dl[i] as f64 + m * g * l[i] as f64) as f32;
        r[i] = ((1.0 - m) * dr[i] as f64 + m * g * r[i] as f64) as f32;
    }
}

// transient shaper: boost where a fast envelope runs ahead of a slow one
fn snap(l: &mut [f32], r: &mut [f32], amt_db: f64, sr: f64) {
    if amt_db < 0.05 { return; }
    let m: Buf = (0..l.len()).map(|i| (0.5 * ((l[i] as f64).abs() + (r[i] as f64).abs())) as f32).collect();
    let (fast, slow) = (env_follow(&m, sr, 0.3, 25.0), env_follow(&m, sr, 12.0, 90.0));
    for i in 0..l.len() {
        let d = 20.0 * ((fast[i] as f64 + 1e-9) / (slow[i] as f64 + 1e-9)).log10();
        let g = 10f64.powf(((d / 12.0).min(1.0).max(0.0) * amt_db) / 20.0);
        l[i] = (l[i] as f64 * g) as f32; r[i] = (r[i] as f64 * g) as f32;
    }
}

// Mids chain: OTT -> snap -> drive/clip. Output loudness matches input, so layer levels keep their meaning.
fn mids_chain(l: &mut [f32], r: &mut [f32], p: &Params, sr: f64) {
    let r0 = rms2(l, r);
    if r0 < 1e-9 { return; }
    ott_core(l, r, p[MST_OTT], p[MST_OTT_UP], p[MST_OTT_TIME], sr);
    snap(l, r, p[MST_SNAP], sr);
    let pk = peak2(l, r);
    let pre = if pk > 0.0 { 0.7 / pk } else { 1.0 };
    scale(l, pre); scale(r, pre);
    par2(l, r, |x| shape(x, p[MST_DRIVE], p[MST_HARD], 0.0));
    let mut d = rms2(l, r);
    if d == 0.0 { d = 1.0; }
    let g = r0 / d;
    scale(l, g); scale(r, g);
}

// Final: tone shelves, mono lows, light soft ceiling, peak at -0.3 dBFS.
fn final_stage(l: &mut [f32], r: &mut [f32], p: &Params, sr: f64) {
    let mut tone = vec![];
    if p[MST_LOW].abs() > 0.05 { tone.push(coef(Bq::Ls, 90.0, 0.7, p[MST_LOW], sr)); }
    if p[MST_MID].abs() > 0.05 { tone.push(coef(Bq::Peak, 280.0, 0.9, p[MST_MID], sr)); }
    if p[MST_HIGH].abs() > 0.05 { tone.push(coef(Bq::Hs, 3000.0, 0.7, p[MST_HIGH], sr)); }
    cascade(&mut [&mut *l, &mut *r], &tone);
    let (mut low_l, mut low_r) = (l.to_vec(), r.to_vec());
    cascade(&mut [&mut low_l, &mut low_r], &lr4c(Bq::Lp, p[MST_MONO], sr));
    cascade(&mut [&mut *l, &mut *r], &lr4c(Bq::Hp, p[MST_MONO], sr));
    for i in 0..l.len() {
        let m = 0.5 * (low_l[i] as f64 + low_r[i] as f64);
        l[i] = (l[i] as f64 + m) as f32; r[i] = (r[i] as f64 + m) as f32;
    }
    let pk = peak2(l, r);
    let pre = if pk > 0.0 { 1.0 / pk } else { 1.0 };
    scale(l, pre); scale(r, pre);
    par2(l, r, |x| shape(x, 3.0, 0.2, 0.0));
    let pk = peak2(l, r);
    let g = if pk > 0.0 { 0.966 / pk } else { 1.0 };
    scale(l, g); scale(r, g);
}

// Bus: sub + tops + reverb -> mids chain (with sub_in of the sub) -> post envelope -> rest of the sub -> final.
pub fn bus(sub: &(Buf, Buf), tops: Option<&(Buf, Buf)>, rev: Option<&(Buf, Buf)>, p: &Params, sr: f64) -> (Buf, Buf) {
    let n = sub.0.len();
    let (mut l, mut r) = (vec![0f32; n], vec![0f32; n]);
    let si = p[MST_SUB_IN];
    for x in [tops, rev].into_iter().flatten() { add_scaled(&mut l, &x.0, 1.0); add_scaled(&mut r, &x.1, 1.0); }
    if si > 0.0 { add_scaled(&mut l, &sub.0, si); add_scaled(&mut r, &sub.1, si); }
    mids_chain(&mut l, &mut r, p, sr);
    // post-crush envelope on the mids: shape survives the distortion upstream (^ for chugs)
    if p[MST_SWELL] > 0.5 || p[MST_FALL] > 0.5 {
        let mut on = 0;
        while on < n && (l[on] as f64).abs() + (r[on] as f64).abs() < 1e-4 { on += 1; }
        let s = (p[MST_SWELL] * sr / 1000.0).max(1.0);
        let f = p[MST_FALL] * sr / 1000.0;
        for i in on..n {
            let t = (i - on) as f64;
            let up = if t < s { 0.5 - 0.5 * (PI * t / s).cos() } else { 1.0 };
            let dn = if f > 0.5 && t > s { (-(t - s) / f).exp() } else { 1.0 };
            l[i] = (l[i] as f64 * (up * dn)) as f32; r[i] = (r[i] as f64 * (up * dn)) as f32;
        }
    }
    if si < 1.0 { add_scaled(&mut l, &sub.0, 1.0 - si); add_scaled(&mut r, &sub.1, 1.0 - si); }
    final_stage(&mut l, &mut r, p, sr);
    (l, r)
}
