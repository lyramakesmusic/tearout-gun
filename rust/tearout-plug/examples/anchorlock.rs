// A stem locked after a randomize holds through the next mutate (the mutate anchor resets on randomize,
// and locked groups always come from the current patch).
use tearout_dsp::id::*;
use tearout_dsp::spec::SPEC;
use tearout_plug::roll::{keep, roll, Mode};
type P = tearout_dsp::Params;

fn same(a: &P, b: &P) -> bool { (0..SPEC.len()).all(|i| (a.0[i] - b.0[i]).abs() < 1e-9) }

/// The editor's reroll, old (`fixed` false) or new.
fn reroll(cur: &P, anchor: &mut Option<P>, last: &mut Option<P>, randomize: bool, locked: &[&'static str], seed: f64, fixed: bool) -> P {
    let q = if randomize {
        if fixed { *anchor = None; *last = None; }
        roll(Mode::Randomize, cur, seed, 2, 2, locked)
    } else {
        if last.as_ref().map_or(true, |m| !same(m, cur)) { *anchor = Some(cur.clone()); }
        roll(Mode::Mutate, anchor.as_ref().unwrap_or(cur), seed, 2, 2, locked)
    };
    let q = if fixed { keep(cur, q, locked) } else { q };
    *last = Some(q.clone());
    q
}

fn main() {
    let start = tearout_plug::presets::list()[0].p.clone();
    for fixed in [false, true] {
        let (mut anchor, mut last) = (None, None);
        let a = reroll(&start, &mut anchor, &mut last, false, &[], 1.0, fixed); // a mutate anchors on `start`
        let mut b = a.clone();
        for s in 2..300 { let q = reroll(&a, &mut anchor, &mut last, true, &[], s as f64, fixed); if q.0[SYN_LVL] > -59.0 { b = q; break; } }
        let c = reroll(&b, &mut anchor, &mut last, false, &["synth"], 999.0, fixed);
        let held = (0..SPEC.len()).filter(|&i| SPEC[i].group == "synth").all(|i| (c.0[i] - b.0[i]).abs() < 1e-9);
        println!("{}: synth before lock {:.1} dB, after mutate {:.1} dB, all synth knobs held: {held}", if fixed { "new" } else { "old" }, b.0[SYN_LVL], c.0[SYN_LVL]);
    }
}
