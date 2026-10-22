// Lane visuals: reassigned (sharpened) spectrogram on a time × log-frequency grid, and min/max waveform columns.
use tearout_dsp::dsp::fft;

pub const FMIN: f64 = 30.0;
pub const FMAX: f64 = 20000.0;

/// Energy grid, row 0 = FMAX, last row = FMIN. Each bin's energy is moved to its instantaneous frequency
/// (ω − Im(X_dh·X_h*)/|X_h|²) and group delay (t + N/2 + Re(X_th·X_h*)/|X_h|²), then 3×3 blurred.
pub fn reassigned(x: &[f32], sr: f64, t1: usize, cols: usize, rows: usize, hop: usize) -> Vec<f32> {
    const NW: usize = 1024;
    let c = NW / 2;
    let mut h = vec![0f64; NW];
    let mut dh = vec![0f64; NW];
    let mut th = vec![0f64; NW];
    for m in 0..NW {
        let a = 2.0 * std::f64::consts::PI * m as f64 / NW as f64;
        h[m] = 0.5 - 0.5 * a.cos();
        dh[m] = (std::f64::consts::PI / NW as f64) * a.sin();
        th[m] = (m as f64 - c as f64) * h[m];
    }
    let lf = (FMAX / FMIN).ln();
    let mut g = vec![0f64; cols * rows];
    let (mut ar, mut ai, mut br, mut bi, mut dr, mut di) = (vec![0f64; NW], vec![0f64; NW], vec![0f64; NW], vec![0f64; NW], vec![0f64; NW], vec![0f64; NW]);
    let mut pts: Vec<(f64, f64, f64)> = Vec::new();
    let mut emax = 0f64;
    let mut t = -(c as i64);
    while t + (c as i64) < t1 as i64 {
        for m in 0..NW {
            let i = t + m as i64;
            let v = if i >= 0 && (i as usize) < x.len() { x[i as usize] as f64 } else { 0.0 };
            ar[m] = v * h[m]; ai[m] = 0.0;
            br[m] = v * dh[m]; bi[m] = 0.0;
            dr[m] = v * th[m]; di[m] = 0.0;
        }
        fft(&mut ar, &mut ai, false);
        fft(&mut br, &mut bi, false);
        fft(&mut dr, &mut di, false);
        for k in 1..NW / 2 {
            let (hr, hi) = (ar[k], ai[k]);
            let p = hr * hr + hi * hi;
            if p > emax { emax = p; }
            if p < 1e-20 { continue; }
            let imr = (bi[k] * hr - br[k] * hi) / p;
            let rer = (dr[k] * hr + di[k] * hi) / p;
            let w = 2.0 * std::f64::consts::PI * k as f64 / NW as f64 - imr;
            let f = w * sr / (2.0 * std::f64::consts::PI);
            let tt = t as f64 + c as f64 + rer;
            pts.push((p, f, tt));
        }
        t += hop as i64;
    }
    for (p, f, tt) in pts {
        if p < emax * 1e-11 || f < FMIN || f > FMAX || tt < 0.0 || tt >= t1 as f64 { continue; }
        let cx = (tt / t1 as f64 * cols as f64) as usize;
        let ry = rows as i64 - 1 - ((f / FMIN).ln() / lf * rows as f64).floor() as i64;
        if ry >= 0 && (ry as usize) < rows && cx < cols { g[ry as usize * cols + cx] += p; }
    }
    // 3×3 blur, weights 1 / 0.5 edge / 0.25 corner
    let mut o = vec![0f32; cols * rows];
    for r in 0..rows {
        for cc in 0..cols {
            let (mut s, mut w) = (0f64, 0f64);
            for d_r in -1i64..=1 {
                for d_c in -1i64..=1 {
                    let (rr, c2) = (r as i64 + d_r, cc as i64 + d_c);
                    if rr < 0 || rr >= rows as i64 || c2 < 0 || c2 >= cols as i64 { continue; }
                    let k = if d_r == 0 && d_c == 0 { 1.0 } else if d_r != 0 && d_c != 0 { 0.25 } else { 0.5 };
                    s += k * g[rr as usize * cols + c2 as usize];
                    w += k;
                }
            }
            o[r * cols + cc] = (s / w) as f32;
        }
    }
    o
}

/// Grid → RGBA, dB relative to the grid's own peak over a 75 dB range, gamma 1.6, tinted by `col`.
pub fn colorize(g: &[f32], col: [u8; 3]) -> Vec<u8> {
    let mx = g.iter().cloned().fold(0f32, f32::max).max(1e-30);
    let mut out = Vec::with_capacity(g.len() * 4);
    for &v in g {
        let db = 10.0 * ((v / mx) as f64 + 1e-20).log10();
        let a = ((db + 75.0) / 75.0).clamp(0.0, 1.0).powf(1.6);
        for c in col { out.push((c as f64 * a).round() as u8); }
        out.push(255);
    }
    out
}

/// Per-column (max, min) of the mid signal over [0, t1), normalized to the lane's own peak.
pub fn wave_cols(l: &[f32], r: &[f32], t1: usize, cols: usize) -> Vec<(f32, f32)> {
    let n = t1.min(l.len());
    let m = |i: usize| 0.5 * (l[i] + r[i]);
    let pk = (0..n).map(|i| m(i).abs()).fold(0f32, f32::max).max(1e-9);
    (0..cols)
        .map(|c| {
            let (a, b) = (c * n / cols, ((c + 1) * n / cols).max(c * n / cols + 1).min(n));
            let (mut hi, mut lo) = (0f32, 0f32);
            for i in a..b { let v = m(i) / pk; hi = hi.max(v); lo = lo.min(v); }
            (hi, lo)
        })
        .collect()
}

/// Log-frequency y (0 = top = FMAX, 1 = bottom = FMIN) for a frequency.
pub fn freq_y(f: f64) -> f32 { (1.0 - (f.clamp(FMIN, FMAX) / FMIN).ln() / (FMAX / FMIN).ln()) as f32 }
pub fn y_freq(y: f32) -> f64 { FMIN * ((FMAX / FMIN).ln() * (1.0 - y as f64)).exp() }

/// Third-octave bands from 50 Hz to 20 kHz (as the web app's tonal balance).
pub const TOB_BANDS: usize = 27;

/// Power per third-octave band, dB, of `x` averaged over 8192-point Hann frames every 2048 samples
/// (`x` shorter than a frame is zero-padded). The web app's `thirdOct`, before it is set relative to its
/// loudest band.
pub fn third_oct(x: &[f32], sr: f64) -> [f64; TOB_BANDS] {
    let n = 8192;
    let w: Vec<f64> = (0..n).map(|i| 0.5 - 0.5 * (2.0 * std::f64::consts::PI * i as f64 / n as f64).cos()).collect();
    let (mut re, mut im, mut acc) = (vec![0.0; n], vec![0.0; n], vec![0.0; n / 2]);
    let mut frames = 0.0;
    let mut o = 0;
    while o + n <= n.max(x.len()) {
        for i in 0..n { re[i] = x.get(o + i).copied().unwrap_or(0.0) as f64 * w[i]; im[i] = 0.0; }
        fft(&mut re, &mut im, false);
        for k in 0..n / 2 { acc[k] += re[k] * re[k] + im[k] * im[k]; }
        frames += 1.0;
        o += 2048;
    }
    std::array::from_fn(|i| {
        let c = 1000.0 * 2f64.powf((i as f64 - 14.0) / 3.0);
        let lo = (c / 2f64.powf(1.0 / 6.0) * n as f64 / sr).floor() as usize;
        let hi = (c * 2f64.powf(1.0 / 6.0) * n as f64 / sr).ceil() as usize;
        let s: f64 = (lo.max(1)..=hi.min(n / 2 - 1)).map(|k| acc[k]).sum();
        10.0 * (s / frames + 1e-14).log10()
    })
}

/// The gun family's reference balance, dB re its loudest band.
pub fn gun_target() -> &'static [f64] {
    static T: std::sync::OnceLock<Vec<f64>> = std::sync::OnceLock::new();
    T.get_or_init(|| {
        let j: serde_json::Value = serde_json::from_str(include_str!("../../../web/targets.json")).unwrap_or_default();
        j["gun"].as_array().into_iter().flatten().filter_map(|v| v.as_f64()).collect()
    })
}
