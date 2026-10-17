// Renders written as 24-bit stereo WAV files under ~/Music/Tearout Farm, named by preset, layer and patch.
use std::io::Write;
use std::path::PathBuf;

pub fn dir() -> PathBuf {
    let home = std::env::var_os("HOME").map(PathBuf::from).unwrap_or_else(std::env::temp_dir);
    home.join("Music").join("Tearout Farm")
}

fn clean(s: &str) -> String {
    s.chars().map(|c| if c.is_alphanumeric() || c == ' ' || c == '-' || c == '_' { c } else { '_' }).collect::<String>().trim().to_string()
}

pub fn write_wav(path: &std::path::Path, l: &[f32], r: &[f32], sr: u32) -> std::io::Result<()> {
    let n = l.len().min(r.len());
    let data = (n * 6) as u32;
    let mut b = Vec::with_capacity(44 + data as usize);
    b.extend_from_slice(b"RIFF");
    b.extend_from_slice(&(36 + data).to_le_bytes());
    b.extend_from_slice(b"WAVEfmt ");
    b.extend_from_slice(&16u32.to_le_bytes());
    b.extend_from_slice(&1u16.to_le_bytes());
    b.extend_from_slice(&2u16.to_le_bytes());
    b.extend_from_slice(&sr.to_le_bytes());
    b.extend_from_slice(&(sr * 6).to_le_bytes());
    b.extend_from_slice(&6u16.to_le_bytes());
    b.extend_from_slice(&24u16.to_le_bytes());
    b.extend_from_slice(b"data");
    b.extend_from_slice(&data.to_le_bytes());
    for i in 0..n {
        for s in [l[i], r[i]] {
            let v = (s.clamp(-1.0, 1.0) as f64 * 8_388_607.0).round() as i32;
            b.extend_from_slice(&v.to_le_bytes()[..3]);
        }
    }
    let tmp = path.with_extension("wav.part");
    std::fs::File::create(&tmp)?.write_all(&b)?;
    std::fs::rename(tmp, path)
}

/// Write one lane's audio (reusing the file when this exact patch was written before) and return its path.
pub fn lane_file(preset: &str, lane: &str, patch: u64, l: &[f32], r: &[f32], sr: u32) -> std::io::Result<PathBuf> {
    let d = dir();
    std::fs::create_dir_all(&d)?;
    let name = format!("{} {} {:06x}.wav", clean(if preset.is_empty() { "tearout" } else { preset }), lane, patch & 0xff_ffff);
    let p = d.join(name);
    if !p.exists() { write_wav(&p, l, r, sr)?; }
    Ok(p)
}
