// Gun presets, embedded from web/presets.json.
use std::sync::OnceLock;
use tearout_dsp::{Params, N, SPEC};

#[derive(Clone)]
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
        // "default" is the sound the web app opens on
        // with its transient and body about half as long
        if let Some(f) = out.iter().find(|x| x.name == "filament").cloned() {
            use tearout_dsp::id::*;
            let mut p = f.p;
            for k in [TR_HOLD, TR_DEC, TR_REL, BODY_HOLD, BODY_DEC, BODY_REL] { p.0[k] = (p.0[k] * 0.5).max(SPEC[k].min); }
            // and spice on, quietly
            p.0[SPICE_LVL] = -20.0;
            // a sub that fades out over its last 180 ms, ending where it did
            let rl = p.0[SUB_REL].max(180.0);
            p.0[SUB_HOLD] = (p.0[SUB_HOLD] + p.0[SUB_REL] - rl).max(SPEC[SUB_HOLD].min);
            p.0[SUB_REL] = rl;
            // and a louder, longer reverb
            p.0[REV_LVL] = -8.0;
            p.0[REV_SIZE] = (p.0[REV_SIZE] * 1.6).min(SPEC[REV_SIZE].max);
            out.insert(0, Preset { name: "default".into(), p });
        }
        out
    })
}

/// Puts a preset in C: the note moves to its nearest C, and the sub and synth tunings round to the
/// octave nearest their designed pitch, so every sub dives toward a C.
/// Layers that ignore the note (noise, transients, fixed resonances) stay as designed.
fn on_c(p: &mut Params) {
    let n0 = p.0[tearout_dsp::id::NOTE];
    retune(p, ((n0 / 12.0).round() * 12.0).clamp(SPEC[tearout_dsp::id::NOTE].min, SPEC[tearout_dsp::id::NOTE].max));
}

/// Moves a patch to play at note `c`: the sub and synth tunings round to the octave of `c` nearest their
/// designed pitch.
pub fn retune(p: &mut Params, c: f64) {
    use tearout_dsp::id::*;
    let n0 = p.0[NOTE];
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

/// The preset a new instance opens on.
pub fn default_index() -> usize { index_of("default").unwrap_or(0) }

/// A preset in the menu: a factory one, or one the user saved (with the samples it layers).
#[derive(Clone)]
pub struct Entry { pub name: String, pub p: Params, pub user: bool, pub samples: [String; 3] }

impl Entry {
    /// How the plugin state remembers which preset is loaded.
    pub fn key(&self) -> String { if self.user { format!("user/{}", self.name) } else { self.name.clone() } }
}

/// Where saved presets live: one JSON file each, in the web app's preset format plus sample paths.
pub fn user_dir() -> std::path::PathBuf { crate::export::root().join("Presets") }

fn file_name(name: &str) -> String {
    let s: String = name.chars().map(|c| if c.is_alphanumeric() || " -_".contains(c) { c } else { '_' }).collect();
    format!("{}.json", s.trim())
}

/// Factory presets, then saved ones by name.
pub fn list() -> Vec<Entry> {
    let mut out: Vec<Entry> = all().iter().map(|p| Entry { name: p.name.clone(), p: p.p.clone(), user: false, samples: Default::default() }).collect();
    let mut user: Vec<Entry> = std::fs::read_dir(user_dir())
        .into_iter()
        .flatten()
        .flatten()
        .filter(|f| f.path().extension().is_some_and(|e| e == "json"))
        .filter_map(|f| {
            let j: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(f.path()).ok()?).ok()?;
            let ps = &j["params"];
            let mut p = Params::default();
            for i in 0..N { p.0[i] = ps.get(SPEC[i].id).and_then(|v| v.as_f64()).unwrap_or(SPEC[i].def); }
            let mut samples: [String; 3] = Default::default();
            for (k, s) in j["samples"].as_array().into_iter().flatten().take(3).enumerate() { samples[k] = s.as_str().unwrap_or("").to_string(); }
            Some(Entry { name: j["name"].as_str()?.to_string(), p, user: true, samples })
        })
        .collect();
    user.sort_by_key(|e| e.name.to_lowercase());
    out.extend(user);
    out
}

/// Save (or overwrite) a user preset.
pub fn save(name: &str, p: &Params, samples: &[String; 3]) -> std::io::Result<()> {
    let d = user_dir();
    std::fs::create_dir_all(&d)?;
    let params: serde_json::Map<String, serde_json::Value> = (0..N).map(|i| (SPEC[i].id.to_string(), serde_json::json!(p.0[i]))).collect();
    let j = serde_json::json!({ "name": name, "params": params, "samples": samples });
    std::fs::write(d.join(file_name(name)), serde_json::to_string_pretty(&j).unwrap_or_default() + "\n")
}

/// Move a user preset into the presets folder's "deleted" folder.
pub fn delete(name: &str) -> std::io::Result<()> {
    let d = user_dir();
    std::fs::create_dir_all(d.join("deleted"))?;
    std::fs::rename(d.join(file_name(name)), d.join("deleted").join(file_name(name)))
}
