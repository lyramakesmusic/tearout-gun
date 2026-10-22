mod renderer;
mod translate;
mod window;

pub use window::{EguiWindow, Queue};

pub use egui;
pub use renderer::GraphicsConfig;

/// Keys the editor doesn't use go on to the host unless this is set (a standalone app has no host to take
/// them, and macOS beeps at keys nobody handles).
pub static KEEP_KEYS: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
