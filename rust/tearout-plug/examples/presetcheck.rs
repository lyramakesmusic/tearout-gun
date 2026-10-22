// Saved presets round-trip through their files (into a scratch folder).
use tearout_dsp::{id::*, SPEC};
fn main() {
    let dir = std::env::temp_dir().join("tf-preset-check");
    let _ = std::fs::remove_dir_all(&dir);
    tearout_plug::export::set_root(dir.clone());
    let mut p = tearout_plug::presets::all()[0].p.clone();
    p.0[SUB_DIVE] = 41.25; p.0[SYN_WAVE] = 4.0;
    let samples = ["/tmp/a.wav".to_string(), String::new(), String::new()];
    tearout_plug::presets::save("my gun", &p, &samples).unwrap();
    tearout_plug::presets::save("my gun", &p, &samples).unwrap(); // overwrite
    let l = tearout_plug::presets::list();
    let u: Vec<_> = l.iter().filter(|e| e.user).collect();
    assert_eq!(u.len(), 1);
    let bad = (0..SPEC.len()).filter(|&i| (u[0].p.0[i] - p.0[i]).abs() > 1e-12).count();
    println!("{} factory + {} saved; '{}' key {} params differing {} samples {:?}", l.len() - u.len(), u.len(), u[0].name, u[0].key(), bad, u[0].samples);
    tearout_plug::presets::delete("my gun").unwrap();
    println!("after delete: {} saved; deleted folder has {:?}", tearout_plug::presets::list().iter().filter(|e| e.user).count(), std::fs::read_dir(dir.join("Presets/deleted")).unwrap().count());
}
