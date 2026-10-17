use tearout_plug::Tearout;
fn lo_hi(x: &[f32]) -> (f64, f64) {
    let a = (-2.0 * std::f64::consts::PI * 150.0 / 48000.0).exp();
    let (mut y, mut lo, mut hi) = (0.0, 0.0, 0.0);
    for &s in x { y = a * y + (1.0 - a) * s as f64; lo += y * y; hi += (s as f64 - y).powi(2); }
    (10.0 * lo.max(1e-12).log10(), 10.0 * hi.max(1e-12).log10())
}
fn hit(t: &mut Tearout, note: u8) -> Vec<f32> {
    let mut out = Vec::new();
    for b in 0..100 {
        let (mut l, mut r) = (vec![0f32; 512], vec![0f32; 512]);
        let ev = if b == 0 { vec![(0usize, note, 0.8f32)] } else { vec![] };
        t.mix(&ev, &mut l, &mut r);
        out.extend_from_slice(&l);
        std::thread::sleep(std::time::Duration::from_micros(10_600));
    }
    out
}
fn main() {
    let mut t = Tearout::default();
    std::thread::sleep(std::time::Duration::from_secs(3));
    for n in [60u8, 60, 62, 62] {
        let x = hit(&mut t, n);
        let pk = x.iter().fold(0f32, |a, &b| a.max(b.abs()));
        println!("note {n}: lo/hi {:?} pk {pk:.2}", lo_hi(&x));
    }
}
