use crate::*;

/// Tray icon data structure
#[repr(C)]
struct NotifyIconData {
    cb_size: u32,
    hwnd: isize,
    u_id: u32,
    u_flags: u32,
    u_callback_message: u32,
    h_icon: *mut c_void,
    sz_tip: [u8; 128],
    dw_state: u32,
    dw_state_mask: u32,
    sz_info: [u8; 256],
    u_timeout_or_version: u32,
    sz_info_title: [u8; 64],
    dw_info_flags: u32,
    guide_id: u128,
}

impl NotifyIconData {
    fn new() -> Self {
        Self {
            cb_size: std::mem::size_of::<NotifyIconData>() as u32,
            hwnd: 0,
            u_id: 0,
            u_flags: 0,
            u_callback_message: 0,
            h_icon: null_mut(),
            sz_tip: [0; 128],
            dw_state: 0,
            dw_state_mask: 0,
            sz_info: [0; 256],
            u_timeout_or_version: 0,
            sz_info_title: [0; 64],
            dw_info_flags: 0,
            guide_id: 0,
        }
    }
}

/// Tray icon notification flags
pub const NIF_MESSAGE: u32 = 0x00000001;
pub const NIF_ICON: u32 = 0x00000002;
pub const NIF_TIP: u32 = 0x00000004;
pub const NIF_STATE: u32 = 0x00000008;
pub const NIF_INFO: u32 = 0x00000010;

/// Tray icon operations
pub const NIM_ADD: u32 = 0x00000000;
pub const NIM_MODIFY: u32 = 0x00000001;
pub const NIM_DELETE: u32 = 0x00000002;
pub const NIM_SETVERSION: u32 = 0x00000004;

/// Icon states
pub const NIS_HIDDEN: u32 = 0x00000001;
pub const NIS_SHAREDICON: u32 = 0x00000002;

/// Notification info flags
pub const NIIF_NONE: u32 = 0x00000000;
pub const NIIF_INFO: u32 = 0x00000001;
pub const NIIF_WARNING: u32 = 0x00000002;
pub const NIIF_ERROR: u32 = 0x00000003;
pub const NIIF_USER: u32 = 0x00000004;

pub const WM_TRAYICON: u32 = WM_APP + 1;

/// Create a new tray icon
pub fn create_tray_icon(hwnd: isize, tray_id: u32, icon_handle: *mut c_void, tooltip: &str) {
    unsafe {
        let mut nid = NotifyIconData::new();
        nid.hwnd = hwnd;
        nid.u_id = tray_id;
        nid.u_flags = NIF_MESSAGE | NIF_ICON | NIF_TIP;
        nid.u_callback_message = WM_TRAYICON; 
        nid.h_icon = icon_handle;

        // Copy tooltip (null-terminated, max 127 chars)
        let tip_len = tooltip.len().min(127);
        nid.sz_tip[..tip_len].copy_from_slice(&tooltip.as_bytes()[..tip_len]);
        nid.sz_tip[tip_len] = 0;

        assert!(Shell_NotifyIconA(NIM_ADD, &nid) != 0);
    }
}

/// Remove a tray icon
pub fn remove_tray_icon(hwnd: isize, tray_id: u32) {
    unsafe {
        let mut nid = NotifyIconData::new();
        nid.hwnd = hwnd;
        nid.u_id = tray_id;

        assert!(Shell_NotifyIconA(NIM_DELETE, &nid) != 0);
    }
}

#[link(name = "shell32")]
extern "system" {
    fn Shell_NotifyIconA(dwMessage: u32, lpData: *const NotifyIconData) -> i32;
}
