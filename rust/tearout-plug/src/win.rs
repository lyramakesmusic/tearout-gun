// Windows shell glue: native file drags out of the editor, and the hidden, pinned cursor for knob drags.
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use windows_sys::core::GUID;
use windows_sys::Win32::Foundation::{HWND, POINT};
use windows_sys::Win32::Graphics::Gdi::ScreenToClient;
use windows_sys::Win32::System::Ole::{OleInitialize, OleUninitialize, DROPEFFECT_COPY};
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{GetCapture, ReleaseCapture};
use windows_sys::Win32::UI::Shell::Common::ITEMIDLIST;
use windows_sys::Win32::UI::Shell::{ILCreateFromPathW, ILFindLastID, ILFree, SHCreateDataObject, SHDoDragDrop};
use windows_sys::Win32::UI::WindowsAndMessaging::{GetCursorPos, PostMessageW, SetCursorPos, ShowCursor, WM_LBUTTONUP};

const IID_IDATAOBJECT: GUID = GUID { data1: 0x0000010e, data2: 0, data3: 0, data4: [0xc0, 0, 0, 0, 0, 0, 0, 0x46] };

/// A drag is in flight; one runs at a time.
static DRAGGING: AtomicBool = AtomicBool::new(false);

fn wide(p: &std::path::Path) -> Vec<u16> {
    use std::os::windows::ffi::OsStrExt;
    p.as_os_str().encode_wide().chain(Some(0)).collect()
}

/// Drag `paths` (all in one folder) out of the editor. The drag's modal loop runs on its own thread, so the
/// host's UI thread stays free to take the drop.
pub fn drag_files(paths: &[PathBuf]) -> bool {
    if paths.is_empty() || DRAGGING.swap(true, Ordering::AcqRel) { return false; }
    // the drag thread takes the mouse from here on
    let owner = unsafe { let o = GetCapture(); ReleaseCapture(); o } as isize;
    let paths = paths.to_vec();
    let spawned = std::thread::Builder::new().name("tearout-drag".into()).spawn(move || {
        unsafe { run(&paths, owner as HWND) };
        DRAGGING.store(false, Ordering::Release);
    });
    if spawned.is_err() { DRAGGING.store(false, Ordering::Release); }
    spawned.is_ok()
}

unsafe fn run(paths: &[PathBuf], owner: HWND) {
    let Some(dir) = paths[0].parent() else { return };
    let ole = OleInitialize(std::ptr::null_mut()) >= 0;
    let folder = ILCreateFromPathW(wide(dir).as_ptr());
    if !folder.is_null() {
        let full: Vec<*mut ITEMIDLIST> = paths.iter().map(|p| ILCreateFromPathW(wide(p).as_ptr())).filter(|p| !p.is_null()).collect();
        let kids: Vec<*const ITEMIDLIST> = full.iter().map(|&p| ILFindLastID(p) as *const ITEMIDLIST).collect();
        let mut obj: *mut core::ffi::c_void = std::ptr::null_mut();
        if !kids.is_empty() && SHCreateDataObject(folder, kids.len() as u32, kids.as_ptr(), std::ptr::null_mut(), &IID_IDATAOBJECT, &mut obj) >= 0 && !obj.is_null() {
            let mut effect = 0;
            SHDoDragDrop(std::ptr::null_mut(), obj, std::ptr::null_mut(), DROPEFFECT_COPY, &mut effect);
            // IUnknown::Release
            let vt = *(obj as *const *const [usize; 3]);
            let release: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32 = std::mem::transmute((*vt)[2]);
            release(obj);
        }
        for p in full { ILFree(p); }
        ILFree(folder);
    }
    if ole { OleUninitialize(); }
    // the drag took the button release; hand the editor one so it stops dragging too
    if !owner.is_null() {
        let mut pt = POINT { x: 0, y: 0 };
        GetCursorPos(&mut pt);
        ScreenToClient(owner, &mut pt);
        PostMessageW(owner, WM_LBUTTONUP, 0, ((pt.y as u16 as isize) << 16) | pt.x as u16 as isize);
    }
}

/// Screen position of the cursor, in physical pixels.
pub fn cursor() -> (f64, f64) {
    let mut pt = POINT { x: 0, y: 0 };
    unsafe { GetCursorPos(&mut pt) };
    (pt.x as f64, pt.y as f64)
}
pub fn warp(p: (f64, f64)) { unsafe { SetCursorPos(p.0 as i32, p.1 as i32) }; }
pub fn show_cursor(on: bool) { unsafe { ShowCursor(on as i32) }; }
