// tg-render <jobs.jsonl> <outdir>: one job per line, {"params": {...}, "samples": {"1": {"l": "a.f32", "r": "b.f32", "rate": 44100}}}.
// Writes <outdir>/<line>.f32 (L then R, little-endian f32) and prints per-job render time in ms.
use std::{env, fs, time::Instant};
use tearout_dsp::{Engine, Params};

fn read_f32(path: &str) -> Vec<f32> {
    fs::read(path).expect(path).chunks_exact(4).map(|b| f32::from_le_bytes([b[0], b[1], b[2], b[3]])).collect()
}

fn main() {
    let a: Vec<String> = env::args().collect();
    let (jobs, out) = (&a[1], &a[2]);
    fs::create_dir_all(out).unwrap();
    for (i, line) in fs::read_to_string(jobs).unwrap().lines().enumerate() {
        let j: serde_json::Value = serde_json::from_str(line).unwrap();
        let mut p = Params::default();
        for (k, v) in j["params"].as_object().unwrap() {
            if let Some(x) = v.as_f64() { p.set(k, x); }
        }
        let mut eng = Engine::new();
        if let Some(s) = j.get("samples").and_then(|s| s.as_object()) {
            for (slot, d) in s {
                let (l, r) = (read_f32(d["l"].as_str().unwrap()), read_f32(d["r"].as_str().unwrap()));
                eng.set_sample(slot.parse().unwrap(), Some((&l, &r, d["rate"].as_f64().unwrap())));
            }
        }
        let t = Instant::now();
        let o = eng.render(&p);
        let ms = t.elapsed().as_secs_f64() * 1000.0;
        let mut bytes = Vec::with_capacity(o.l.len() * 8);
        for v in o.l.iter().chain(o.r.iter()) { bytes.extend_from_slice(&v.to_le_bytes()); }
        fs::write(format!("{out}/{i}.f32"), bytes).unwrap();
        println!("{i}\t{ms:.2}");
    }
}
