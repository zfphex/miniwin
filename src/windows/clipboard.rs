use core::ffi::c_void;

pub fn copy_to_clipboard(text: &str) {
    const GMEM_MOVEABLE: u32 = 0x0002;
    const GMEM_ZEROINIT: u32 = 0x0040;

    unsafe {
        assert!(OpenClipboard(0) != 0);
        assert!(EmptyClipboard() != 0);

        let galloc = GlobalAlloc(GMEM_MOVEABLE | GMEM_ZEROINIT, text.len() + 1);
        assert!(!galloc.is_null());

        let glock = GlobalLock(galloc) as *mut u8;
        assert!(!glock.is_null());

        core::ptr::copy_nonoverlapping(text.as_ptr(), glock, text.len());
        *glock.add(text.len()) = 0;

        GlobalUnlock(galloc);

        assert!(!SetClipboardData(CF_TEXT, galloc).is_null());
        assert!(CloseClipboard() != 0);
    }
}

#[link(name = "user32")]
unsafe extern "system" {
    pub fn OpenClipboard(hwnd: isize) -> i32;
    pub fn CloseClipboard() -> i32;
    pub fn SetClipboardData(format: u32, mem: *mut c_void) -> *mut c_void;
    pub fn GetClipboardData(format: u32) -> *mut c_void;
    pub fn EmptyClipboard() -> i32;
    pub fn GlobalAlloc(flags: u32, bytes: usize) -> *mut c_void;
    pub fn GlobalLock(mem: *mut c_void) -> *mut c_void;
    pub fn GlobalUnlock(mem: *mut c_void) -> i32;
}

pub const CF_TEXT: u32 = 1;
