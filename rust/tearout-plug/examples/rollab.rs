// Writes web fitted rolls (JSON param maps) and plugin rolls as wavs for listening side by side.
use tearout_dsp::{id::*, Engine, Params, SPEC};
use tearout_plug::roll::*;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let out = std::path::PathBuf::from(&a[2]);
    std::fs::create_dir_all(&out).unwrap();
    let web: Vec<serde_json::Value> = serde_json::from_str(&std::fs::read_to_string(&a[1]).unwrap()).unwrap();
    let mut e = Engine::new();
    let mut put = |name: String, p: &Params| { let mut q = p.clone(); q.0[B_SHOTS] = 1.0; let o = e.render(&q); tearout_plug::export::write_wav(&out.join(name), &o.l, &o.r, 48000).unwrap(); };
    for (i, o) in web.iter().take(10).enumerate() {
        let mut p = Params::default();
        for (k, s) in SPEC.iter().enumerate() { p.0[k] = o.get(s.id).and_then(|v| v.as_f64()).unwrap_or(s.def); }
        put(format!("web {:02}.wav", i + 1), &p);
    }
    let pr = tearout_plug::presets::all();
    let base = &pr[tearout_plug::presets::default_index()].p;
    for i in 0..10 { let q = roll(Mode::Randomize, base, 900.0 + i as f64 * 17.0, 4, 2, &[]); put(format!("plugin {:02}.wav", i + 1), &q); }
}
