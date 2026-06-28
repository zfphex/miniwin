use crate::ffi::{self, NO, NSRect, YES, id};
use crate::objc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowStyle {
    Standard,
    Borderless,
    Transparent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FullscreenMode {
    None,
    Workspace,  // Native macOS fullscreen
    MonitorFit, // Fits the borderless window to monitor size
}

pub struct Window {
    pub ns_window: id,
    pub ns_view: id,
}

impl Window {
    pub fn new(
        title: &str,
        width: f64,
        height: f64,
        style: WindowStyle,
        fullscreen: FullscreenMode,
    ) -> Self {
        unsafe {
            let alloc_sel = ffi::sel_registerName(b"alloc\0".as_ptr() as *const _);

            let mut final_width = width;
            let mut final_height = height;

            let mut style_mask = match style {
                WindowStyle::Standard => {
                    ffi::NSWindowStyleMaskTitled
                        | ffi::NSWindowStyleMaskClosable
                        | ffi::NSWindowStyleMaskMiniaturizable
                        | ffi::NSWindowStyleMaskResizable
                }
                WindowStyle::Borderless | WindowStyle::Transparent => {
                    ffi::NSWindowStyleMaskBorderless
                }
            };

            if fullscreen == FullscreenMode::Workspace {
                style_mask |= ffi::NSWindowStyleMaskFullScreen;
            }

            if fullscreen == FullscreenMode::MonitorFit {
                let main_screen = objc::msg_send_id(
                    ffi::objc_getClass(b"NSScreen\0".as_ptr() as *const _),
                    ffi::sel_registerName(b"mainScreen\0".as_ptr() as *const _),
                );
                let frame_sel = ffi::sel_registerName(b"frame\0".as_ptr() as *const _);
                let frame_func: unsafe extern "C" fn(id, ffi::SEL) -> NSRect =
                    std::mem::transmute(ffi::objc_msgSend as *const std::ffi::c_void);
                let screen_rect = frame_func(main_screen, frame_sel);
                final_width = screen_rect.size.width;
                final_height = screen_rect.size.height;
                style_mask = ffi::NSWindowStyleMaskBorderless;
            }

            let rect = NSRect::new(0.0, 0.0, final_width, final_height);
            let window_class = ffi::objc_getClass(b"NSWindow\0".as_ptr() as *const _);
            let window_alloc = objc::msg_send_id(window_class, alloc_sel);

            let init_window_func: unsafe extern "C" fn(
                id,
                ffi::SEL,
                NSRect,
                ffi::NSWindowStyleMask,
                ffi::NSBackingStoreType,
                ffi::BOOL,
            ) -> id = std::mem::transmute(ffi::objc_msgSend as *const std::ffi::c_void);

            let ns_window = init_window_func(
                window_alloc,
                ffi::sel_registerName(
                    b"initWithContentRect:styleMask:backing:defer:\0".as_ptr() as *const _
                ),
                rect,
                style_mask,
                ffi::NSBackingStoreBuffered,
                NO,
            );

            if style == WindowStyle::Transparent {
                objc::msg_send_id_bool_void(
                    ns_window,
                    ffi::sel_registerName(b"setOpaque:\0".as_ptr() as *const _),
                    NO,
                );
                let color_class = ffi::objc_getClass(b"NSColor\0".as_ptr() as *const _);
                let clear_color = objc::msg_send_id(
                    color_class,
                    ffi::sel_registerName(b"clearColor\0".as_ptr() as *const _),
                );
                objc::msg_send_id_id_void(
                    ns_window,
                    ffi::sel_registerName(b"setBackgroundColor:\0".as_ptr() as *const _),
                    clear_color,
                );
            }

            let title_ns = objc::nsstring(title);
            objc::msg_send_id_id_void(
                ns_window,
                ffi::sel_registerName(b"setTitle:\0".as_ptr() as *const _),
                title_ns,
            );

            let view_class = ffi::objc_getClass(b"NSView\0".as_ptr() as *const _);
            let view_alloc = objc::msg_send_id(view_class, alloc_sel);
            let init_view_func: unsafe extern "C" fn(id, ffi::SEL, NSRect) -> id =
                std::mem::transmute(ffi::objc_msgSend as *const std::ffi::c_void);
            let ns_view = init_view_func(
                view_alloc,
                ffi::sel_registerName(b"initWithFrame:\0".as_ptr() as *const _),
                rect,
            );

            objc::msg_send_id_bool_void(
                ns_view,
                ffi::sel_registerName(b"setWantsLayer:\0".as_ptr() as *const _),
                YES,
            );

            objc::msg_send_id_id_void(
                ns_window,
                ffi::sel_registerName(b"setContentView:\0".as_ptr() as *const _),
                ns_view,
            );

            if fullscreen == FullscreenMode::Workspace {
                // NSWindowCollectionBehaviorFullScreenPrimary = 1 << 7
                objc::msg_send_id_usize_void(
                    ns_window,
                    ffi::sel_registerName(b"setCollectionBehavior:\0".as_ptr() as *const _),
                    1 << 7,
                );
            }

            Window { ns_window, ns_view }
        }
    }

    pub fn make_key_and_order_front(&self) {
        unsafe {
            objc::msg_send_id_id_void(
                self.ns_window,
                ffi::sel_registerName(b"makeKeyAndOrderFront:\0".as_ptr() as *const _),
                ffi::nil,
            );
        }
    }
}
