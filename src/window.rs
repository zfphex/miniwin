use crate::event_loop::*;
use crate::ffi::*;
use crate::objc::*;

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
    pub ns_delegate: id,
    _marker: std::marker::PhantomData<*mut ()>,
}

impl Window {
    pub fn new(
        title: &str,
        width: f64,
        height: f64,
        style: WindowStyle,
        fullscreen: FullscreenMode,
    ) -> Self {
        #[cfg(debug_assertions)]
        assert_main_thread();
        unsafe {
            let alloc_sel = sel_registerName(b"alloc\0".as_ptr() as *const _);

            let mut final_width = width;
            let mut final_height = height;

            let mut style_mask = match style {
                WindowStyle::Standard => {
                    NSWindowStyleMaskTitled
                        | NSWindowStyleMaskClosable
                        | NSWindowStyleMaskMiniaturizable
                        | NSWindowStyleMaskResizable
                }
                WindowStyle::Borderless | WindowStyle::Transparent => NSWindowStyleMaskBorderless,
            };

            if fullscreen == FullscreenMode::Workspace {
                style_mask |= NSWindowStyleMaskFullScreen;
            }

            if fullscreen == FullscreenMode::MonitorFit {
                let main_screen = msg_send_id(
                    objc_getClass(b"NSScreen\0".as_ptr() as *const _),
                    sel_registerName(b"mainScreen\0".as_ptr() as *const _),
                );
                let frame_sel = sel_registerName(b"frame\0".as_ptr() as *const _);
                let screen_rect = msg_send_rect(main_screen, frame_sel);
                final_width = screen_rect.size.width;
                final_height = screen_rect.size.height;
                style_mask = NSWindowStyleMaskBorderless;
            }

            let rect = NSRect::new(0.0, 0.0, final_width, final_height);
            let window_class = objc_getClass(b"NSWindow\0".as_ptr() as *const _);
            let window_alloc = msg_send_id(window_class, alloc_sel);

            let init_window_func: unsafe extern "C" fn(
                id,
                SEL,
                NSRect,
                NSWindowStyleMask,
                NSBackingStoreType,
                BOOL,
            ) -> id = std::mem::transmute(objc_msgSend as *const std::ffi::c_void);

            let ns_window = init_window_func(
                window_alloc,
                sel_registerName(
                    b"initWithContentRect:styleMask:backing:defer:\0".as_ptr() as *const _
                ),
                rect,
                style_mask,
                NSBackingStoreBuffered,
                NO,
            );

            if style == WindowStyle::Transparent {
                msg_send_id_bool_void(
                    ns_window,
                    sel_registerName(b"setOpaque:\0".as_ptr() as *const _),
                    NO,
                );
                let color_class = objc_getClass(b"NSColor\0".as_ptr() as *const _);
                let clear_color = msg_send_id(
                    color_class,
                    sel_registerName(b"clearColor\0".as_ptr() as *const _),
                );
                msg_send_id_id_void(
                    ns_window,
                    sel_registerName(b"setBackgroundColor:\0".as_ptr() as *const _),
                    clear_color,
                );
            }

            let title_ns = nsstring(title);
            msg_send_id_id_void(
                ns_window,
                sel_registerName(b"setTitle:\0".as_ptr() as *const _),
                title_ns,
            );

            let view_class = objc_getClass(b"NSView\0".as_ptr() as *const _);
            let view_alloc = msg_send_id(view_class, alloc_sel);
            let init_view_func: unsafe extern "C" fn(id, SEL, NSRect) -> id =
                std::mem::transmute(objc_msgSend as *const std::ffi::c_void);
            let ns_view = init_view_func(
                view_alloc,
                sel_registerName(b"initWithFrame:\0".as_ptr() as *const _),
                rect,
            );

            msg_send_id_bool_void(
                ns_view,
                sel_registerName(b"setWantsLayer:\0".as_ptr() as *const _),
                YES,
            );

            msg_send_id_id_void(
                ns_window,
                sel_registerName(b"setContentView:\0".as_ptr() as *const _),
                ns_view,
            );

            if fullscreen == FullscreenMode::Workspace {
                // NSWindowCollectionBehaviorFullScreenPrimary = 1 << 7
                msg_send_id_usize_void(
                    ns_window,
                    sel_registerName(b"setCollectionBehavior:\0".as_ptr() as *const _),
                    1 << 7,
                );
            }

            // Create and set delegate
            let delegate_class = register_delegate_class();
            let delegate_alloc = msg_send_id(
                delegate_class,
                sel_registerName(b"alloc\0".as_ptr() as *const _),
            );
            let ns_delegate = msg_send_id(
                delegate_alloc,
                sel_registerName(b"init\0".as_ptr() as *const _),
            );

            msg_send_id_id_void(
                ns_window,
                sel_registerName(b"setDelegate:\0".as_ptr() as *const _),
                ns_delegate,
            );

            Window {
                ns_window,
                ns_view,
                ns_delegate,
                _marker: std::marker::PhantomData,
            }
        }
    }

    pub fn make_key_and_order_front(&self) {
        unsafe {
            msg_send_id_id_void(
                self.ns_window,
                sel_registerName(b"makeKeyAndOrderFront:\0".as_ptr() as *const _),
                nil,
            );
        }
    }

    pub fn update_buffer(&mut self, pixels: &[u32], width: usize, height: usize) {
        unsafe {
            let size = pixels.len() * 4;
            let boxed_pixels = pixels.to_vec();
            let data_ptr = boxed_pixels.as_ptr() as *const std::ffi::c_void;
            std::mem::forget(boxed_pixels);

            let provider = CGDataProviderCreateWithData(
                std::ptr::null_mut(),
                data_ptr,
                size,
                Some(release_provider_data),
            );

            let color_space = CGColorSpaceCreateDeviceRGB();
            let bitmap_info = kCGImageAlphaNoneSkipFirst | kCGBitmapByteOrder32Little;

            let cg_image = CGImageCreate(
                width,
                height,
                8,
                32,
                width * 4,
                color_space,
                bitmap_info,
                provider,
                std::ptr::null(),
                false,
                0,
            );

            let layer_sel = sel_registerName(b"layer\0".as_ptr() as *const _);
            let layer = msg_send_id(self.ns_view, layer_sel);

            let set_contents_sel = sel_registerName(b"setContents:\0".as_ptr() as *const _);
            msg_send_id_id_void(layer, set_contents_sel, cg_image as id);

            CFRelease(cg_image as CFTypeRef);
            CFRelease(provider as CFTypeRef);
            CFRelease(color_space as CFTypeRef);
        }
    }

    pub fn backing_scale_factor(&self) -> f64 {
        unsafe {
            let scale_sel = sel_registerName(b"backingScaleFactor\0".as_ptr() as *const _);
            let scale_func: unsafe extern "C" fn(id, SEL) -> f64 =
                std::mem::transmute(objc_msgSend as *const std::ffi::c_void);
            scale_func(self.ns_window, scale_sel)
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        #[cfg(debug_assertions)]
        assert_main_thread();
        unsafe {
            msg_send_id_id_void(
                self.ns_window,
                sel_registerName(b"setDelegate:\0".as_ptr() as *const _),
                nil,
            );
            msg_send_id_bool_void(
                self.ns_window,
                sel_registerName(b"setReleasedWhenClosed:\0".as_ptr() as *const _),
                YES,
            );
            msg_send_id(
                self.ns_window,
                sel_registerName(b"close\0".as_ptr() as *const _),
            );
            msg_send_id(
                self.ns_delegate,
                sel_registerName(b"release\0".as_ptr() as *const _),
            );
        }
    }
}

unsafe extern "C" fn release_provider_data(
    _info: *mut std::ffi::c_void,
    data: *const std::ffi::c_void,
    size: usize,
) {
    unsafe {
        let _vec = Vec::from_raw_parts(data as *mut u32, size / 4, size / 4);
    }
}
