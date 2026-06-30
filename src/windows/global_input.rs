use crate::*;
use std::sync::{
    atomic::{
        AtomicBool, AtomicI32,
        Ordering::{self, *},
    },
    Once,
};

pub const WH_MOUSE_LL: i32 = 14;

#[repr(C)]
#[derive(Debug, Default)]
pub struct MSLLHOOKSTRUCT {
    pub pt: POINT,
    pub mouseData: DWORD,
    pub flags: DWORD,
    pub time: DWORD,
    pub dwExtraInfo: usize,
}

pub type HOOKPROC =
    Option<unsafe extern "system" fn(code: i32, wParam: WPARAM, lParam: LPARAM) -> LRESULT>;

#[link(name = "user32")]
extern "system" {
    pub fn GetModuleHandleA(lpModuleName: LPCSTR) -> *mut c_void;
    pub fn SetWindowsHookExA(
        hook: i32,
        lpfn: HOOKPROC,
        hmod: *mut c_void,
        thread_id: u32,
    ) -> *mut c_void;
    pub fn CallNextHookEx(hhk: *mut c_void, code: i32, wParam: usize, lParam: isize) -> isize;
    pub fn UnhookWindowsHookEx(hhk: *mut c_void) -> i32;
    pub fn GetAsyncKeyState(key: i32) -> i16;
    pub fn PostThreadMessageA(idThread: DWORD, msg: UINT, wParam: WPARAM, lParam: LPARAM) -> BOOL;
    pub fn GetCurrentThreadId() -> u32;
}

pub static mut HOOK: *mut c_void = core::ptr::null_mut();
pub static mut ONCE: Once = Once::new();

// const USER_MOUSEWHEEL: u32 = WM_USER + 1;
// const USER_LBUTTONDOWN: u32 = WM_USER + 2;
// const USER_LBUTTONUP: u32 = WM_USER + 3;
// const USER_LBUTTONDBLCLK: u32 = WM_USER + 4;
// const USER_RBUTTONDOWN: u32 = WM_USER + 5;
// const USER_RBUTTONUP: u32 = WM_USER + 6;
// const USER_RBUTTONDBLCLK: u32 = WM_USER + 7;
// const USER_MBUTTONDOWN: u32 = WM_USER + 8;
// const USER_MBUTTONUP: u32 = WM_USER + 9;
// const USER_MBUTTONDBLCLK: u32 = WM_USER + 10;
// const USER_XBUTTONDOWN: u32 = WM_USER + 11;
// const USER_XBUTTONUP: u32 = WM_USER + 12;
// const USER_XBUTTONDBLCLK: u32 = WM_USER + 13;

// pub unsafe extern "system" fn mouse_proc(code: i32, w_param: usize, l_param: isize) -> isize {
//     if code >= 0 {
//         let msg = match w_param as u32 {
//             WM_MOUSEWHEEL => {
//                 let thread_id = GetCurrentThreadId();
//                 //For some reaons the mouse data is not working when I send it over.
//                 let mouse = unsafe { &*(l_param as *const MSLLHOOKSTRUCT) };
//                 //Just pass the delta into the w_param and convert it back to i16.
//                 //Should be fine right? 😏
//                 let delta = (mouse.mouseData >> 16) as i16;
//                 assert!(
//                     PostThreadMessageA(thread_id, USER_MOUSEWHEEL, delta as usize, l_param) != 0
//                 );
//                 return CallNextHookEx(HOOK, code, w_param, l_param);
//             }
//             WM_LBUTTONDOWN => USER_LBUTTONDOWN,
//             WM_LBUTTONUP => USER_LBUTTONUP,
//             WM_LBUTTONDBLCLK => USER_LBUTTONDBLCLK,
//             WM_RBUTTONDOWN => USER_RBUTTONDOWN,
//             WM_RBUTTONUP => USER_RBUTTONUP,
//             WM_RBUTTONDBLCLK => USER_RBUTTONDBLCLK,
//             WM_MBUTTONDOWN => USER_MBUTTONDOWN,
//             WM_MBUTTONUP => USER_MBUTTONUP,
//             WM_MBUTTONDBLCLK => USER_MBUTTONDBLCLK,
//             WM_XBUTTONDOWN => USER_XBUTTONDOWN,
//             WM_XBUTTONUP => USER_XBUTTONUP,
//             WM_XBUTTONDBLCLK => USER_XBUTTONDBLCLK,
//             _ => return CallNextHookEx(HOOK, code, w_param, l_param),
//         };

//         let thread_id = GetCurrentThreadId();
//         assert!(PostThreadMessageA(thread_id, msg, w_param, l_param) != 0);
//     }

//     CallNextHookEx(HOOK, code, w_param, l_param)
// }

pub unsafe fn watch_global_mouse_events() {
    unsafe {
        ONCE.call_once(|| {
            let instance = GetModuleHandleA(core::ptr::null());
            HOOK = SetWindowsHookExA(WH_MOUSE_LL, Some(mouse_proc), instance, 0);
            assert!(!HOOK.is_null());
        });
        let mut msg: MSG = core::mem::zeroed();
        let result = PeekMessageA(&mut msg, 0, 0, 0, PM_REMOVE);
        TranslateMessage(&msg);
        DispatchMessageA(&msg);
    }
}

// pub fn wait_for_global_events() -> Option<Event> {
//     unsafe {
//         ONCE.call_once(|| {
//             let instance = GetModuleHandleA(core::ptr::null());
//             HOOK = SetWindowsHookExA(WH_MOUSE_LL, Some(mouse_proc), instance, 0);
//             assert!(!HOOK.is_null());
//         });

//         let mut msg: MSG = core::mem::zeroed();
//         let result = GetMessageA(&mut msg, 0, 0, 0);

//         if msg.message > WM_USER {
//             handle_mouse_msg(msg, result)
//         } else {
//             None
//         }
//     }
// }

#[derive(Debug, Default)]
pub struct AtomicPos {
    pub x: AtomicI32,
    pub y: AtomicI32,
}

impl AtomicPos {
    pub const fn new() -> Self {
        Self {
            x: AtomicI32::new(0),
            y: AtomicI32::new(0),
        }
    }
    pub fn get_x(&self) -> i32 {
        self.x.load(Ordering::Relaxed)
    }
    pub fn get_y(&self) -> i32 {
        self.y.load(Ordering::Relaxed)
    }
}

#[derive(Debug, Default)]
pub struct AtomicMouseButtonState {
    pub pressed: AtomicBool,
    pub released: AtomicBool,
    pub inital_position: AtomicPos,
    pub release_position: Option<AtomicPos>,
}

impl AtomicMouseButtonState {
    pub const fn new() -> Self {
        Self {
            pressed: AtomicBool::new(false),
            released: AtomicBool::new(false),
            inital_position: AtomicPos::new(),
            release_position: None,
        }
    }
    pub fn clicked(&mut self) -> bool {
        if self.released.load(Relaxed) {
            self.pressed.store(false, Relaxed);
            self.released.store(false, Relaxed);
            true
        } else {
            false
        }
    }
    pub fn pressed(&mut self, pos: AtomicPos) {
        self.pressed.store(true, Relaxed);
        self.released.store(false, Relaxed);
        self.inital_position = pos;
        self.release_position = None;
    }
    pub fn released(&mut self, pos: AtomicPos) {
        self.pressed.store(false, Relaxed);
        self.released.store(true, Relaxed);
        self.release_position = Some(pos);
    }
}

#[derive(Default)]
pub struct GlobalMouseState {
    pub left_mouse: AtomicMouseButtonState,
    pub right_mouse: AtomicMouseButtonState,
    pub middle_mouse: AtomicMouseButtonState,
    pub mouse_4: AtomicMouseButtonState,
    pub mouse_5: AtomicMouseButtonState,
}

impl GlobalMouseState {
    pub const fn new() -> Self {
        Self {
            left_mouse: AtomicMouseButtonState::new(),
            right_mouse: AtomicMouseButtonState::new(),
            middle_mouse: AtomicMouseButtonState::new(),
            mouse_4: AtomicMouseButtonState::new(),
            mouse_5: AtomicMouseButtonState::new(),
        }
    }
}

//TODO: Use UnsafeCell for this.
pub static mut GLOBAL_MOUSE_STATE: GlobalMouseState = GlobalMouseState::new();

#[allow(static_mut_refs)]
pub const fn global_state() -> &'static mut GlobalMouseState {
    unsafe { &mut GLOBAL_MOUSE_STATE }
}

pub unsafe extern "system" fn mouse_proc(code: i32, w_param: usize, l_param: isize) -> isize {
    let msg = w_param as u32;
    let ptr = l_param as *const MSLLHOOKSTRUCT;

    if code >= 0 && !ptr.is_null() {
        let mouse = unsafe { &*(l_param as *const MSLLHOOKSTRUCT) };
        let modifiers = modifiers();

        let pos = AtomicPos {
            x: AtomicI32::new(mouse.pt.x),
            y: AtomicI32::new(mouse.pt.y),
        };

        // dbg!(mouse, modifiers, &pos);

        let msg = match w_param as u32 {
            WM_MOUSEWHEEL => {
                const WHEEL_DELTA: i16 = 120;
                let delta = (w_param as i16) as f32 / WHEEL_DELTA as f32;
                // if delta >= 0.0 {
                //     return Some(Event::Input(Key::ScrollUp, modifiers));
                // } else {
                //     return Some(Event::Input(Key::ScrollDown, modifiers));
                // }
            }
            //TODO: Double clicks.
            WM_LBUTTONDOWN => GLOBAL_MOUSE_STATE.left_mouse.pressed(pos),
            WM_LBUTTONUP => GLOBAL_MOUSE_STATE.left_mouse.released(pos),
            WM_RBUTTONDOWN => GLOBAL_MOUSE_STATE.right_mouse.pressed(pos),
            WM_RBUTTONUP => GLOBAL_MOUSE_STATE.right_mouse.released(pos),
            WM_MBUTTONDOWN => GLOBAL_MOUSE_STATE.middle_mouse.pressed(pos),
            WM_MBUTTONUP => GLOBAL_MOUSE_STATE.middle_mouse.released(pos),
            WM_XBUTTONDOWN => match mouse.mouseData.high() {
                1 => GLOBAL_MOUSE_STATE.mouse_4.pressed(pos),
                2 => GLOBAL_MOUSE_STATE.mouse_5.pressed(pos),
                _ => {} // _ => return None,
            },
            WM_XBUTTONUP => match mouse.mouseData.high() {
                1 => GLOBAL_MOUSE_STATE.mouse_4.released(pos),
                2 => GLOBAL_MOUSE_STATE.mouse_5.released(pos),
                _ => {} // _ => return None,
            },
            _ => {}
        };
    }

    CallNextHookEx(HOOK, code, w_param, l_param)
}

///https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
pub fn is_down(virtual_key: i32) -> bool {
    (unsafe { GetAsyncKeyState(virtual_key) } & 0x8000u16 as i16) != 0
}

pub fn get_mouse_position() -> (i32, i32) {
    let mut point = POINT { x: 0, y: 0 };
    let _ = unsafe { GetCursorPos(&mut point) };

    (point.x, point.y)
}

pub fn get_physical_mouse_position() -> (i32, i32) {
    let mut point = POINT { x: 0, y: 0 };
    let _ = unsafe { GetPhysicalCursorPos(&mut point) };

    (point.x, point.y)
}
