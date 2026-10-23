// Mutate keeps every stem's on/off state: count flips over many mutations from several patches.
use tearout_dsp::id::*;
use tearout_plug::roll::{roll, Mode};
fn main() {
    let base = tearout_plug::presets::list()[0].p.clone();
    let mut starts = vec![base.clone()];
    for s in 0..6 { starts.push(roll(Mode::Randomize, &base, 70.0 + s as f64, 2, 1, &[])); }
    let lv = [("sub", SUB_LVL), ("transient", TR_LVL), ("body", BODY_LVL), ("synth", SYN_LVL), ("spice", SPICE_LVL), ("reverb", REV_LVL)];
    let mut flips = vec![0; lv.len()];
    let mut n = 0;
    for p in &starts {
        for s in 0..10 {
            let mut q = roll(Mode::Mutate, p, 900.0 + s as f64, 2, 1, &[]);
            q.0[SYN_LVL] = if p.0[SYN_LVL] > -59.0 { -60.0 } else { -20.0 }; // force a flip, which hold_stems must undo
            tearout_plug::roll::hold_stems(p, &mut q);
            n += 1;
            for (k, &(_, l)) in lv.iter().enumerate() { if (q.0[l] > -59.0) != (p.0[l] > -59.0) { flips[k] += 1; } }
        }
    }
    for (k, &(name, _)) in lv.iter().enumerate() { println!("{name:9} on/off flipped in {}/{n} mutations", flips[k]); }
}
