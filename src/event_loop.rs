use crate::ffi::{self, id};
use crate::objc;

pub struct EventLoop {
    pub ns_app: id,
}

impl EventLoop {
    pub fn new() -> Self {
        unsafe {
            let ns_app = objc::msg_send_id(
                ffi::objc_getClass(b"NSApplication\0".as_ptr() as *const _),
                ffi::sel_registerName(b"sharedApplication\0".as_ptr() as *const _),
            );

            let set_policy_sel =
                ffi::sel_registerName(b"setActivationPolicy:\0".as_ptr() as *const _);
            let set_policy: unsafe extern "C" fn(
                id,
                ffi::SEL,
                ffi::NSApplicationActivationPolicy,
            ) -> ffi::BOOL = std::mem::transmute(ffi::objc_msgSend as *const std::ffi::c_void);
            set_policy(
                ns_app,
                set_policy_sel,
                ffi::NSApplicationActivationPolicyRegular,
            );

            Self::setup_menu_bar(ns_app);

            EventLoop { ns_app }
        }
    }

    unsafe fn setup_menu_bar(ns_app: id) {
        unsafe {
            let alloc_sel = ffi::sel_registerName(b"alloc\0".as_ptr() as *const _);
            let init_sel = ffi::sel_registerName(b"init\0".as_ptr() as *const _);

            let main_menu = objc::msg_send_id(
                objc::msg_send_id(
                    ffi::objc_getClass(b"NSMenu\0".as_ptr() as *const _),
                    alloc_sel,
                ),
                init_sel,
            );

            let app_menu_item = objc::msg_send_id(
                objc::msg_send_id(
                    ffi::objc_getClass(b"NSMenuItem\0".as_ptr() as *const _),
                    alloc_sel,
                ),
                init_sel,
            );

            objc::msg_send_id_id_void(
                main_menu,
                ffi::sel_registerName(b"addItem:\0".as_ptr() as *const _),
                app_menu_item,
            );

            let app_menu = objc::msg_send_id(
                objc::msg_send_id(
                    ffi::objc_getClass(b"NSMenu\0".as_ptr() as *const _),
                    alloc_sel,
                ),
                init_sel,
            );

            objc::msg_send_id_id_void(
                app_menu_item,
                ffi::sel_registerName(b"setSubmenu:\0".as_ptr() as *const _),
                app_menu,
            );

            let quit_title = objc::nsstring("Quit");
            let quit_sel = ffi::sel_registerName(b"terminate:\0".as_ptr() as *const _);
            let key = objc::nsstring("q");
            let quit_item_alloc = objc::msg_send_id(
                ffi::objc_getClass(b"NSMenuItem\0".as_ptr() as *const _),
                alloc_sel,
            );

            let init_quit_func: unsafe extern "C" fn(id, ffi::SEL, id, ffi::SEL, id) -> id =
                std::mem::transmute(ffi::objc_msgSend as *const std::ffi::c_void);
            let quit_item = init_quit_func(
                quit_item_alloc,
                ffi::sel_registerName(b"initWithTitle:action:keyEquivalent:\0".as_ptr() as *const _),
                quit_title,
                quit_sel,
                key,
            );

            objc::msg_send_id_id_void(
                app_menu,
                ffi::sel_registerName(b"addItem:\0".as_ptr() as *const _),
                quit_item,
            );

            objc::msg_send_id_id_void(
                ns_app,
                ffi::sel_registerName(b"setMainMenu:\0".as_ptr() as *const _),
                main_menu,
            );
        }
    }
}
