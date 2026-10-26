// Windows shell glue: native file drags out of the editor, and the hidden, pinned cursor for knob drags.
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use windows_sys::core::GUID;
use windows_sys::Win32::Foundation::{HWND, POINT};
use windows_sys::Win32::Graphics::Gdi::ScreenToClient;
use windows_sys::Win32::Foundation::{BOOL, DRAGDROP_S_CANCEL, DRAGDROP_S_DROP, DRAGDROP_S_USEDEFAULTCURSORS, E_NOINTERFACE, S_OK};
use windows_sys::Win32::System::Ole::{DoDragDrop, OleInitialize, OleUninitialize, DROPEFFECT_COPY};
use windows_sys::Win32::System::Threading::{AttachThreadInput, GetCurrentThreadId};
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, GetCapture, VK_LBUTTON};
use windows_sys::Win32::UI::Shell::Common::ITEMIDLIST;
use windows_sys::Win32::UI::Shell::{ILCreateFromPathW, ILFindLastID, ILFree, SHCreateDataObject};
use windows_sys::Win32::UI::WindowsAndMessaging::{GetCursorPos, PostMessageW, SetCursorPos, ShowCursor, WM_LBUTTONUP};

const IID_IDATAOBJECT: GUID = GUID { data1: 0x0000010e, data2: 0, data3: 0, data4: [0xc0, 0, 0, 0, 0, 0, 0, 0x46] };

const IID_IUNKNOWN: GUID = GUID { data1: 0, data2: 0, data3: 0, data4: [0xc0, 0, 0, 0, 0, 0, 0, 0x46] };
const IID_IDROPSOURCE: GUID = GUID { data1: 0x00000121, data2: 0, data3: 0, data4: [0xc0, 0, 0, 0, 0, 0, 0, 0x46] };

/// The drag's IDropSource. It reads the physical button, since the drag thread's own key state never saw
/// the press. It lives on the drag thread's stack for the length of `DoDragDrop`.
#[repr(C)]
struct DropSource { vtbl: *const DropSourceVtbl }

#[repr(C)]
struct DropSourceVtbl {
    query_interface: unsafe extern "system" fn(*mut DropSource, *const GUID, *mut *mut core::ffi::c_void) -> i32,
    add_ref: unsafe extern "system" fn(*mut DropSource) -> u32,
    release: unsafe extern "system" fn(*mut DropSource) -> u32,
    query_continue_drag: unsafe extern "system" fn(*mut DropSource, BOOL, u32) -> i32,
    give_feedback: unsafe extern "system" fn(*mut DropSource, u32) -> i32,
}

static DROP_SOURCE_VTBL: DropSourceVtbl = DropSourceVtbl { query_interface: ds_query, add_ref: ds_ref, release: ds_ref, query_continue_drag: ds_continue, give_feedback: ds_feedback };

fn same_guid(a: &GUID, b: &GUID) -> bool { a.data1 == b.data1 && a.data2 == b.data2 && a.data3 == b.data3 && a.data4 == b.data4 }

unsafe extern "system" fn ds_query(this: *mut DropSource, iid: *const GUID, out: *mut *mut core::ffi::c_void) -> i32 {
    if same_guid(&*iid, &IID_IUNKNOWN) || same_guid(&*iid, &IID_IDROPSOURCE) { *out = this as *mut core::ffi::c_void; S_OK } else { *out = std::ptr::null_mut(); E_NOINTERFACE }
}
unsafe extern "system" fn ds_ref(_: *mut DropSource) -> u32 { 1 }
unsafe extern "system" fn ds_continue(_: *mut DropSource, escape: BOOL, _keys: u32) -> i32 {
    if escape != 0 { DRAGDROP_S_CANCEL } else if GetAsyncKeyState(VK_LBUTTON as i32) as u16 & 0x8000 == 0 { DRAGDROP_S_DROP } else { S_OK }
}
unsafe extern "system" fn ds_feedback(_: *mut DropSource, _: u32) -> i32 { DRAGDROP_S_USEDEFAULTCURSORS }

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
    let owner = unsafe { GetCapture() } as isize;
    let ui = unsafe { GetCurrentThreadId() };
    let paths = paths.to_vec();
    let spawned = std::thread::Builder::new().name("tearout-drag".into()).spawn(move || {
        unsafe { run(&paths, owner as HWND, ui) };
        DRAGGING.store(false, Ordering::Release);
    });
    if spawned.is_err() { DRAGGING.store(false, Ordering::Release); }
    spawned.is_ok()
}

/// Load the shell's drag-and-drop machinery once in the background, so the first drag starts at once.
pub fn warm_shell() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let _ = std::thread::Builder::new().name("tearout-shell".into()).spawn(|| unsafe {
            let t0 = std::time::Instant::now();
            struct Done(std::time::Instant);
            impl Drop for Done { fn drop(&mut self) { crate::export::log(&format!("shell warm-up took {:.0} ms", self.0.elapsed().as_secs_f64() * 1000.0)); } }
            let _done = Done(t0);
            let dir = crate::export::dir();
            let _ = std::fs::create_dir_all(&dir);
            let Some(parent) = dir.parent() else { return };
            let ole = OleInitialize(std::ptr::null_mut()) >= 0;
            let folder = ILCreateFromPathW(wide(parent).as_ptr());
            let item = ILCreateFromPathW(wide(&dir).as_ptr());
            if !folder.is_null() && !item.is_null() {
                let kids = [ILFindLastID(item) as *const ITEMIDLIST];
                let mut obj: *mut core::ffi::c_void = std::ptr::null_mut();
                if SHCreateDataObject(folder, 1, kids.as_ptr(), std::ptr::null_mut(), &IID_IDATAOBJECT, &mut obj) >= 0 && !obj.is_null() {
                    let vt = *(obj as *const *const [usize; 3]);
                    let release: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32 = std::mem::transmute((*vt)[2]);
                    release(obj);
                }
            }
            if !item.is_null() { ILFree(item); }
            if !folder.is_null() { ILFree(folder); }
            if ole { OleUninitialize(); }
        });
    });
}

unsafe fn run(paths: &[PathBuf], owner: HWND, ui: u32) {
    let Some(dir) = paths[0].parent() else { return };
    let t0 = std::time::Instant::now();
    let ms = |t: std::time::Instant| t.elapsed().as_secs_f64() * 1000.0;
    let ole = OleInitialize(std::ptr::null_mut()) >= 0;
    let folder = ILCreateFromPathW(wide(dir).as_ptr());
    if !folder.is_null() {
        let full: Vec<*mut ITEMIDLIST> = paths.iter().map(|p| ILCreateFromPathW(wide(p).as_ptr())).filter(|p| !p.is_null()).collect();
        let kids: Vec<*const ITEMIDLIST> = full.iter().map(|&p| ILFindLastID(p) as *const ITEMIDLIST).collect();
        let mut obj: *mut core::ffi::c_void = std::ptr::null_mut();
        if !kids.is_empty() && SHCreateDataObject(folder, kids.len() as u32, kids.as_ptr(), std::ptr::null_mut(), &IID_IDATAOBJECT, &mut obj) >= 0 && !obj.is_null() {
            let mut effect = 0;
            let mut src = DropSource { vtbl: &DROP_SOURCE_VTBL };
            // sharing the UI thread's input state for the drag itself gives it the held button and the mouse capture
            let me = GetCurrentThreadId();
            let attached = AttachThreadInput(me, ui, 1) != 0;
            crate::export::log(&format!("drag: shell ready after {:.0} ms", ms(t0)));
            let t1 = std::time::Instant::now();
            let hr = DoDragDrop(obj, &mut src as *mut DropSource as *mut core::ffi::c_void, DROPEFFECT_COPY, &mut effect);
            if attached { AttachThreadInput(me, ui, 0); }
            crate::export::log(&format!("drag: held {:.0} ms, dropped (effect {effect}, hr {hr:#x})", ms(t1)));
            nih_plug::nih_log!("drag out ended: hr {hr:#x}, effect {effect}");
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
