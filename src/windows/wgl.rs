use crate::*;

#[link(name = "Opengl32")]
extern "system" {
    pub fn wglCreateContext(hdc: *mut core::ffi::c_void) -> HGLRC;
    pub fn wglMakeCurrent(hdc: *mut core::ffi::c_void, hglrc: HGLRC) -> i32;
    pub fn wglDeleteContext(hglrc: HGLRC) -> i32;
    pub fn wglGetProcAddress(name: *const i8) -> *const c_void;
}
