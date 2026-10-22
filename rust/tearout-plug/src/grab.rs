// Pinned cursor for knob drags: the cursor hides while the drag reads its raw movement, and is put back
// where the drag began whenever it wanders far, so a drag never runs into the edge of the screen.
use std::sync::Mutex;

#[cfg(target_os = "macos")]
use crate::macos as os;
#[cfg(windows)]
use crate::win as os;

struct Grab {
    anchor: (f64, f64),
    prev: (f64, f64),
    /// where the cursor was when it was last put back; a reading still there predates the move
    stale: Option<(f64, f64)>,
}

static GRAB: Mutex<Option<Grab>> = Mutex::new(None);

/// How far the hidden cursor travels before it is put back.
const LEASH: f64 = 120.0;

pub fn begin() {
    #[cfg(any(target_os = "macos", windows))]
    {
        let mut g = GRAB.lock().unwrap();
        if g.is_none() {
            let c = os::cursor();
            *g = Some(Grab { anchor: c, prev: c, stale: None });
            os::show_cursor(false);
        }
    }
}

/// Movement since the last call, in screen units. `None` where the cursor can't be pinned.
pub fn delta() -> Option<(f64, f64)> {
    #[cfg(any(target_os = "macos", windows))]
    {
        let mut g = GRAB.lock().unwrap();
        let g = g.as_mut()?;
        let c = os::cursor();
        if let Some(s) = g.stale {
            if (c.0 - s.0).abs() < 1.0 && (c.1 - s.1).abs() < 1.0 { return Some((0.0, 0.0)); }
            g.stale = None;
        }
        let d = (c.0 - g.prev.0, c.1 - g.prev.1);
        g.prev = c;
        if (c.0 - g.anchor.0).hypot(c.1 - g.anchor.1) > LEASH {
            os::warp(g.anchor);
            g.stale = Some(c);
            g.prev = g.anchor;
        }
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
    if let Some(g) = GRAB.lock().unwrap().take() { os::warp(g.anchor); os::show_cursor(true); }
}
