use tearout_dsp::{id::NOTE, Engine};
use tearout_plug::worker::fold_note;
fn bands(x: &[f32]) -> (f64, f64) {
    // crude: one-pole lowpass at ~150 Hz vs remainder
    let a = (-2.0 * std::f64::consts::PI * 150.0 / 48000.0).exp();
    let (mut y, mut lo, mut hi) = (0.0, 0.0, 0.0);
    for &s in x { y = a * y + (1.0 - a) * s as f64; lo += y * y; hi += (s as f64 - y).powi(2); }
    (10.0 * lo.log10(), 10.0 * hi.log10())
}
fn main() {
    let pre = &tearout_plug::presets::all()[tearout_plug::presets::default_index()];
    let p = pre.p.clone();
    let mut e = Engine::new();
    let base = e.render(&p);
    println!("base note {} len {}", p.0[NOTE], base.l.len());
    for n in [36u8, 48, 60, 72] {
        let t = fold_note(n);
        let rate = 2f64.powf((t - p.0[NOTE]) / 12.0);
        let mut rs = Vec::new();
        let mut pos = 0.0;
        while (pos as usize) + 1 < base.l.len() { let i = pos as usize; let f = (pos - i as f64) as f32; rs.push(base.l[i] + (base.l[i + 1] - base.l[i]) * f); pos += rate; }
        let mut q = p.clone(); q.0[NOTE] = t;
        let ex = Engine::new().render(&q);
        let pk = |x: &[f32]| x.iter().fold(0f32, |a, &b| a.max(b.abs()));
        println!("midi {n} -> {t} rate {rate:.3}  resampled lo/hi {:?} pk {:.2}   exact lo/hi {:?} pk {:.2}", bands(&rs), pk(&rs), bands(&ex.l), pk(&ex.l));
    }
}
#[allow(dead_code)]
fn _t() {}
