// tg-memo-test <jobs.jsonl> <sample.f32 L> <sample.f32 R>: stage reuse must never change the output.
// For base patches, change every param in turn (and swap sample audio); the long-lived engine's render must be
// bit-identical to a fresh engine's.
use std::fs;
use tearout_dsp::{dsp::Rng, from_norm, Engine, Params, SPEC};

fn read_f32(path: &str) -> Vec<f32> { fs::read(path).unwrap().chunks_exact(4).map(|b| f32::from_le_bytes([b[0], b[1], b[2], b[3]])).collect() }

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let (sl, sr) = (read_f32(&a[2]), read_f32(&a[3]));
    let bases: Vec<Params> = fs::read_to_string(&a[1]).unwrap().lines().enumerate().filter(|(i, _)| [0, 5, 20, 47, 52, 60, 70, 90, 110].contains(i)).map(|(_, line)| {
        let j: serde_json::Value = serde_json::from_str(line).unwrap();
        let mut p = Params::default();
        for (k, v) in j["params"].as_object().unwrap() { if let Some(x) = v.as_f64() { p.set(k, x); } }
        p[tearout_dsp::id::S2_LVL] = -6.0;
        p
    }).collect();
    let mut r = Rng::new(99.0);
    let (mut checks, mut bad) = (0, 0);
    let mut live = Engine::new();
    live.set_sample(2, Some((&sl, &sr, 44100.0)));
    for (bi, base) in bases.iter().enumerate() {
        live.render(base);
        for (i, s) in SPEC.iter().enumerate() {
            if s.id == "b_shots" || s.id == "len" { continue; }
            let mut p = base.clone();
            p[i] = from_norm(s, r.next());
            let got = live.render(&p);
            let mut fresh = Engine::new();
            fresh.set_sample(2, Some((&sl, &sr, 44100.0)));
            let want = fresh.render(&p);
            checks += 1;
            if got.l != want.l || got.r != want.r { bad += 1; println!("base {bi}: stale after changing {}", s.id); }
        }
        // swapping the sample audio (reversed) must re-render that layer
        let rev: Vec<f32> = sl.iter().rev().cloned().collect();
        live.set_sample(2, Some((&rev, &sr, 44100.0)));
        let got = live.render(base);
        let mut fresh = Engine::new();
        fresh.set_sample(2, Some((&rev, &sr, 44100.0)));
        let want = fresh.render(base);
        checks += 1;
        if got.l != want.l { bad += 1; println!("base {bi}: stale after sample swap"); }
        live.set_sample(2, Some((&sl, &sr, 44100.0)));
    }
    println!("{checks} checks, {bad} stale");
}
