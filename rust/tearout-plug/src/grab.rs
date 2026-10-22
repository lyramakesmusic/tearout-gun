// Pinned cursor for knob drags: the cursor hides and stays put while the drag reads raw movement, so a
// drag never runs into the edge of the screen.
use std::sync::Mutex;

static ANCHOR: Mutex<Option<(f64, f64)>> = Mutex::new(None);

#[cfg(target_os = "macos")]
use crate::macos as os;
#[cfg(windows)]
use crate::win as os;

pub fn begin() {
    #[cfg(any(target_os = "macos", windows))]
    {
        let mut a = ANCHOR.lock().unwrap();
        if a.is_none() { *a = Some(os::cursor()); os::show_cursor(false); }
    }
}

/// Movement since the last call, in screen units, with the cursor put back where the drag began.
/// `None` where the cursor can't be pinned.
pub fn delta() -> Option<(f64, f64)> {
    #[cfg(any(target_os = "macos", windows))]
    {
        let a = (*ANCHOR.lock().unwrap())?;
        let c = os::cursor();
        let d = (c.0 - a.0, c.1 - a.1);
        if d != (0.0, 0.0) { os::warp(a); }
        return Some(d);
    }
    #[allow(unreachable_code)]
    None
}

/// Screen position of the cursor where it can be read: points on mac, pixels on Windows.
pub fn screen_cursor() -> Option<(f64, f64)> {
    #[cfg(any(target_os = "macos", windows))]
    return Some(os::cursor());
    #[allow(unreachable_code)]
    None
}

pub fn end() {
    #[cfg(any(target_os = "macos", windows))]
    if let Some(a) = ANCHOR.lock().unwrap().take() { os::warp(a); os::show_cursor(true); }
}
