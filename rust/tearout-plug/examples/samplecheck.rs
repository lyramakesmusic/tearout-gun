// Decode a file, layer it in slot 1 and report the mix with and without it.
use tearout_dsp::{id::*, Engine};
fn main() {
    let path = std::env::args().nth(1).expect("audio file");
    let (l, r, sr) = tearout_plug::worker::decode(&path).expect("decode");
    println!("decoded {} frames @ {sr} Hz, peak {:.3}", l.len(), l.iter().chain(&r).fold(0f32, |a, &b| a.max(b.abs())));
    let pr = tearout_plug::presets::all();
    let mut p = pr[tearout_plug::presets::default_index()].p.clone();
    let mut e = Engine::new();
    let rms = |o: &tearout_dsp::Out| (o.l.iter().map(|v| (*v as f64).powi(2)).sum::<f64>() / o.l.len() as f64).sqrt();
    let a = e.render(&p);
    e.set_sample(1, Some((&l, &r, sr)));
    p.0[S1_LVL] = -6.0;
    let b = e.render(&p);
    let mut solo = p.clone();
    for k in [SUB_LVL, TR_LVL, BODY_LVL, SYN_LVL, SPICE_LVL, REV_LVL] { solo.0[k] = -60.0; }
    let c = e.render(&solo);
    println!("rms without {:.4}  with {:.4}  sample alone {:.4}", rms(&a), rms(&b), rms(&c));
}
