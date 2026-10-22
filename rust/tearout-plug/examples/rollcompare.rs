// Gun score (lower = more gun-like) of web fitted rolls (JSON of param maps) and plugin rolls, scored alike.
use tearout_dsp::{id::*, Engine, Params, SPEC};
use tearout_plug::roll::*;
fn score(e: &mut Engine, p: &Params) -> f64 {
    let mut q = p.clone(); q.0[B_SHOTS] = 1.0;
    let o = e.render(&q);
    let m: Vec<f32> = o.l.iter().zip(&o.r).map(|(a, b)| 0.5 * (a + b)).collect();
    gun_score(&features(&m)) + lock_penalty(&m)
}
fn stats(name: &str, v: &mut Vec<f64>) {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mean = v.iter().sum::<f64>() / v.len() as f64;
    println!("{name:<28} n {:>3} mean {:.3} median {:.3} p90 {:.3}", v.len(), mean, v[v.len() / 2], v[v.len() * 9 / 10]);
}
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let web: Vec<serde_json::Value> = serde_json::from_str(&std::fs::read_to_string(&a[1]).unwrap()).unwrap();
    let mut e = Engine::new();
    let mut ws: Vec<f64> = web.iter().map(|o| { let mut p = Params::default(); for (i, s) in SPEC.iter().enumerate() { p.0[i] = o.get(s.id).and_then(|v| v.as_f64()).unwrap_or(s.def); } score(&mut e, &p) }).collect();
    stats("web fitted (own notes)", &mut ws);
    let pr = tearout_plug::presets::all();
    let base = &pr[tearout_plug::presets::default_index()].p;
    let n: usize = a.get(2).and_then(|s| s.parse().ok()).unwrap_or(30);
    let mut ps: Vec<f64> = (0..n).map(|i| { let q = roll(Mode::Randomize, base, 500.0 + i as f64 * 13.0, 4, 2, &[]); score(&mut e, &q) }).collect();
    stats("plugin random (at C)", &mut ps);
    let mut pres: Vec<f64> = pr.iter().map(|x| score(&mut e, &x.p)).collect();
    stats("presets (at C)", &mut pres);
}
