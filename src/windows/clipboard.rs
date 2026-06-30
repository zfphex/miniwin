use core::ffi::c_void;

pub fn copy_to_clipboard(text: &str) {
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

pub const GMEM_FIXED: u32 = 0x0000;
pub const GMEM_MOVEABLE: u32 = 0x0002;
pub const GMEM_NOCOMPACT: u32 = 0x0010;
pub const GMEM_NODISCARD: u32 = 0x0020;
pub const GMEM_ZEROINIT: u32 = 0x0040;
pub const GMEM_MODIFY: u32 = 0x0080;
pub const GMEM_DISCARDABLE: u32 = 0x0100;
pub const GMEM_NOT_BANKED: u32 = 0x1000;
pub const GMEM_SHARE: u32 = 0x2000;
pub const GMEM_DDESHARE: u32 = 0x2000;
pub const GMEM_NOTIFY: u32 = 0x4000;
pub const GMEM_LOWER: u32 = GMEM_NOT_BANKED;
pub const GMEM_VALID_FLAGS: u32 = 0x7F72;
pub const GMEM_INVALID_HANDLE: u32 = 0x8000;
pub const GHND: u32 = GMEM_MOVEABLE | GMEM_ZEROINIT;
pub const GPTR: u32 = GMEM_FIXED | GMEM_ZEROINIT;
pub const GMEM_DISCARDED: u32 = 0x4000;
pub const GMEM_LOCKCOUNT: u32 = 0x00FF;

pub const CF_TEXT: u32 = 1;
pub const CF_BITMAP: u32 = 2;
pub const CF_METAFILEPICT: u32 = 3;
pub const CF_SYLK: u32 = 4;
pub const CF_DIF: u32 = 5;
pub const CF_TIFF: u32 = 6;
pub const CF_OEMTEXT: u32 = 7;
pub const CF_DIB: u32 = 8;
pub const CF_PALETTE: u32 = 9;
pub const CF_PENDATA: u32 = 10;
pub const CF_RIFF: u32 = 11;
pub const CF_WAVE: u32 = 12;
pub const CF_UNICODETEXT: u32 = 13;
pub const CF_ENHMETAFILE: u32 = 14;
pub const CF_HDROP: u32 = 15;
pub const CF_LOCALE: u32 = 16;
pub const CF_DIBV5: u32 = 17;
pub const CF_MAX: u32 = 18;
pub const CF_OWNERDISPLAY: u32 = 0x0080;
pub const CF_DSPTEXT: u32 = 0x0081;
pub const CF_DSPBITMAP: u32 = 0x0082;
pub const CF_DSPMETAFILEPICT: u32 = 0x0083;
pub const CF_DSPENHMETAFILE: u32 = 0x008E;
pub const CF_PRIVATEFIRST: u32 = 0x0200;
pub const CF_PRIVATELAST: u32 = 0x02FF;
pub const CF_GDIOBJFIRST: u32 = 0x0300;
pub const CF_GDIOBJLAST: u32 = 0x03FF;
