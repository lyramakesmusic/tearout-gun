// The transient stem of a preset: its gate knobs and where its energy actually ends, through the chains and
// with crunch off.
use tearout_dsp::{id::*, Engine, Params};
fn end_ms(e: &mut Engine, p: &Params) -> f64 {
    let o = e.render(p);
    let w = 48;
    let env: Vec<f64> = o.l.chunks(w).map(|c| (c.iter().map(|v| (*v as f64).powi(2)).sum::<f64>() / w as f64).sqrt()).collect();
    let pk = env.iter().cloned().fold(0.0, f64::max);
    env.iter().rposition(|&v| v > pk * 0.03).unwrap_or(0) as f64
}
fn main() {
    let name = std::env::args().nth(1).unwrap_or("filament".into());
    let pr = tearout_plug::presets::all();
    let mut p = pr.iter().find(|x| x.name == name).unwrap().p.clone();
    p.0[B_SHOTS] = 1.0;
    for k in [SUB_LVL, BODY_LVL, SYN_LVL, SPICE_LVL, REV_LVL] { p.0[k] = -60.0; }
    println!("{name}: hold {:.1} gate {:.1} decay {:.1} shape {} stutter {} | crunch mix {:.2} | post fall {:.0}", p.0[TR_HOLD], p.0[TR_REL], p.0[TR_DEC], p.0[TR_SHAPE], p.0[TR_STUTTER], p.0[CR_MIX], p.0[MST_FALL]);
    let mut e = Engine::new();
    println!("  ends (-30 dB) at {:.0} ms through the chains", end_ms(&mut e, &p));
    let mut q = p.clone(); q.0[CR_MIX] = 0.0;
    println!("  ends at {:.0} ms with crunch off", end_ms(&mut e, &q));
}
