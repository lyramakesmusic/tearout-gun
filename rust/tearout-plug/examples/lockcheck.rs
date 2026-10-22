// Locked groups come through random and mutate untouched; the locked layer's own stem should sound the same.
use tearout_dsp::{id::*, Engine, Params, SPEC};
use tearout_plug::roll::*;
fn solo(e: &mut Engine, p: &Params, lvl: usize) -> Vec<f32> {
    let mut q = p.clone();
    for k in [SUB_LVL, TR_LVL, BODY_LVL, SYN_LVL, SPICE_LVL, REV_LVL, S1_LVL, S1_LVL + 10, S1_LVL + 20] { if k != lvl { q.0[k] = -60.0; } }
    e.render(&q).l
}
fn diff_db(a: &[f32], b: &[f32]) -> f64 {
    let n = a.len().min(b.len());
    let (mut d, mut s) = (0.0, 0.0);
    for i in 0..n { d += ((a[i] - b[i]) as f64).powi(2); s += (a[i] as f64).powi(2); }
    10.0 * (d / s.max(1e-12)).log10()
}
fn main() {
    let pr = tearout_plug::presets::all();
    let p = &pr[tearout_plug::presets::default_index()].p;
    let mut e = Engine::new();
    for (g, lvl) in [("sub", SUB_LVL), ("tr", TR_LVL), ("body", BODY_LVL)] {
        for mode in [Mode::Randomize, Mode::Mutate] {
            let mut worst = f64::MIN;
            for seed in 0..4 {
                let q = roll(mode, p, 7.0 + seed as f64 * 31.0, 4, 2, &[g]);
                let moved = (0..SPEC.len()).filter(|&i| SPEC[i].group == g && (q.0[i] - p.0[i]).abs() > 1e-9).count();
                assert_eq!(moved, 0);
                worst = worst.max(diff_db(&solo(&mut e, p, lvl), &solo(&mut e, &q, lvl)));
            }
            println!("lock {g:<5} {:<6}: locked knobs unchanged; locked stem changes by {worst:.1} dB (residual re signal, worst of 4)", if mode == Mode::Mutate { "mutate" } else { "random" });
        }
    }
}
