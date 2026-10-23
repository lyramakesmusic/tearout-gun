// Locked stems hold through rolls: synth locked, Randomize and Mutate never move a synth knob.
use tearout_dsp::spec::SPEC;
use tearout_dsp::id::*;
use tearout_plug::roll::{roll, Mode};

fn main() {
    let base = tearout_plug::presets::list()[0].p.clone();
    // find a randomized patch with synth on
    let mut cur = base.clone();
    for s in 1..200 { let q = roll(Mode::Randomize, &base, s as f64, 2, 1, &[]); if q.0[SYN_LVL] > -59.0 { cur = q; println!("synth on at seed {s}, lvl {:.1}", cur.0[SYN_LVL]); break; } }
    let locked = ["synth"];
    let synth: Vec<usize> = (0..SPEC.len()).filter(|&i| SPEC[i].group == "synth").collect();
    for (mode, name) in [(Mode::Randomize, "randomize"), (Mode::Mutate, "mutate")] {
        let mut bad = 0;
        for s in 0..20 {
            let q = roll(mode, &cur, 1000.0 + s as f64, 2, 2, &locked);
            let moved: Vec<&str> = synth.iter().filter(|&&i| (q.0[i] - cur.0[i]).abs() > 1e-9).map(|&i| SPEC[i].id).collect();
            if !moved.is_empty() { bad += 1; if bad <= 3 { println!("  {name} seed {s}: moved {:?} (syn_lvl {:.1} -> {:.1})", moved, cur.0[SYN_LVL], q.0[SYN_LVL]); } }
        }
        println!("{name}: {bad}/20 rolls moved a locked synth knob");
    }
}
