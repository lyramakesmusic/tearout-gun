// Locked groups come through random and mutate untouched; the rest moves.
use tearout_dsp::SPEC;
use tearout_plug::roll::*;
fn main() {
    let pr = tearout_plug::presets::all();
    let p = &pr[tearout_plug::presets::default_index()].p;
    for mode in [Mode::Randomize, Mode::Mutate] {
        let q = roll(mode, p, 7.0, 4, 2, &["sub", "tr"]);
        let moved = |g: &str| (0..SPEC.len()).filter(|&i| SPEC[i].group == g && (q.0[i] - p.0[i]).abs() > 1e-9).count();
        println!("{}: sub {} tr {} body {} synth {} seed kept {}", if mode == Mode::Mutate { "mutate" } else { "random" }, moved("sub"), moved("tr"), moved("body"), moved("synth"), q.0[tearout_dsp::id::SEED] == p.0[tearout_dsp::id::SEED]);
    }
}
