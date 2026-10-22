// Burst interval from the host grid: shots fill the division, accel bends the spacing.
use tearout_dsp::{id::*, Params};
fn main() {
    for (shots, beats, accel, bpm) in [(4.0, 1.0, 0.0, 150.0), (10.0, 0.5, 0.0, 150.0), (8.0, 1.0, 0.8, 140.0), (32.0, 4.0, -0.5, 174.0)] {
        let mut p = Params::default();
        p.0[B_SHOTS] = shots; p.0[B_ACCEL] = accel; p.0[B_RATE] = 90.0;
        tearout_plug::worker::grid_interval(&mut p, beats, bpm);
        let n = shots as usize;
        let mut on = vec![0.0];
        for s in 1..n { let f = (s - 1) as f64 / (n - 1) as f64; on.push(on[s - 1] + p.0[B_RATE] * 2f64.powf(-accel * f * 1.5)); }
        let gap = on[n - 1] - on[n - 2];
        println!("{n} shots over {beats} beats @ {bpm}: rate {:.2} ms, last onset {:.1} + gap {:.1} = {:.1}, span {:.1}", p.0[B_RATE], on[n - 1], gap, on[n - 1] + gap, beats * 60000.0 / bpm);
    }
}
