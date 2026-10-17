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
            // every preset sits on its nearest C, so a C on the keyboard plays it as designed
            let n = p.0[tearout_dsp::id::NOTE];
            let c = ((n / 12.0).round() * 12.0).clamp(SPEC[tearout_dsp::id::NOTE].min, SPEC[tearout_dsp::id::NOTE].max);
            p.0[tearout_dsp::id::NOTE] = c;
            out.push(Preset { name: e["name"].as_str().unwrap_or("?").to_string(), p });
        }
        out
    })
}

pub fn default_index() -> usize { all().iter().position(|p| p.name == "rivet E").unwrap_or(0) }
