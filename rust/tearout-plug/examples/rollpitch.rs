// Random rolls: how many subs hold a steady pitch, and whether that pitch lands on the note.
use tearout_dsp::{id::*, Engine};
use tearout_plug::roll::*;
fn main() {
    let pr = tearout_plug::presets::all();
    let base = &pr[tearout_plug::presets::default_index()].p;
    let mut e = Engine::new();
    let (mut steady, mut worst) = (0, 0f64);
    let n = 20;
    for i in 0..n {
        let q = roll(Mode::Randomize, base, 300.0 + i as f64 * 7.0, 4, 2, &[]);
        if let Some(t) = sub_on_note(&mut e, &q) { steady += 1; worst = worst.max((t - q.0[SUB_TUNE]).abs()); }
        if i < 6 { println!("roll {i}: note {} sub tune {:+.2}", q.0[NOTE], q.0[SUB_TUNE]); }
    }
    println!("{steady}/{n} rolls hold a steady sub; worst re-measured offset {worst:.4} st");
}
