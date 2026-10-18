// Writes tearout-plug/src/sub_tunes.json: for every preset whose sub holds a steady pitch, the tuning that
// puts that pitch exactly on C. Pitch comes from the sub's dive formula, loudness from the rendered sub
// (drive included); the steady part is the longest stretch within 20 dB of the peak whose pitch moves
// under 0.4 semitones, and it must last 80 ms. Subs without one keep their octave tuning.
use tearout_dsp::{id::*, Engine, SPEC};
fn main() {
    let mut e = Engine::new();
    let mut out = serde_json::Map::new();
    for pr in tearout_plug::presets::all() {
        let mut p = pr.p.clone();
        p.0[SUB_TUNE] = (p.0[SUB_TUNE] / 12.0).round() * 12.0;
        p.0[B_SHOTS] = 1.0;
        for l in [TR_LVL, BODY_LVL, SYN_LVL, SPICE_LVL, REV_LVL] { p.0[l] = -60.0; }
        let o = e.render(&p);
        let sr = tearout_dsp::SR;
        let hop = (sr / 1000.0) as usize;
        // loudness per ms over a 20 ms window
        let pw: Vec<f64> = (0..o.l.len() / hop).map(|i| {
            let a = (i * hop).saturating_sub(10 * hop); let b = (i * hop + 10 * hop).min(o.l.len());
            o.l[a..b].iter().map(|&v| (v as f64).powi(2)).sum::<f64>() / (b - a) as f64
        }).collect();
        let pk = pw.iter().cloned().fold(0.0, f64::max);
        let (tau, k, tc, dl) = (p.0[SUB_TAU] / 1000.0, p.0[SUB_K], p.0[SUB_CLICK_MS] / 1000.0, p.0[SUB_DELAY]);
        let off = |ms: usize| { let t = ((ms as f64 - dl) / 1000.0).max(0.0); p.0[SUB_DIVE] * (-(t / tau).powf(k)).exp() + p.0[SUB_CLICK] * (-t / tc).exp() };
        let loud = |i: usize| pw[i] > pk * 0.01;
        let mut best = (0usize, 0usize);
        let mut i = 0;
        while i < pw.len() {
            if !loud(i) { i += 1; continue; }
            let mut j = i;
            while j + 1 < pw.len() && loud(j + 1) && (off(i) - off(j + 1)).abs() < 0.4 { j += 1; }
            if j - i > best.1 - best.0 { best = (i, j); }
            i = j + 1;
        }
        let ms = best.1 - best.0;
        if ms < 80 { println!("{:11} sweep (steady {ms} ms)", pr.name); continue; }
        let (mut a, mut w) = (0.0, 0.0);
        for i in best.0..=best.1 { a += off(i) * pw[i]; w += pw[i]; }
        let held = a / w;
        let t = (p.0[SUB_TUNE] - held).clamp(SPEC[SUB_TUNE].min, SPEC[SUB_TUNE].max);
        println!("{:11} steady {ms:4} ms from {:4} ms, heard {:+5.2} st sharp -> tune {:+6.2}", pr.name, best.0, held, t);
        out.insert(pr.name.clone(), serde_json::json!((t * 1000.0).round() / 1000.0));
    }
    std::fs::write(concat!(env!("CARGO_MANIFEST_DIR"), "/src/sub_tunes.json"), serde_json::to_string_pretty(&out).unwrap() + "\n").unwrap();
}
