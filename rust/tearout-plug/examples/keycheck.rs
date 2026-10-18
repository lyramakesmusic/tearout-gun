// Renders every preset at its stored note and at its nearest C, plus the non-tracking remainder
// (sub, synth and samples muted) at C and C+3, for pitch analysis.
use tearout_dsp::{id::*, Engine, Params, N, SPEC};
fn main() {
    let out = std::env::args().nth(1).unwrap();
    std::fs::create_dir_all(&out).unwrap();
    let j: serde_json::Value = serde_json::from_str(include_str!("../../../web/presets.json")).unwrap();
    let mut e = Engine::new();
    let mono = |o: tearout_dsp::Out| -> Vec<f32> { o.l.iter().zip(&o.r).map(|(a, b)| 0.5 * (a + b)).collect() };
    for (k, x) in j.as_array().unwrap().iter().enumerate() {
        let ps = &x["params"];
        let mut p = Params::default();
        for i in 0..N { if let Some(v) = ps.get(SPEC[i].id).and_then(|v| v.as_f64()) { p.0[i] = v; } }
        p.0[B_SHOTS] = 1.0;
        let n0 = p.0[NOTE];
        let c = ((n0 / 12.0).round() * 12.0).clamp(SPEC[NOTE].min, SPEC[NOTE].max);
        let at = |q: &Params, n: f64| { let mut q = q.clone(); q.0[NOTE] = n; q };
        let mut rest = p.clone();
        for l in [SUB_LVL, SYN_LVL, S1_LVL, S1_LVL + 10, S1_LVL + 20] { rest.0[l] = -60.0; }
        let w = |tag: &str, v: Vec<f32>| tearout_plug::export::write_wav(&std::path::PathBuf::from(format!("{out}/{k:02}_{tag}.wav")), &v, &v, tearout_dsp::SR as u32).unwrap();
        w("orig", mono(e.render(&at(&p, n0))));
        w("snap", mono(e.render(&at(&p, c))));
        w("rest0", mono(e.render(&at(&rest, c))));
        w("rest3", mono(e.render(&at(&rest, c + 3.0))));
        let g = |i: usize| p.0[i];
        println!("{}", serde_json::json!({"k": k, "name": x["name"], "n0": n0, "c": c,
            "sub": g(SUB_LVL), "body": g(BODY_LVL), "tr": g(TR_LVL), "syn": g(SYN_LVL), "spice": g(SPICE_LVL),
            "b_key": g(BODY_KEY), "b_modes": g(BODY_MODES), "b_mode_f": g(BODY_MODE_F), "b_q": g(BODY_Q), "b_comb": g(BODY_COMB), "b_comb_f": g(BODY_COMB_F), "b_comb_fb": g(BODY_COMB_FB),
            "b_fm": g(BODY_FM), "b_noise": g(BODY_NOISE), "cr_pk_f": g(CR_PK_F), "cr_mix": g(CR_MIX)}));
    }
}
