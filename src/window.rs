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
    pub ns_delegate: id,
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
                let screen_rect = objc::msg_send_rect(main_screen, frame_sel);
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

            // Create and set delegate
            let delegate_class = crate::event_loop::register_delegate_class();
            let delegate_alloc = objc::msg_send_id(delegate_class, ffi::sel_registerName(b"alloc\0".as_ptr() as *const _));
            let ns_delegate = objc::msg_send_id(delegate_alloc, ffi::sel_registerName(b"init\0".as_ptr() as *const _));
            
            objc::msg_send_id_id_void(
                ns_window,
                ffi::sel_registerName(b"setDelegate:\0".as_ptr() as *const _),
                ns_delegate,
            );

            Window { ns_window, ns_view, ns_delegate }
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

    pub fn update_buffer(&mut self, pixels: &[u32], width: usize, height: usize) {
        unsafe {
            let size = pixels.len() * 4;
            let boxed_pixels = pixels.to_vec();
            let data_ptr = boxed_pixels.as_ptr() as *const std::ffi::c_void;
            std::mem::forget(boxed_pixels);
            
            let provider = ffi::CGDataProviderCreateWithData(
                std::ptr::null_mut(),
                data_ptr,
                size,
                Some(release_provider_data),
            );
            
            let color_space = ffi::CGColorSpaceCreateDeviceRGB();
            let bitmap_info = ffi::kCGImageAlphaNoneSkipFirst | ffi::kCGBitmapByteOrder32Little;
            
            let cg_image = ffi::CGImageCreate(
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
            
            let layer_sel = ffi::sel_registerName(b"layer\0".as_ptr() as *const _);
            let layer = objc::msg_send_id(self.ns_view, layer_sel);
            
            let set_contents_sel = ffi::sel_registerName(b"setContents:\0".as_ptr() as *const _);
            objc::msg_send_id_id_void(layer, set_contents_sel, cg_image as id);
            
            ffi::CFRelease(cg_image as ffi::CFTypeRef);
            ffi::CFRelease(provider as ffi::CFTypeRef);
            ffi::CFRelease(color_space as ffi::CFTypeRef);
        }
    }

    pub fn backing_scale_factor(&self) -> f64 {
        unsafe {
            let scale_sel = ffi::sel_registerName(b"backingScaleFactor\0".as_ptr() as *const _);
            let scale_func: unsafe extern "C" fn(id, ffi::SEL) -> f64 =
                std::mem::transmute(ffi::objc_msgSend as *const std::ffi::c_void);
            scale_func(self.ns_window, scale_sel)
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
