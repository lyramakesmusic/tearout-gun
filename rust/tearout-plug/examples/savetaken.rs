// Which save names would replace another saved preset. Run with HOME pointed at a scratch folder.
fn main() {
    let p = tearout_plug::presets::list()[0].p.clone();
    tearout_plug::presets::save("AAAAA", &p, &Default::default()).unwrap();
    let t = tearout_plug::presets::taken;
    println!("AAAAA new save {}  resave while loaded {}  'aaaaa' with another loaded {}  unused name {}", t("AAAAA", None), t("AAAAA", Some("AAAAA")), t("aaaaa", Some("other")), t("BBBBB", None));
}
