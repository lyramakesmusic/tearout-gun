use tearout_dsp::{id::*, Engine};
use tearout_plug::roll::*;
fn main() {
    let pr = &tearout_plug::presets::all();
    let i = pr.iter().position(|p| p.name == "rivet E").unwrap();
    let mut p = pr[i].p.clone(); p.0[B_SHOTS] = 1.0;
    let o = Engine::new().render(&p);
    let m: Vec<f32> = o.l.iter().zip(&o.r).map(|(a, b)| 0.5 * (a + b)).collect();
    let f = features(&m);
    println!("feat {:?}\nscore {:.4} lock {:.4}", f, gun_score(&f), lock_penalty(&m));
    for mode in [Mode::Randomize, Mode::Mutate] {
        let t = std::time::Instant::now();
        let q = roll(mode, &pr[i].p, 42.0, 4, 2);
        let mut q1 = q.clone(); q1.0[B_SHOTS] = 1.0;
        let o = Engine::new().render(&q1);
        let m: Vec<f32> = o.l.iter().zip(&o.r).map(|(a, b)| 0.5 * (a + b)).collect();
        let f = features(&m);
        println!("{} in {:?}: score {:.3}", if mode == Mode::Mutate { "mutate" } else { "randomize" }, t.elapsed(), gun_score(&f) + lock_penalty(&m));
    }
}
