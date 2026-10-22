// The Tearout Farm folder (saved presets, dragged-out stems) and the stems written into it as 24-bit stereo
// WAV files, named by preset, layer, note and patch.
use std::io::Write;
use std::path::PathBuf;
use std::sync::RwLock;

/// The app-data home: Application Support on macOS, %APPDATA% on Windows, the XDG data folder elsewhere.
fn app_data() -> PathBuf {
    let env = |k: &str| std::env::var_os(k).map(PathBuf::from);
    let home = env("HOME").unwrap_or_else(std::env::temp_dir);
    if cfg!(target_os = "macos") { home.join("Library").join("Application Support").join("Tearout Farm") }
    else if cfg!(windows) { env("APPDATA").unwrap_or_else(std::env::temp_dir).join("Tearout Farm") }
    else { env("XDG_DATA_HOME").unwrap_or_else(|| home.join(".local").join("share")).join("tearout-farm") }
}

fn settings_file() -> PathBuf { app_data().join("settings.json") }

static ROOT: RwLock<Option<PathBuf>> = RwLock::new(None);

/// The Tearout Farm folder: the one chosen in the preset menu, else the app-data home.
pub fn root() -> PathBuf {
    if let Some(p) = ROOT.read().ok().and_then(|r| r.clone()) { return p; }
    let chosen = std::fs::read_to_string(settings_file()).ok().and_then(|s| serde_json::from_str::<serde_json::Value>(&s).ok()).and_then(|j| j["folder"].as_str().map(PathBuf::from));
    let p = chosen.unwrap_or_else(app_data);
    if let Ok(mut r) = ROOT.write() { *r = Some(p.clone()); }
    p
}

/// Make `p` the Tearout Farm folder, remembered across sessions.
pub fn set_root(p: PathBuf) {
    let _ = std::fs::create_dir_all(app_data());
    let _ = std::fs::write(settings_file(), serde_json::json!({ "folder": p.to_string_lossy() }).to_string());
    if let Ok(mut r) = ROOT.write() { *r = Some(p); }
}

/// Dragged-out stems.
pub fn dir() -> PathBuf { root().join("Stems") }

/// Show a folder in the file manager.
pub fn reveal(p: &std::path::Path) {
    let _ = std::fs::create_dir_all(p);
    let opener = if cfg!(target_os = "macos") { "open" } else if cfg!(windows) { "explorer" } else { "xdg-open" };
    let _ = std::process::Command::new(opener).arg(p).spawn();
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
pub fn lane_file(preset: &str, lane: &str, note: &str, patch: u64, l: &[f32], r: &[f32], sr: u32) -> std::io::Result<PathBuf> {
    let d = dir();
    std::fs::create_dir_all(&d)?;
    let name = format!("gun-{} {} {} {:06x}.wav", clean(if preset.is_empty() { "tearout" } else { preset }), lane, clean(note), patch & 0xff_ffff);
    let p = d.join(name);
    if !p.exists() { write_wav(&p, l, r, sr)?; }
    Ok(p)
}
