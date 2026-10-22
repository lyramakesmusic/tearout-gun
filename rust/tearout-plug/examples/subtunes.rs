// Writes tearout-plug/src/sub_tunes.json: for every preset whose sub holds a steady pitch, the tuning that
// puts that pitch exactly on C (`roll::sub_on_note`). Subs without one keep their octave tuning.
use tearout_dsp::Engine;
fn main() {
    let mut e = Engine::new();
    let mut out = serde_json::Map::new();
    for pr in tearout_plug::presets::all() {
        match tearout_plug::roll::sub_on_note(&mut e, &pr.p) {
            Some(t) => { println!("{:11} tune {:+6.2}", pr.name, t); out.insert(pr.name.clone(), serde_json::json!((t * 1000.0).round() / 1000.0)); }
            None => println!("{:11} sweep", pr.name),
        }
    }
    std::fs::write(concat!(env!("CARGO_MANIFEST_DIR"), "/src/sub_tunes.json"), serde_json::to_string_pretty(&out).unwrap() + "\n").unwrap();
}
