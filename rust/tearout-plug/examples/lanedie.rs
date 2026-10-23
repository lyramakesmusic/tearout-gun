// A stem's randomize die: over many rolls, how often the stem switches off, with and without holding stems.
use tearout_dsp::id::*;
use tearout_dsp::spec::SPEC;
use tearout_plug::roll::{hold_stems, keep, roll, Mode};

fn main() {
    let mut p = tearout_plug::presets::list()[0].p.clone();
    p.0[SYN_LVL] = -20.0;
    p.0[SPICE_LVL] = -20.0;
    let mut groups: Vec<&'static str> = Vec::new();
    for s in SPEC.iter() { if !groups.contains(&s.group) { groups.push(s.group); } }
    for (g, lvl) in [("sub", SUB_LVL), ("tr", TR_LVL), ("body", BODY_LVL), ("synth", SYN_LVL), ("spice", SPICE_LVL), ("rev", REV_LVL)] {
        let others: Vec<&'static str> = groups.iter().copied().filter(|&x| x != g).collect();
        let n = 60;
        let (mut raw_off, mut held_off, mut changed) = (0, 0, 0);
        for s in 0..n {
            let q = keep(&p, roll(Mode::Randomize, &p, 900.0 + s as f64, 4, 2, &others), &others);
            if q.0[lvl] <= -59.0 { raw_off += 1; }
            let mut h = q.clone();
            hold_stems(&p, &mut h);
            if h.0[lvl] <= -59.0 { held_off += 1; }
            if SPEC.iter().enumerate().any(|(i, sp)| sp.group == g && (h.0[i] - p.0[i]).abs() > 1e-9) { changed += 1; }
        }
        println!("{g:6} off without hold {raw_off:2}/{n}  off with hold {held_off}/{n}  stem still rerolled {changed}/{n}");
    }
}
