// Windows glue: native file drags out of the editor, and the hidden, pinned cursor for knob drags.
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use windows_sys::core::GUID;
use windows_sys::Win32::Foundation::{HWND, POINT};
use windows_sys::Win32::Graphics::Gdi::ScreenToClient;
use windows_sys::Win32::Foundation::{BOOL, DRAGDROP_S_CANCEL, DRAGDROP_S_DROP, DRAGDROP_S_USEDEFAULTCURSORS, E_NOINTERFACE, S_OK};
use windows_sys::Win32::Foundation::GlobalFree;
use windows_sys::Win32::System::Memory::{GlobalAlloc, GlobalLock, GlobalSize, GlobalUnlock, GMEM_MOVEABLE, GMEM_ZEROINIT};
use windows_sys::Win32::System::Ole::{DoDragDrop, OleInitialize, OleUninitialize, DROPEFFECT_COPY};
use windows_sys::Win32::System::Threading::{AttachThreadInput, GetCurrentThreadId};
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, GetCapture, VK_LBUTTON};
use windows_sys::Win32::UI::WindowsAndMessaging::{GetCursorPos, PostMessageW, SetCursorPos, ShowCursor, WM_LBUTTONUP};

const IID_IDATAOBJECT: GUID = GUID { data1: 0x0000010e, data2: 0, data3: 0, data4: [0xc0, 0, 0, 0, 0, 0, 0, 0x46] };
const IID_IENUMFORMATETC: GUID = GUID { data1: 0x00000103, data2: 0, data3: 0, data4: [0xc0, 0, 0, 0, 0, 0, 0, 0x46] };

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

type Ptr = *mut core::ffi::c_void;
const CF_HDROP: u16 = 15;
const TYMED_HGLOBAL: u32 = 1;
const DVASPECT_CONTENT: u32 = 1;
const DATADIR_GET: u32 = 1;
const S_FALSE: i32 = 1;
const E_NOTIMPL: i32 = 0x8000_4001u32 as i32;
const E_POINTER: i32 = 0x8000_4003u32 as i32;
const DV_E_FORMATETC: i32 = 0x8004_0064u32 as i32;
const OLE_E_ADVISENOTSUPPORTED: i32 = 0x8004_0003u32 as i32;

#[repr(C)]
#[derive(Clone, Copy)]
struct FormatEtc { cf: u16, ptd: Ptr, aspect: u32, lindex: i32, tymed: u32 }
#[repr(C)]
struct StgMedium { tymed: u32, data: Ptr, unk_for_release: Ptr }

const HDROP_FORMAT: FormatEtc = FormatEtc { cf: CF_HDROP, ptd: std::ptr::null_mut(), aspect: DVASPECT_CONTENT, lindex: -1, tymed: TYMED_HGLOBAL };

/// The dragged files as the drop target reads them: one CF_HDROP list of paths, the only format offered, the
/// way JUCE plugins hand files to a DAW.
#[repr(C)]
struct DataObject { vtbl: *const DataObjectVtbl, refs: std::sync::atomic::AtomicU32, hdrop: Ptr }

#[repr(C)]
struct DataObjectVtbl {
    query_interface: unsafe extern "system" fn(*mut DataObject, *const GUID, *mut Ptr) -> i32,
    add_ref: unsafe extern "system" fn(*mut DataObject) -> u32,
    release: unsafe extern "system" fn(*mut DataObject) -> u32,
    get_data: unsafe extern "system" fn(*mut DataObject, *const FormatEtc, *mut StgMedium) -> i32,
    get_data_here: unsafe extern "system" fn(*mut DataObject, *const FormatEtc, *mut StgMedium) -> i32,
    query_get_data: unsafe extern "system" fn(*mut DataObject, *const FormatEtc) -> i32,
    get_canonical_format_etc: unsafe extern "system" fn(*mut DataObject, *const FormatEtc, *mut FormatEtc) -> i32,
    set_data: unsafe extern "system" fn(*mut DataObject, *const FormatEtc, *mut StgMedium, BOOL) -> i32,
    enum_format_etc: unsafe extern "system" fn(*mut DataObject, u32, *mut Ptr) -> i32,
    d_advise: unsafe extern "system" fn(*mut DataObject, *const FormatEtc, u32, Ptr, *mut u32) -> i32,
    d_unadvise: unsafe extern "system" fn(*mut DataObject, u32) -> i32,
    enum_d_advise: unsafe extern "system" fn(*mut DataObject, *mut Ptr) -> i32,
}

static DATA_OBJECT_VTBL: DataObjectVtbl = DataObjectVtbl {
    query_interface: do_query, add_ref: do_add_ref, release: do_release, get_data: do_get_data, get_data_here: do_get_data_here,
    query_get_data: do_query_get_data, get_canonical_format_etc: do_canonical, set_data: do_set_data, enum_format_etc: do_enum,
    d_advise: do_d_advise, d_unadvise: do_d_unadvise, enum_d_advise: do_enum_d_advise,
};

fn is_hdrop(f: &FormatEtc) -> bool { f.cf == CF_HDROP && f.tymed & TYMED_HGLOBAL != 0 && f.aspect == DVASPECT_CONTENT }

unsafe extern "system" fn do_query(this: *mut DataObject, iid: *const GUID, out: *mut Ptr) -> i32 {
    if same_guid(&*iid, &IID_IUNKNOWN) || same_guid(&*iid, &IID_IDATAOBJECT) { do_add_ref(this); *out = this as Ptr; S_OK } else { *out = std::ptr::null_mut(); E_NOINTERFACE }
}
unsafe extern "system" fn do_add_ref(this: *mut DataObject) -> u32 { (*this).refs.fetch_add(1, Ordering::AcqRel) + 1 }
unsafe extern "system" fn do_release(this: *mut DataObject) -> u32 {
    let left = (*this).refs.fetch_sub(1, Ordering::AcqRel) - 1;
    if left == 0 { GlobalFree((*this).hdrop); drop(Box::from_raw(this)); }
    left
}
unsafe extern "system" fn do_get_data(this: *mut DataObject, f: *const FormatEtc, m: *mut StgMedium) -> i32 {
    if f.is_null() || m.is_null() { return E_POINTER; }
    if !is_hdrop(&*f) { return DV_E_FORMATETC; }
    // each reader gets its own copy, which it frees
    let src = (*this).hdrop;
    let len = GlobalSize(src);
    let dst = GlobalAlloc(GMEM_MOVEABLE, len);
    let from = GlobalLock(src);
    let to = if dst.is_null() { std::ptr::null_mut() } else { GlobalLock(dst) };
    if to.is_null() || from.is_null() { GlobalUnlock(src); if !dst.is_null() { GlobalFree(dst); } return DV_E_FORMATETC; }
    std::ptr::copy_nonoverlapping(from as *const u8, to as *mut u8, len);
    GlobalUnlock(dst);
    GlobalUnlock(src);
    *m = StgMedium { tymed: TYMED_HGLOBAL, data: dst, unk_for_release: std::ptr::null_mut() };
    S_OK
}
unsafe extern "system" fn do_get_data_here(_: *mut DataObject, _: *const FormatEtc, _: *mut StgMedium) -> i32 { DV_E_FORMATETC }
unsafe extern "system" fn do_query_get_data(_: *mut DataObject, f: *const FormatEtc) -> i32 { if !f.is_null() && is_hdrop(&*f) { S_OK } else { DV_E_FORMATETC } }
unsafe extern "system" fn do_canonical(_: *mut DataObject, _: *const FormatEtc, out: *mut FormatEtc) -> i32 { if !out.is_null() { (*out).ptd = std::ptr::null_mut(); } E_NOTIMPL }
unsafe extern "system" fn do_set_data(_: *mut DataObject, _: *const FormatEtc, _: *mut StgMedium, _: BOOL) -> i32 { E_NOTIMPL }
unsafe extern "system" fn do_enum(_: *mut DataObject, dir: u32, out: *mut Ptr) -> i32 {
    if out.is_null() { return E_POINTER; }
    if dir != DATADIR_GET { *out = std::ptr::null_mut(); return E_NOTIMPL; }
    *out = Box::into_raw(Box::new(EnumFormats { vtbl: &ENUM_VTBL, refs: std::sync::atomic::AtomicU32::new(1), index: 0 })) as Ptr;
    S_OK
}
unsafe extern "system" fn do_d_advise(_: *mut DataObject, _: *const FormatEtc, _: u32, _: Ptr, _: *mut u32) -> i32 { OLE_E_ADVISENOTSUPPORTED }
unsafe extern "system" fn do_d_unadvise(_: *mut DataObject, _: u32) -> i32 { E_NOTIMPL }
unsafe extern "system" fn do_enum_d_advise(_: *mut DataObject, _: *mut Ptr) -> i32 { OLE_E_ADVISENOTSUPPORTED }

/// Lists the one format the data object offers.
#[repr(C)]
struct EnumFormats { vtbl: *const EnumVtbl, refs: std::sync::atomic::AtomicU32, index: u32 }

#[repr(C)]
struct EnumVtbl {
    query_interface: unsafe extern "system" fn(*mut EnumFormats, *const GUID, *mut Ptr) -> i32,
    add_ref: unsafe extern "system" fn(*mut EnumFormats) -> u32,
    release: unsafe extern "system" fn(*mut EnumFormats) -> u32,
    next: unsafe extern "system" fn(*mut EnumFormats, u32, *mut FormatEtc, *mut u32) -> i32,
    skip: unsafe extern "system" fn(*mut EnumFormats, u32) -> i32,
    reset: unsafe extern "system" fn(*mut EnumFormats) -> i32,
    clone: unsafe extern "system" fn(*mut EnumFormats, *mut Ptr) -> i32,
}

static ENUM_VTBL: EnumVtbl = EnumVtbl { query_interface: ef_query, add_ref: ef_add_ref, release: ef_release, next: ef_next, skip: ef_skip, reset: ef_reset, clone: ef_clone };

unsafe extern "system" fn ef_query(this: *mut EnumFormats, iid: *const GUID, out: *mut Ptr) -> i32 {
    if same_guid(&*iid, &IID_IUNKNOWN) || same_guid(&*iid, &IID_IENUMFORMATETC) { ef_add_ref(this); *out = this as Ptr; S_OK } else { *out = std::ptr::null_mut(); E_NOINTERFACE }
}
unsafe extern "system" fn ef_add_ref(this: *mut EnumFormats) -> u32 { (*this).refs.fetch_add(1, Ordering::AcqRel) + 1 }
unsafe extern "system" fn ef_release(this: *mut EnumFormats) -> u32 {
    let left = (*this).refs.fetch_sub(1, Ordering::AcqRel) - 1;
    if left == 0 { drop(Box::from_raw(this)); }
    left
}
unsafe extern "system" fn ef_next(this: *mut EnumFormats, n: u32, out: *mut FormatEtc, fetched: *mut u32) -> i32 {
    if !fetched.is_null() { *fetched = 0; } else if n != 1 { return S_FALSE; }
    if (*this).index == 0 && n > 0 && !out.is_null() {
        *out = HDROP_FORMAT;
        (*this).index = 1;
        if !fetched.is_null() { *fetched = 1; }
        return if n == 1 { S_OK } else { S_FALSE };
    }
    S_FALSE
}
unsafe extern "system" fn ef_skip(this: *mut EnumFormats, n: u32) -> i32 { if (*this).index + n <= 1 { (*this).index += n; S_OK } else { (*this).index = 1; S_FALSE } }
unsafe extern "system" fn ef_reset(this: *mut EnumFormats) -> i32 { (*this).index = 0; S_OK }
unsafe extern "system" fn ef_clone(this: *mut EnumFormats, out: *mut Ptr) -> i32 {
    if out.is_null() { return E_POINTER; }
    *out = Box::into_raw(Box::new(EnumFormats { vtbl: &ENUM_VTBL, refs: std::sync::atomic::AtomicU32::new(1), index: (*this).index })) as Ptr;
    S_OK
}

/// A DROPFILES block listing `paths`, as CF_HDROP carries it.
unsafe fn hdrop(paths: &[PathBuf]) -> Ptr {
    let names: Vec<u16> = paths.iter().flat_map(|p| wide(p)).chain(Some(0)).collect();
    let head = 20usize; // DROPFILES: pFiles, pt (x, y), fNC, fWide
    let mem = GlobalAlloc(GMEM_MOVEABLE | GMEM_ZEROINIT, head + names.len() * 2);
    if mem.is_null() { return mem; }
    let at = GlobalLock(mem) as *mut u8;
    if at.is_null() { GlobalFree(mem); return std::ptr::null_mut(); }
    *(at as *mut u32) = head as u32;
    *(at.add(16) as *mut i32) = 1;
    std::ptr::copy_nonoverlapping(names.as_ptr() as *const u8, at.add(head), names.len() * 2);
    GlobalUnlock(mem);
    mem
}

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

unsafe fn run(paths: &[PathBuf], owner: HWND, ui: u32) {
    let t0 = std::time::Instant::now();
    let ms = |t: std::time::Instant| t.elapsed().as_secs_f64() * 1000.0;
    let ole = OleInitialize(std::ptr::null_mut()) >= 0;
    let files = hdrop(paths);
    if !files.is_null() {
        let obj = Box::into_raw(Box::new(DataObject { vtbl: &DATA_OBJECT_VTBL, refs: std::sync::atomic::AtomicU32::new(1), hdrop: files }));
        let mut effect = 0;
        let mut src = DropSource { vtbl: &DROP_SOURCE_VTBL };
        // sharing the UI thread's input state for the drag itself gives it the held button and the mouse capture
        let me = GetCurrentThreadId();
        let attached = AttachThreadInput(me, ui, 1) != 0;
        crate::export::log(&format!("drag: ready after {:.0} ms", ms(t0)));
        let t1 = std::time::Instant::now();
        let hr = DoDragDrop(obj as Ptr, &mut src as *mut DropSource as Ptr, DROPEFFECT_COPY, &mut effect);
        if attached { AttachThreadInput(me, ui, 0); }
        crate::export::log(&format!("drag: held {:.0} ms, dropped (effect {effect}, hr {hr:#x})", ms(t1)));
        do_release(obj);
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
