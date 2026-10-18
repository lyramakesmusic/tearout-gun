// Gun presets, embedded from web/presets.json.
use std::sync::OnceLock;
use tearout_dsp::{Params, N, SPEC};

pub struct Preset { pub name: String, pub p: Params }

pub fn all() -> &'static [Preset] {
    static P: OnceLock<Vec<Preset>> = OnceLock::new();
    P.get_or_init(|| {
        let j: serde_json::Value = serde_json::from_str(include_str!("../../../web/presets.json")).unwrap_or_default();
        let mut out = Vec::new();
        for e in j.as_array().into_iter().flatten() {
            let ps = &e["params"];
            if ps.get("engine").and_then(|v| v.as_f64()).unwrap_or(0.0) != 0.0 { continue; }
            let mut p = Params::default();
            for i in 0..N { if let Some(v) = ps.get(SPEC[i].id).and_then(|v| v.as_f64()) { p.0[i] = v; } }
            p.0[tearout_dsp::id::B_SHOTS] = p.0[tearout_dsp::id::B_SHOTS].max(1.0);
            on_c(&mut p);
            let name = base_name(e["name"].as_str().unwrap_or("?")).to_string();
            if let Some(t) = measured_tune(&name) { p.0[tearout_dsp::id::SUB_TUNE] = t; }
            out.push(Preset { name, p });
        }
        out
    })
}

/// Puts a preset in C: the note moves to its nearest C, and the sub and synth tunings round to the
/// octave nearest their designed pitch, so every sub dives toward a C.
/// Layers that ignore the note (noise, transients, fixed resonances) stay as designed.
fn on_c(p: &mut Params) {
    use tearout_dsp::id::*;
    let n0 = p.0[NOTE];
    let c = ((n0 / 12.0).round() * 12.0).clamp(SPEC[NOTE].min, SPEC[NOTE].max);
    let oct = |t: f64, s: &tearout_dsp::Spec| (((n0 + t - c) / 12.0).round() * 12.0).clamp(s.min, s.max);
    p.0[NOTE] = c;
    p.0[SUB_TUNE] = oct(p.0[SUB_TUNE], &SPEC[SUB_TUNE]);
    p.0[SYN_TUNE] = oct(p.0[SYN_TUNE], &SPEC[SYN_TUNE]);
}

/// Sub tunings measured so each sub's steady pitch, slow dive tail included, sits exactly on its C
/// (`cargo run --example subtunes`). Subs that fade mid-dive keep the octave tuning.
fn measured_tune(name: &str) -> Option<f64> {
    static T: OnceLock<serde_json::Value> = OnceLock::new();
    T.get_or_init(|| serde_json::from_str(include_str!("sub_tunes.json")).unwrap_or_default()).get(name).and_then(|v| v.as_f64())
}

/// Preset name without a trailing key ("slag C" -> "slag").
pub fn base_name(n: &str) -> &str {
    match n.rsplit_once(' ') {
        Some((b, k)) if matches!(k.len(), 1 | 2) && k.as_bytes()[0].is_ascii_uppercase() && (k.len() == 1 || k.ends_with('#')) => b,
        _ => n,
    }
}

pub fn index_of(name: &str) -> Option<usize> { all().iter().position(|p| p.name == base_name(name)) }

pub fn default_index() -> usize { index_of("rivet").unwrap_or(0) }
