// A factory preset by name: loads, and each knob matches the value in web/presets.json.
use tearout_dsp::spec::SPEC;
fn main() {
    let name = std::env::args().nth(1).unwrap();
    let j: serde_json::Value = serde_json::from_str(include_str!("../../../web/presets.json")).unwrap();
    let src = j.as_array().unwrap().iter().find(|e| e["name"] == name.as_str()).expect("not in presets.json");
    let e = tearout_plug::presets::list().into_iter().find(|e| !e.user && e.name == name).expect("not in the plugin's list");
    let (mut hit, mut off) = (0, Vec::new());
    for (i, s) in SPEC.iter().enumerate() {
        if let Some(v) = src["params"][s.id].as_f64() { hit += 1; if (e.p.0[i] - v).abs() > 1e-6 { off.push((s.id, v, e.p.0[i])); } }
    }
    println!("{name}: {hit} knobs from the file, {} differ {:?}", off.len(), off);
}
