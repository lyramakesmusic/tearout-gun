// The lane level drag floor sits at -58 dB in plain units, above the -59 dB cutoff that hides a lane.
use tearout_dsp::{from_norm, to_norm, SPEC};
fn main() {
    for i in [tearout_dsp::id::SUB_LVL, tearout_dsp::id::TR_LVL, tearout_dsp::id::BODY_LVL, tearout_dsp::id::REV_LVL] {
        let f = to_norm(&SPEC[i], -58.0) as f32;
        let back = from_norm(&SPEC[i], f as f64);
        println!("{:10} floor={f:.4} -> {back:.2} dB, hides={}", SPEC[i].id, back <= -59.0);
    }
}
