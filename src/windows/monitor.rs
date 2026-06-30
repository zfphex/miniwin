use crate::*;

#[inline]
pub fn get_window_rect(hwnd: isize) -> RECT {
    let mut rect = RECT::default();
    let _ = unsafe { GetWindowRect(hwnd, &mut rect) };
    rect
}

pub fn get_client_rect(hwnd: isize) -> Rect {
    let mut rect = RECT::default();
    let _ = unsafe { GetClientRect(hwnd, &mut rect) };
    Rect::from_windows(rect)
}

#[inline]
pub fn desktop_area() -> Rect {
    unsafe { get_client_rect(GetDesktopWindow()) }
}

pub const MONITOR_DEFAULTTONULL: u32 = 0x00000000;
pub const MONITOR_DEFAULTTOPRIMARY: u32 = 0x00000001;
pub const MONITOR_DEFAULTTONEAREST: u32 = 0x00000002;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct MONITORINFO {
    pub cbSize: u32,
    pub rcMonitor: RECT,
    pub rcWork: RECT,
    pub dwFlags: u32,
}

impl Default for MONITORINFO {
    fn default() -> Self {
        Self {
            cbSize: size_of::<Self>() as u32,
            rcMonitor: RECT::default(),
            rcWork: RECT::default(),
            dwFlags: 0,
        }
    }
}

#[link(name = "user32")]
extern "system" {
    pub fn MonitorFromPoint(pt: POINT, dwFlags: u32) -> *mut c_void;
    /// You must set the cbSize member of the structure to sizeof(MONITORINFO) or sizeof(MONITORINFOEX) before calling the GetMonitorInfo function.
    /// Doing so lets the function determine the type of structure you are passing to it.
    pub fn GetMonitorInfoA(hMonitor: *mut c_void, lpmi: *mut MONITORINFO) -> BOOL;
}
