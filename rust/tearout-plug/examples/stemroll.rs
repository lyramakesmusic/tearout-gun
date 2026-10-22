// A stem's own dice: rerolling one group leaves every knob outside it untouched.
use tearout_dsp::spec::SPEC;
use tearout_plug::roll::{roll, Mode};

fn main() {
    let p = tearout_plug::presets::list()[0].p.clone();
    let mut groups: Vec<&'static str> = Vec::new();
    for s in SPEC.iter() { if !groups.contains(&s.group) { groups.push(s.group); } }
    for g in ["sub", "tr", "body", "synth", "spice", "rev"] {
        let others: Vec<&'static str> = groups.iter().copied().filter(|&x| x != g).collect();
        for (mode, name) in [(Mode::Randomize, "randomize"), (Mode::Mutate, "mutate")] {
            let q = roll(mode, &p, 42.0, 2, 2, &others);
            let (mut inside, mut outside) = (0, Vec::new());
            for (i, s) in SPEC.iter().enumerate() {
                if (q.0[i] - p.0[i]).abs() > 1e-9 { if s.group == g { inside += 1 } else { outside.push(s.id) } }
            }
            println!("{g:6} {name:9} changed in-group {inside:2}  outside {:?}", outside);
        }
    }
}
