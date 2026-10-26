// How long one patch render takes, per factory preset (the time a note would wait for a fresh render).
fn main() {
    let mut eng = tearout_dsp::Engine::new();
    let list = tearout_plug::presets::list();
    let mut ts: Vec<(f64, String)> = Vec::new();
    for e in list.iter().filter(|e| !e.user) {
        let _ = eng.render(&e.p); // warm the IR cache like a running plugin
        let mut p = e.p.clone();
        p.0[tearout_dsp::id::B_SHOTS] = 4.0;
        let t = std::time::Instant::now();
        let _ = eng.render(&p);
        ts.push((t.elapsed().as_secs_f64() * 1000.0, e.name.clone()));
    }
    ts.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let n = ts.len();
    println!("{n} presets, 4 shots, re-render after a change: min {:.1} ms  median {:.1} ms  p90 {:.1} ms  max {:.1} ms ({})", ts[0].0, ts[n / 2].0, ts[n * 9 / 10].0, ts[n - 1].0, ts[n - 1].1);
}
