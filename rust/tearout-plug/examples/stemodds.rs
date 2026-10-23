// How often a randomize turns each unlocked stem on from the default, and off from a patch that has it.
use tearout_dsp::id::*;
use tearout_plug::roll::{roll, Mode};
fn main() {
    let base = tearout_plug::presets::list()[0].p.clone();
    for (name, lvl) in [("synth", SYN_LVL), ("spice", SPICE_LVL), ("transient", TR_LVL), ("body", BODY_LVL), ("reverb", REV_LVL)] {
        let n = 30;
        let on_base = base.0[lvl] > -59.0;
        let flips = (0..n).filter(|&s| (roll(Mode::Randomize, &base, 500.0 + s as f64, 2, 1, &[]).0[lvl] > -59.0) != on_base).count();
        println!("{name:9} default {}: {flips}/{n} randomizes turn it {}", if on_base { "on" } else { "off" }, if on_base { "off" } else { "on" });
    }
}
