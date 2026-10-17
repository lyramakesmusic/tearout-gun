// tg-bench <jobs.jsonl> <jobs a-b> <seconds>: renders the jobs round-robin on one warm engine; prints ms per render.
use std::{env, fs, time::Instant};
use tearout_dsp::{Engine, Params};

fn main() {
    let a: Vec<String> = env::args().collect();
    let (lo, hi) = a[2].split_once('-').map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())).unwrap();
    let secs: f64 = a[3].parse().unwrap();
    let ps: Vec<Params> = fs::read_to_string(&a[1]).unwrap().lines().skip(lo).take(hi - lo).map(|line| {
        let j: serde_json::Value = serde_json::from_str(line).unwrap();
        let mut p = Params::default();
        for (k, v) in j["params"].as_object().unwrap() { if let Some(x) = v.as_f64() { p.set(k, x); } }
        p
    }).collect();
    let mut eng = Engine::new();
    let t = Instant::now();
    let mut n = 0;
    while t.elapsed().as_secs_f64() < secs { std::hint::black_box(eng.render(&ps[n % ps.len()])); n += 1; }
    println!("{:.2} ms/render over {} renders", t.elapsed().as_secs_f64() * 1000.0 / n as f64, n);
    if cfg!(feature = "timing") { println!("{}", tearout_dsp::timing::report(n)); }
}
