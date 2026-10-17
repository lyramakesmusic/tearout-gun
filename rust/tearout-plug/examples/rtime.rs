use tearout_dsp::{id::NOTE, Engine};
fn main() {
    let mut p = tearout_plug::presets::all()[tearout_plug::presets::default_index()].p.clone();
    let mut e = Engine::new();
    e.render(&p);
    for n in [30.0, 33.0, 36.0] { p.0[NOTE] = n; let t = std::time::Instant::now(); e.render(&p); println!("note render {:?}", t.elapsed()); }
}
