#![allow(dead_code)]

use crate::ffi::{BOOL, NSRect, SEL, id, objc_msgSend};
use std::mem::transmute;
use std::os::raw::c_void;

// Basic msgSend wrappers
pub unsafe fn msg_send_id(obj: id, sel: SEL) -> id {
    unsafe {
        let func: unsafe extern "C" fn(id, SEL) -> id = transmute(objc_msgSend as *const c_void);
        func(obj, sel)
    }
}

pub unsafe fn msg_send_id_id(obj: id, sel: SEL, arg1: id) -> id {
    unsafe {
        let func: unsafe extern "C" fn(id, SEL, id) -> id =
            transmute(objc_msgSend as *const c_void);
        func(obj, sel, arg1)
    }
}

pub unsafe fn msg_send_id_id_void(obj: id, sel: SEL, arg1: id) {
    unsafe {
        let func: unsafe extern "C" fn(id, SEL, id) = transmute(objc_msgSend as *const c_void);
        func(obj, sel, arg1)
    }
}

pub unsafe fn msg_send_id_bool_void(obj: id, sel: SEL, arg1: BOOL) {
    unsafe {
        let func: unsafe extern "C" fn(id, SEL, BOOL) = transmute(objc_msgSend as *const c_void);
        func(obj, sel, arg1)
    }
}

pub unsafe fn msg_send_id_usize_void(obj: id, sel: SEL, arg1: usize) {
    unsafe {
        let func: unsafe extern "C" fn(id, SEL, usize) = transmute(objc_msgSend as *const c_void);
        func(obj, sel, arg1)
    }
}

pub unsafe fn msg_send_id_rect_void(obj: id, sel: SEL, rect: NSRect) {
    unsafe {
        let func: unsafe extern "C" fn(id, SEL, NSRect) = transmute(objc_msgSend as *const c_void);
        func(obj, sel, rect)
    }
}

// Convert a Rust string to a Cocoa NSString (retaining pointer)
pub unsafe fn nsstring(s: &str) -> id {
    unsafe {
        let nsstring_class = crate::ffi::objc_getClass(b"NSString\0".as_ptr() as *const _);
        let alloc_sel = crate::ffi::sel_registerName(b"alloc\0".as_ptr() as *const _);
        let init_sel =
            crate::ffi::sel_registerName(b"initWithBytes:length:encoding:\0".as_ptr() as *const _);

        let allocated = msg_send_id(nsstring_class, alloc_sel);
        let init_func: unsafe extern "C" fn(id, SEL, *const c_void, usize, usize) -> id =
            transmute(objc_msgSend as *const c_void);
        init_func(allocated, init_sel, s.as_ptr() as *const c_void, s.len(), 4) // 4 is NSUTF8StringEncoding
    }
}

#[cfg(target_arch = "x86_64")]
pub unsafe fn msg_send_rect(obj: id, sel: SEL) -> NSRect {
    unsafe {
        let mut rect = std::mem::zeroed();
        let func: unsafe extern "C" fn(*mut NSRect, id, SEL) = transmute(crate::ffi::objc_msgSend_stret as *const c_void);
        func(&mut rect, obj, sel);
        rect
    }
}

#[cfg(not(target_arch = "x86_64"))]
pub unsafe fn msg_send_rect(obj: id, sel: SEL) -> NSRect {
    unsafe {
        let func: unsafe extern "C" fn(id, SEL) -> NSRect = transmute(objc_msgSend as *const c_void);
        func(obj, sel)
    }
}
