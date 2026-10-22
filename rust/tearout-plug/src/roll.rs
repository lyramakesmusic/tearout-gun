// Randomize and mutate, as in the web app's "fitted" mode: candidates drawn around the fitted guns (or
// around the current patch), balanced against the gun high-band target, and the most gun-like kept.
use std::sync::OnceLock;
use tearout_dsp::dsp::{fft, js_round, lr4, Bq, Rng};
use tearout_dsp::id::*;
use tearout_dsp::{from_norm, to_norm, Engine, Params, Scale, N, SPEC};

const SR: f64 = 48000.0;

struct Priors {
    mean: Vec<Option<f64>>,
    sd: Vec<Option<f64>>,
    guns: Vec<Vec<Option<f64>>>,
}
struct Ref { median: [f64; 8], mad: [f64; 8] }
const KEYS: [&str; 8] = ["noisy", "peaky", "move", "crest", "harsh", "subMid", "midSustain", "midLate"];

fn by_id(o: &serde_json::Value) -> Vec<Option<f64>> { SPEC.iter().map(|s| o.get(s.id).and_then(|v| v.as_f64())).collect() }

fn priors() -> &'static Priors {
    static P: OnceLock<Priors> = OnceLock::new();
    P.get_or_init(|| {
        let j: serde_json::Value = serde_json::from_str(include_str!("../../../web/priors.json")).unwrap_or_default();
        Priors { mean: by_id(&j["mean"]), sd: by_id(&j["sd"]), guns: j["guns"].as_array().into_iter().flatten().map(by_id).collect() }
    })
}

fn gun_ref() -> &'static Ref {
    static R: OnceLock<Ref> = OnceLock::new();
    R.get_or_init(|| {
        let j: serde_json::Value = serde_json::from_str(include_str!("../../../web/gunness.json")).unwrap_or_default();
        let f = |o: &serde_json::Value| KEYS.map(|k| o[k].as_f64().unwrap_or(0.0));
        Ref { median: f(&j["median"]), mad: f(&j["mad"]) }
    })
}

fn gauss(r: &mut Rng) -> f64 {
    let mut u = 0.0;
    while u == 0.0 { u = r.next(); }
    (-2.0 * u.ln()).sqrt() * (2.0 * std::f64::consts::PI * r.next()).cos()
}

fn is_enum(i: usize) -> bool { SPEC[i].scale == Scale::Int && SPEC[i].unit.contains('/') }

/// A fresh gun: a fitted gun jittered, spice usually off, tops balanced.
pub fn randomize(p: &Params, r: &mut Rng) -> Params {
    let pr = priors();
    let mut q = p.clone();
    let anchor = if pr.guns.is_empty() { None } else { pr.guns.get((r.next() * pr.guns.len() as f64) as usize) };
    for i in 0..N {
        let s = &SPEC[i];
        if s.group.starts_with("sample") || i == LEN || (s.group == "burst" && s.id != "b_jitter") { continue; }
        if i == SEED { q.0[i] = 1.0 + (r.next() * 9999.0).floor(); continue; }
        let fitted = anchor.and_then(|a| a[i]);
        let base = match fitted { Some(v) => to_norm(s, v), None => pr.mean[i].unwrap_or_else(|| to_norm(s, s.def)) };
        let sd = if fitted.is_some() { 0.08 } else { pr.sd[i].unwrap_or(0.22) };
        let mut u = base + gauss(r) * sd;
        if is_enum(i) && (fitted.is_none() || r.next() < 0.3) { u = r.next(); }
        q.0[i] = from_norm(s, u);
    }
    q.0[SPICE_LVL] = if r.next() < 0.6 { -60.0 } else { -28.0 + r.next() * 14.0 };
    q
}

fn is_tail(id: &str) -> bool {
    ["_hold", "_dec", "_rel", "_len", "_fb"].iter().any(|t| id.ends_with(t)) || id.starts_with("rev_") || id.starts_with("cr_pp")
}

/// A nearby variation: continuous knobs nudged, tails gently; anything switched off stays off.
pub fn mutate(p: &Params, r: &mut Rng, amt: f64) -> Params {
    let mut q = p.clone();
    for i in 0..N {
        let s = &SPEC[i];
        if s.group.starts_with("sample") || s.group == "burst" || s.group == "global" || s.scale == Scale::Int { continue; }
        let u = to_norm(s, q.0[i]);
        if u <= 0.002 || (s.id.ends_with("_lvl") && q.0[i] <= -59.0) { continue; }
        q.0[i] = from_norm(s, u + gauss(r) * amt * if is_tail(s.id) { 0.4 } else { 1.0 });
    }
    q
}

fn mono_shot(e: &mut Engine, p: &Params) -> (Vec<f32>, f64) {
    let mut q = p.clone();
    q.0[B_SHOTS] = 1.0;
    let o = e.render(&q);
    let m: Vec<f32> = o.l.iter().zip(&o.r).map(|(a, b)| 0.5 * (a + b)).collect();
    let n = m.len();
    let cut = (0.5 * SR) as usize;
    let (mut a, mut b) = (0.0, 0.0);
    for (i, v) in m.iter().enumerate() { let e = (*v as f64).powi(2); a += e; if i >= cut { b += e; } }
    let _ = n;
    (m, 10.0 * (b / if a == 0.0 { 1.0 } else { a } + 1e-9).log10())
}

/// 1–8 kHz average of the third-octave spectrum, dB re the loudest band.
fn high_band(x: &[f32]) -> f64 {
    let n = 8192;
    let w: Vec<f64> = (0..n).map(|i| 0.5 - 0.5 * (2.0 * std::f64::consts::PI * i as f64 / n as f64).cos()).collect();
    let mut acc = vec![0.0; n / 2];
    let (mut re, mut im) = (vec![0.0; n], vec![0.0; n]);
    let mut fr = 0.0;
    let mut o = 0;
    while o + n <= n.max(x.len()) {
        for i in 0..n { re[i] = x.get(o + i).copied().unwrap_or(0.0) as f64 * w[i]; im[i] = 0.0; }
        fft(&mut re, &mut im, false);
        for k in 0..n / 2 { acc[k] += re[k] * re[k] + im[k] * im[k]; }
        fr += 1.0;
        o += 2048;
    }
    let centers: Vec<f64> = (0..27).map(|i| 1000.0 * 2f64.powf((i as f64 - 14.0) / 3.0)).collect();
    let db: Vec<f64> = centers
        .iter()
        .map(|&c| {
            let lo = (c / 2f64.powf(1.0 / 6.0) * n as f64 / SR).floor() as usize;
            let hi = (c * 2f64.powf(1.0 / 6.0) * n as f64 / SR).ceil() as usize;
            let s: f64 = (lo.max(1)..=hi.min(n / 2 - 1)).map(|k| acc[k]).sum();
            10.0 * (s / fr + 1e-14).log10()
        })
        .collect();
    let mx = db.iter().cloned().fold(f64::MIN, f64::max);
    let v: Vec<f64> = centers.iter().zip(&db).filter(|(c, _)| **c >= 1000.0 && **c < 8100.0).map(|(_, d)| d - mx).collect();
    v.iter().sum::<f64>() / v.len() as f64
}

/// Move the unlocked tops (transient, body, spice) until the 1–8 kHz band sits on the gun target.
fn balance(e: &mut Engine, mut q: Params, locked: &[&str]) -> Params {
    let tops: Vec<usize> = [TR_LVL, BODY_LVL, SPICE_LVL].into_iter().filter(|&k| !locked.contains(&SPEC[k].group)).collect();
    if tops.is_empty() { return q; }
    for _ in 0..3 {
        let d = (-17.4 - high_band(&mono_shot(e, &q).0)).clamp(-18.0, 18.0);
        if d.abs() < 1.0 { break; }
        for &k in &tops { if q.0[k] > -59.0 { q.0[k] = (q.0[k] + d).clamp(-58.0, 6.0); } }
    }
    q
}

/// Copy every knob of the locked groups back from `p`; any lock also keeps the noise seed, which every
/// layer draws from.
fn keep(p: &Params, mut q: Params, locked: &[&str]) -> Params {
    if locked.is_empty() { return q; }
    for i in 0..N { if locked.contains(&SPEC[i].group) { q.0[i] = p.0[i]; } }
    q.0[SEED] = p.0[SEED];
    q
}

pub fn features(x: &[f32]) -> [f64; 8] {
    const NF: usize = 2048;
    const HOP: usize = 256;
    let x = &x[..x.len().min((0.6 * SR) as usize)];
    let win: Vec<f64> = (0..NF).map(|i| 0.5 - 0.5 * (2.0 * std::f64::consts::PI * i as f64 / NF as f64).cos()).collect();
    let bin = |f: f64| js_round(f * NF as f64 / SR) as usize;
    let (mut re, mut im) = (vec![0.0; NF], vec![0.0; NF]);
    let mut fr: Vec<Vec<f64>> = Vec::new();
    let mut o = 0;
    while o + NF <= x.len() {
        for i in 0..NF { re[i] = x[o + i] as f64 * win[i]; im[i] = 0.0; }
        fft(&mut re, &mut im, false);
        fr.push((0..NF / 2).map(|k| re[k] * re[k] + im[k] * im[k] + 1e-18).collect());
        o += HOP;
    }
    let band = |p: &[f64], a: f64, b: f64| -> f64 { (bin(a)..bin(b)).map(|k| p[k]).sum() };
    let tot: Vec<f64> = fr.iter().map(|p| band(p, 30.0, 16000.0)).collect();
    let emax = tot.iter().cloned().fold(f64::MIN, f64::max);
    let act: Vec<&Vec<f64>> = fr.iter().zip(&tot).filter(|(_, t)| **t > emax * 1e-3).map(|(p, _)| p).collect();
    let q = |a: &[f64], f: f64| { let mut s = a.to_vec(); s.sort_by(|x, y| x.partial_cmp(y).unwrap()); s[(f * (s.len() - 1) as f64).floor() as usize] };
    let db = |v: f64| 10.0 * (v + 1e-18).log10();
    let flat: Vec<f64> = act.iter().map(|p| { let (a, b) = (bin(400.0), bin(10000.0)); let (mut lg, mut ar) = (0.0, 0.0); for k in a..b { lg += p[k].ln(); ar += p[k]; } (lg / (b - a) as f64).exp() / (ar / (b - a) as f64) }).collect();
    let cen: Vec<f64> = act.iter().map(|p| { let (mut a, mut b) = (0.0, 0.0); for k in bin(100.0)..bin(16000.0) { a += k as f64 * p[k]; b += p[k]; } ((a / b) * SR / NF as f64).log2() }).collect();
    let mut lt = vec![0.0; NF / 2];
    for p in &act { for k in 0..NF / 2 { lt[k] += p[k]; } }
    let ldb: Vec<f64> = lt.iter().map(|&v| db(v)).collect();
    let (mut pk, mut c) = (0.0, 0.0);
    for k in bin(400.0)..bin(8000.0) {
        let w = (js_round(k as f64 * 0.115) as usize).max(1);
        let mut seg: Vec<f64> = ldb[k - w..(k + w + 1).min(ldb.len())].to_vec();
        seg.sort_by(|a, b| a.partial_cmp(b).unwrap());
        pk += (ldb[k] - seg[seg.len() >> 1]).max(0.0);
        c += 1.0;
    }
    let mid: Vec<f64> = fr.iter().map(|p| band(p, 300.0, 3000.0)).collect();
    let mpk = mid.iter().cloned().fold(f64::MIN, f64::max);
    let ip = mid.iter().position(|&v| v == mpk).unwrap_or(0);
    let mut j = ip;
    while j + 1 < mid.len() && mid[j] > mpk * 0.01 { j += 1; }
    let t60 = js_round(0.06 * SR / HOP as f64) as usize;
    let early: f64 = mid.iter().take(t60).sum();
    let late: f64 = mid.iter().skip(t60).sum();
    let n = x.len().min(act.len() * HOP);
    let (mut peak, mut ss) = (0f64, 0.0);
    for &v in &x[..n] { peak = peak.max((v as f64).abs()); ss += (v as f64).powi(2); }
    [
        q(&flat, 0.5),
        pk / c,
        q(&cen, 0.9) - q(&cen, 0.1),
        20.0 * (peak / (ss / n.max(1) as f64).sqrt()).log10(),
        db(band(&lt, 2500.0, 6000.0)) - db(band(&lt, 500.0, 2000.0)),
        db(band(&lt, 35.0, 90.0)) - db(band(&lt, 300.0, 3000.0)),
        ((j - ip) * HOP) as f64 / SR * 1000.0,
        db(late) - db(early),
    ]
}

pub fn gun_score(f: &[f64; 8]) -> f64 {
    let r = gun_ref();
    (0..8).map(|k| ((f[k] - r.median[k]).abs() / (1.4826 * r.mad[k] + 1e-9)).min(4.0)).sum::<f64>() / 8.0
}

/// Mids should be a block over the sub, not the sub's own clipping buzz riding its peaks.
pub fn lock_penalty(x: &[f32]) -> f64 {
    let (mut hi, mut lo) = (x.to_vec(), x.to_vec());
    lr4(&mut hi, Bq::Hp, 300.0, SR);
    lr4(&mut lo, Bq::Lp, 120.0, SR);
    let a0 = (0.02 * SR) as usize;
    let a1 = x.len().min((0.2 * SR) as usize);
    let (mut sx, mut sy, mut sxx, mut syy, mut sxy, mut k, mut sm, mut ph, mut pl) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0f64, 0f64);
    for i in 0..x.len() { ph = ph.max((hi[i] as f64).abs()); pl = pl.max((lo[i] as f64).abs()); }
    for i in a0..a1 {
        sm = 0.98 * sm + 0.02 * (hi[i] as f64).abs();
        let (u, v) = (sm, (lo[i] as f64).abs());
        sx += u; sy += v; sxx += u * u; syy += v * v; sxy += u * v; k += 1.0;
    }
    let am = if k > 0.0 { (sxy / k - (sx / k) * (sy / k)) / ((sxx / k - (sx / k).powi(2)) * (syy / k - (sy / k).powi(2)) + 1e-20).sqrt() } else { 0.0 };
    let mos = 20.0 * ((ph + 1e-9) / (pl + 1e-9)).log10();
    2.0 * (am + 0.1).max(0.0) + 0.15 * (-6.0 - mos).max(0.0)
}

#[derive(Clone, Copy, PartialEq)]
pub enum Mode { Randomize, Mutate }

/// Best of `k` candidates per thread over `threads` threads; lower score is more gun-like. Knobs in the
/// `locked` groups stay as they are in `p`.
pub fn roll(mode: Mode, p: &Params, seed: f64, threads: usize, k: usize, locked: &[&str]) -> Params {
    let limit = if mode == Mode::Mutate { (mono_shot(&mut Engine::new(), p).1 + 3.0).max(-25.0) } else { f64::INFINITY };
    let best = std::thread::scope(|sc| {
        let hs: Vec<_> = (0..threads)
            .map(|t| {
                sc.spawn(move || {
                    let mut r = Rng::new(seed + 7919.0 * t as f64);
                    let mut e = Engine::new();
                    let mut best: Option<(f64, Params)> = None;
                    let (mut i, mut tries) = (0, 0);
                    while i < k {
                        let q = match mode { Mode::Mutate => keep(p, mutate(p, &mut r, 0.06), locked), Mode::Randomize => balance(&mut e, keep(p, randomize(p, &mut r), locked), locked) };
                        let (m, late) = mono_shot(&mut e, &q);
                        tries += 1;
                        if late > limit && tries < 3 * k { continue; }
                        let s = gun_score(&features(&m)) + lock_penalty(&m);
                        if best.as_ref().map_or(true, |b| s < b.0) { best = Some((s, q)); }
                        i += 1;
                    }
                    best
                })
            })
            .collect();
        hs.into_iter().filter_map(|h| h.join().ok().flatten()).min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
    });
    best.map(|b| b.1).unwrap_or_else(|| p.clone())
}
