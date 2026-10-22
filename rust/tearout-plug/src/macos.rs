// Native file drag: starts an AppKit dragging session for files on disk from the view under the pointer, so
// a render can be dropped onto a DAW track or the Finder.
#![allow(unexpected_cfgs)]
use cocoa::base::{id, nil, NO};
use cocoa::foundation::{NSArray, NSPoint, NSRect, NSSize, NSString};
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Protocol, Sel};
use objc::{class, msg_send, sel, sel_impl};
use std::sync::Once;

const NS_DRAG_OP_COPY: usize = 1;
const NS_EVENT_LEFT_DRAGGED: usize = 6;

extern "C" fn op_mask(_: &Object, _: Sel, _session: id, _ctx: isize) -> usize { NS_DRAG_OP_COPY }

fn source_class() -> &'static Class {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        let mut decl = ClassDecl::new("TearoutDragSource", class!(NSObject)).expect("drag source class");
        if let Some(p) = Protocol::get("NSDraggingSource") { decl.add_protocol(p); }
        unsafe {
            decl.add_method(sel!(draggingSession:sourceOperationMaskForDraggingContext:), op_mask as extern "C" fn(&Object, Sel, id, isize) -> usize);
        }
        decl.register();
    });
    Class::get("TearoutDragSource").unwrap()
}

/// Begin dragging `paths` from wherever the pointer is. Call while the mouse button is held.
pub fn drag_files(paths: &[std::path::PathBuf]) -> bool {
    unsafe {
        let mouse: NSPoint = msg_send![class!(NSEvent), mouseLocation];
        let num: isize = msg_send![class!(NSWindow), windowNumberAtPoint: mouse belowWindowWithWindowNumber: 0isize];
        let app: id = msg_send![class!(NSApplication), sharedApplication];
        let win: id = msg_send![app, windowWithWindowNumber: num];
        if win == nil { return false; }
        let content: id = msg_send![win, contentView];
        let in_win: NSPoint = msg_send![win, convertPointFromScreen: mouse];
        let sup: id = msg_send![content, superview];
        let probe = if sup == nil { in_win } else { msg_send![sup, convertPoint: in_win fromView: nil] };
        let view: id = msg_send![content, hitTest: probe];
        let view = if view == nil { content } else { view };
        let at: NSPoint = msg_send![view, convertPoint: in_win fromView: nil];

        let ws: id = msg_send![class!(NSWorkspace), sharedWorkspace];
        let mut items: Vec<id> = Vec::new();
        for (k, p) in paths.iter().enumerate() {
            let s = NSString::alloc(nil).init_str(&p.to_string_lossy());
            let url: id = msg_send![class!(NSURL), fileURLWithPath: s];
            let item: id = msg_send![class!(NSDraggingItem), alloc];
            let item: id = msg_send![item, initWithPasteboardWriter: url];
            let icon: id = msg_send![ws, iconForFile: s];
            let o = k as f64 * 6.0;
            let frame = NSRect::new(NSPoint::new(at.x - 16.0 + o, at.y - 16.0 - o), NSSize::new(32.0, 32.0));
            let _: () = msg_send![item, setDraggingFrame: frame contents: icon];
            let _: id = msg_send![item, autorelease];
            items.push(item);
        }
        let arr = NSArray::arrayWithObjects(nil, &items);
        let wnum: isize = msg_send![win, windowNumber];
        let pi: id = msg_send![class!(NSProcessInfo), processInfo];
        let up: f64 = msg_send![pi, systemUptime];
        let ev: id = msg_send![class!(NSEvent), mouseEventWithType: NS_EVENT_LEFT_DRAGGED location: in_win modifierFlags: 0usize timestamp: up windowNumber: wnum context: nil eventNumber: 0isize clickCount: 1isize pressure: 1.0f32];
        if ev == nil { return false; }
        let src: id = msg_send![source_class(), new];
        let session: id = msg_send![view, beginDraggingSessionWithItems: arr event: ev source: src];
        let _: () = msg_send![session, setAnimatesToStartingPositionsOnCancelOrFail: NO];
        session != nil
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct CGPoint { x: f64, y: f64 }

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {
    fn CGEventCreate(source: *const std::ffi::c_void) -> *mut std::ffi::c_void;
    fn CGEventGetLocation(ev: *mut std::ffi::c_void) -> CGPoint;
    fn CGWarpMouseCursorPosition(p: CGPoint) -> i32;
    fn CGAssociateMouseAndMouseCursorPosition(connected: u8) -> i32;
    fn CGMainDisplayID() -> u32;
    fn CGDisplayHideCursor(display: u32) -> i32;
    fn CGDisplayShowCursor(display: u32) -> i32;
}
#[link(name = "CoreFoundation", kind = "framework")]
extern "C" { fn CFRelease(p: *mut std::ffi::c_void); }

/// Screen position of the cursor, in points from the top-left of the main display.
pub fn cursor() -> (f64, f64) {
    unsafe {
        let ev = CGEventCreate(std::ptr::null());
        if ev.is_null() { return (0.0, 0.0); }
        let p = CGEventGetLocation(ev);
        CFRelease(ev);
        (p.x, p.y)
    }
}
pub fn warp(p: (f64, f64)) {
    unsafe {
        CGWarpMouseCursorPosition(CGPoint { x: p.0, y: p.1 });
        // lifts the short freeze that follows a warp
        CGAssociateMouseAndMouseCursorPosition(1);
    }
}
pub fn show_cursor(on: bool) { unsafe { if on { CGDisplayShowCursor(CGMainDisplayID()) } else { CGDisplayHideCursor(CGMainDisplayID()) }; } }
