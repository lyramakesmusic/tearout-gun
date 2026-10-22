// Where the transient lands: onset (first 1 ms RMS within 12 dB of its peak) and peak time of the soloed
// transient stem, for the presets and for random rolls, with the knobs that can push it late.
use tearout_dsp::{id::*, Engine, Params};
use tearout_plug::roll::*;
fn tr_times(e: &mut Engine, p: &Params) -> Option<(f64, f64)> {
    if p.0[TR_LVL] <= -59.0 { return None; }
    let mut q = p.clone();
    q.0[B_SHOTS] = 1.0;
    for k in [SUB_LVL, BODY_LVL, SYN_LVL, SPICE_LVL, REV_LVL, S1_LVL, S1_LVL + 10, S1_LVL + 20] { q.0[k] = -60.0; }
    let o = e.render(&q);
    let w = 48;
    let env: Vec<f64> = o.l.chunks(w).map(|c| (c.iter().map(|v| (*v as f64).powi(2)).sum::<f64>() / w as f64).sqrt()).collect();
    let pk = env.iter().cloned().fold(0.0, f64::max);
    if pk < 1e-6 { return None; }
    let on = env.iter().position(|&v| v > pk * 0.25)? as f64;
    let at = env.iter().position(|&v| v == pk)? as f64;
    Some((on, at))
}
fn main() {
    let mut e = Engine::new();
    let row = |name: &str, p: &Params, t: Option<(f64, f64)>| {
        if let Some((on, at)) = t {
            println!("{name:<14} onset {on:>5.0} peak {at:>5.0} | delay {:>5.1} swell {:>5.1} att {:.2} stut {} mst_swell {:>5.1} zap {:>5.1} click {:>5.1}", p.0[TR_DELAY], p.0[TR_SWELL], p.0[TR_ATT], p.0[TR_STUTTER], p.0[MST_SWELL], p.0[TR_ZAP], p.0[TR_CLICK]);
        }
    };
    let pr = tearout_plug::presets::all();
    for x in pr.iter() { let t = tr_times(&mut e, &x.p); row(&x.name, &x.p, t); }
    println!("--- random");
    let base = &pr[tearout_plug::presets::default_index()].p;
    let n: usize = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(40);
    for i in 0..n {
        let q = roll(Mode::Randomize, base, 100.0 + i as f64 * 37.0, 4, 2, &[]);
        let t = tr_times(&mut e, &q);
        row(&format!("roll {i}"), &q, t);
    }
}
