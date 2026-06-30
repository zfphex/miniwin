#![allow(non_upper_case_globals)]

use crate::event::*;
use crate::common::*;
use crate::ffi::*;
use crate::objc::*;
use crate::vsync::VsyncTracker;
use std::path::PathBuf;

thread_local! {
    pub(crate) static REPAINT_CALLBACK: std::cell::Cell<Option<*mut std::ffi::c_void>> = std::cell::Cell::new(None);
    pub(crate) static REPAINT_FUNC: std::cell::Cell<Option<unsafe fn(*mut std::ffi::c_void, &mut Window, usize, usize)>> = std::cell::Cell::new(None);
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorIcon {
    Arrow,
    IBeam,
    PointingHand,
    ClosedHand,
    OpenHand,
    Crosshair,
    ResizeLeftRight,
    ResizeUpDown,
}

pub struct Window {
    pub(crate) ns_window: id,
    pub(crate) ns_view: id,
    pub(crate) ns_delegate: id,
    vsync: VsyncTracker,
    _marker: std::marker::PhantomData<*mut ()>,
}

static APP_INIT: std::sync::Once = std::sync::Once::new();

pub fn create_window(
    title: &str,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    style: WindowStyle,
) -> std::pin::Pin<Box<Window>> {
    Box::pin(Window::new(title, width as f64, height as f64, style, FullscreenMode::None))
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
            APP_INIT.call_once(|| {
                let ns_app = msg_send_id(
                    objc_getClass(b"NSApplication\0".as_ptr() as *const _),
                    sel_registerName(b"sharedApplication\0".as_ptr() as *const _),
                );

                let set_policy_sel = sel_registerName(b"setActivationPolicy:\0".as_ptr() as *const _);
                let set_policy: unsafe extern "C" fn(
                    id,
                    SEL,
                    NSApplicationActivationPolicy,
                ) -> BOOL = std::mem::transmute(objc_msgSend as *const std::ffi::c_void);
                set_policy(
                    ns_app,
                    set_policy_sel,
                    NSApplicationActivationPolicyRegular,
                );

                // Register delegate and view classes
                register_delegate_class();
                register_view_class();

                // Setup menu bar
                let alloc_sel = sel_registerName(b"alloc\0".as_ptr() as *const _);
                let init_sel = sel_registerName(b"init\0".as_ptr() as *const _);

                let main_menu = msg_send_id(
                    msg_send_id(
                        objc_getClass(b"NSMenu\0".as_ptr() as *const _),
                        alloc_sel,
                    ),
                    init_sel,
                );

                let app_menu_item = msg_send_id(
                    msg_send_id(
                        objc_getClass(b"NSMenuItem\0".as_ptr() as *const _),
                        alloc_sel,
                    ),
                    init_sel,
                );

                msg_send_id_id_void(
                    main_menu,
                    sel_registerName(b"addItem:\0".as_ptr() as *const _),
                    app_menu_item,
                );

                let app_menu = msg_send_id(
                    msg_send_id(
                        objc_getClass(b"NSMenu\0".as_ptr() as *const _),
                        alloc_sel,
                    ),
                    init_sel,
                );

                msg_send_id_id_void(
                    app_menu_item,
                    sel_registerName(b"setSubmenu:\0".as_ptr() as *const _),
                    app_menu,
                );

                let quit_title = nsstring("Quit");
                let quit_sel = sel_registerName(b"terminate:\0".as_ptr() as *const _);
                let key = nsstring("q");
                let quit_item_alloc = msg_send_id(
                    objc_getClass(b"NSMenuItem\0".as_ptr() as *const _),
                    alloc_sel,
                );

                let init_quit_func: unsafe extern "C" fn(id, SEL, id, SEL, id) -> id =
                    std::mem::transmute(objc_msgSend as *const std::ffi::c_void);
                let quit_item = init_quit_func(
                    quit_item_alloc,
                    sel_registerName(b"initWithTitle:action:keyEquivalent:\0".as_ptr() as *const _),
                    quit_title,
                    quit_sel,
                    key,
                );

                msg_send_id_id_void(
                    app_menu,
                    sel_registerName(b"addItem:\0".as_ptr() as *const _),
                    quit_item,
                );

                msg_send_id_id_void(
                    ns_app,
                    sel_registerName(b"setMainMenu:\0".as_ptr() as *const _),
                    main_menu,
                );

                // Finish launching
                msg_send_id(
                    ns_app,
                    sel_registerName(b"finishLaunching\0".as_ptr() as *const _),
                );
            });

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
                WindowStyle::Borderless | WindowStyle::Transparent => {
                    NSWindowStyleMaskBorderless
                }
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

            // Instantiate our custom RustView class
            let view_class = register_view_class();
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

            // Register content view for Drag & Drop file drops
            let pb_type = nsstring("public.file-url");
            let array_class = objc_getClass(b"NSArray\0".as_ptr() as *const _);
            let array_sel = sel_registerName(b"arrayWithObject:\0".as_ptr() as *const _);
            let array_func: unsafe extern "C" fn(id, SEL, id) -> id = std::mem::transmute(objc_msgSend as *const std::ffi::c_void);
            let types_array = array_func(array_class, array_sel, pb_type);

            let register_sel = sel_registerName(b"registerForDraggedTypes:\0".as_ptr() as *const _);
            let register_func: unsafe extern "C" fn(id, SEL, id) = std::mem::transmute(objc_msgSend as *const std::ffi::c_void);
            register_func(ns_view, register_sel, types_array);

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

            msg_send_id_id_void(
                ns_window,
                sel_registerName(b"makeKeyAndOrderFront:\0".as_ptr() as *const _),
                nil,
            );

            let vsync = VsyncTracker::new();

            Window {
                ns_window,
                ns_view,
                ns_delegate,
                vsync,
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

    pub fn content_size(&self) -> (f64, f64) {
        unsafe {
            let content_view = msg_send_id(self.ns_window, sel_registerName(b"contentView\0".as_ptr() as *const _));
            let frame = msg_send_rect(content_view, sel_registerName(b"frame\0".as_ptr() as *const _));
            (frame.size.width, frame.size.height)
        }
    }

    pub fn wait_for_vsync(&self) {
        self.vsync.wait_for_vsync();
    }

    pub fn set_cursor_visible(&self, visible: bool) {
        unsafe {
            let ns_cursor = objc_getClass(b"NSCursor\0".as_ptr() as *const _);
            let sel = if visible {
                sel_registerName(b"unhide\0".as_ptr() as *const _)
            } else {
                sel_registerName(b"hide\0".as_ptr() as *const _)
            };
            msg_send_id(ns_cursor, sel);
        }
    }

    pub fn set_cursor_grab(&self, grab: bool) {
        unsafe {
            CGAssociateMouseAndMouseCursorPosition(!grab);
        }
    }

    pub fn set_cursor_icon(&self, icon: CursorIcon) {
        unsafe {
            let ns_cursor = objc_getClass(b"NSCursor\0".as_ptr() as *const _);
            let selector = match icon {
                CursorIcon::Arrow => b"arrowCursor\0".as_ptr(),
                CursorIcon::IBeam => b"IBeamCursor\0".as_ptr(),
                CursorIcon::PointingHand => b"pointingHandCursor\0".as_ptr(),
                CursorIcon::ClosedHand => b"closedHandCursor\0".as_ptr(),
                CursorIcon::OpenHand => b"openHandCursor\0".as_ptr(),
                CursorIcon::Crosshair => b"crosshairCursor\0".as_ptr(),
                CursorIcon::ResizeLeftRight => b"resizeLeftRightCursor\0".as_ptr(),
                CursorIcon::ResizeUpDown => b"resizeUpDownCursor\0".as_ptr(),
            };
            let cursor_sel = sel_registerName(selector as *const _);
            let cursor = msg_send_id(ns_cursor, cursor_sel);
            if !cursor.is_null() {
                msg_send_id(cursor, sel_registerName(b"set\0".as_ptr() as *const _));
            }
        }
    }

    pub fn get_clipboard_text(&self) -> Option<String> {
        unsafe {
            let pb_class = objc_getClass(b"NSPasteboard\0".as_ptr() as *const _);
            let pb = msg_send_id(pb_class, sel_registerName(b"generalPasteboard\0".as_ptr() as *const _));
            if pb.is_null() {
                return None;
            }

            let type_ns = nsstring("public.utf8-plain-text");
            let string_sel = sel_registerName(b"stringForType:\0".as_ptr() as *const _);
            let string_func: unsafe extern "C" fn(id, SEL, id) -> id = std::mem::transmute(objc_msgSend as *const std::ffi::c_void);
            let ns_string = string_func(pb, string_sel, type_ns);

            if ns_string.is_null() {
                return None;
            }

            let utf8_func: unsafe extern "C" fn(id, SEL) -> *const std::os::raw::c_char = std::mem::transmute(objc_msgSend as *const std::ffi::c_void);
            let utf8_ptr = utf8_func(ns_string, sel_registerName(b"UTF8String\0".as_ptr() as *const _));
            if utf8_ptr.is_null() {
                return None;
            }

            let c_str = std::ffi::CStr::from_ptr(utf8_ptr);
            c_str.to_str().ok().map(|s| s.to_string())
        }
    }

    pub fn set_clipboard_text(&self, text: &str) {
        unsafe {
            let pb_class = objc_getClass(b"NSPasteboard\0".as_ptr() as *const _);
            let pb = msg_send_id(pb_class, sel_registerName(b"generalPasteboard\0".as_ptr() as *const _));
            if pb.is_null() {
                return;
            }

            msg_send_id(pb, sel_registerName(b"clearContents\0".as_ptr() as *const _));

            let type_ns = nsstring("public.utf8-plain-text");
            let text_ns = nsstring(text);

            let set_sel = sel_registerName(b"setString:forType:\0".as_ptr() as *const _);
            let set_func: unsafe extern "C" fn(id, SEL, id, id) -> BOOL = std::mem::transmute(objc_msgSend as *const std::ffi::c_void);
            set_func(pb, set_sel, text_ns, type_ns);
        }
    }

    pub fn poll_events<F>(&mut self, mut repaint: F) -> Vec<Event>
    where
        F: FnMut(&mut Window, usize, usize),
    {
        #[cfg(debug_assertions)]
        assert_main_thread();

        // Store the closure in thread-local storage
        let repaint_ptr = &mut repaint as *mut F as *mut std::ffi::c_void;
        let repaint_func = |ptr: *mut std::ffi::c_void, window: &mut Window, w: usize, h: usize| unsafe {
            let f = &mut *(ptr as *mut F);
            f(window, w, h);
        };

        REPAINT_CALLBACK.with(|c| c.set(Some(repaint_ptr)));
        REPAINT_FUNC.with(|f| f.set(Some(repaint_func)));

        let mut events = Vec::new();
        unsafe {
            let ns_app = msg_send_id(
                objc_getClass(b"NSApplication\0".as_ptr() as *const _),
                sel_registerName(b"sharedApplication\0".as_ptr() as *const _),
            );

            // Allocate an autorelease pool for this tick
            let pool_class = objc_getClass(b"NSAutoreleasePool\0".as_ptr() as *const _);
            let pool = msg_send_id(
                msg_send_id(
                    pool_class,
                    sel_registerName(b"alloc\0".as_ptr() as *const _),
                ),
                sel_registerName(b"init\0".as_ptr() as *const _),
            );

            let date_class = objc_getClass(b"NSDate\0".as_ptr() as *const _);
            let distant_past = msg_send_id(
                date_class,
                sel_registerName(b"distantPast\0".as_ptr() as *const _),
            );

            let mode = nsstring("kCFRunLoopDefaultMode");

            let next_event_sel = sel_registerName(
                b"nextEventMatchingMask:untilDate:inMode:dequeue:\0".as_ptr() as *const _,
            );

            let next_event_func: unsafe extern "C" fn(id, SEL, u64, id, id, BOOL) -> id =
                std::mem::transmute(objc_msgSend as *const std::ffi::c_void);

            loop {
                let event = next_event_func(
                    ns_app,
                    next_event_sel,
                    NSEventMaskAny,
                    distant_past,
                    mode,
                    YES,
                );

                if event.is_null() {
                    break;
                }

                if let Some(ev) = translate_event(event) {
                    events.push(ev);
                }

                // Dispatch event to targets
                msg_send_id_id_void(
                    ns_app,
                    sel_registerName(b"sendEvent:\0".as_ptr() as *const _),
                    event,
                );
            }

            // Release the autorelease pool
            msg_send_id(pool, sel_registerName(b"drain\0".as_ptr() as *const _));
        }

        // Clear thread-local storage
        REPAINT_CALLBACK.with(|c| c.set(None));
        REPAINT_FUNC.with(|f| f.set(None));

        // Append any events captured by the delegate callbacks (like close/resize)
        events.extend(pop_all_events());
        events
    }

    pub(crate) unsafe fn from_raw(ns_window: id, ns_view: id, ns_delegate: id) -> Self {
        Window {
            ns_window,
            ns_view,
            ns_delegate,
            vsync: VsyncTracker::new(),
            _marker: std::marker::PhantomData,
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

fn parse_modifiers(flags: usize) -> Modifiers {
    Modifiers {
        shift: (flags & (1 << 17)) != 0,
        ctrl: (flags & (1 << 18)) != 0,
        alt: (flags & (1 << 19)) != 0,
        logo: (flags & (1 << 20)) != 0,
    }
}

unsafe fn translate_event(ns_event: id) -> Option<Event> {
    unsafe {
        let event_type_sel = sel_registerName(b"type\0".as_ptr() as *const _);
        let event_type_func: unsafe extern "C" fn(id, SEL) -> NSEventType =
            std::mem::transmute(objc_msgSend as *const std::ffi::c_void);
        let event_type = event_type_func(ns_event, event_type_sel);

        let modifier_flags_sel = sel_registerName(b"modifierFlags\0".as_ptr() as *const _);
        let modifier_flags_func: unsafe extern "C" fn(id, SEL) -> usize =
            std::mem::transmute(objc_msgSend as *const std::ffi::c_void);
        let flags = modifier_flags_func(ns_event, modifier_flags_sel);
        let modifiers = parse_modifiers(flags);

        match event_type {
            NSEventTypeKeyDown | NSEventTypeKeyUp => {
                let key_code_sel = sel_registerName(b"keyCode\0".as_ptr() as *const _);
                let key_code_func: unsafe extern "C" fn(id, SEL) -> u16 =
                    std::mem::transmute(objc_msgSend as *const std::ffi::c_void);
                let keycode = key_code_func(ns_event, key_code_sel);

                if event_type == NSEventTypeKeyDown {
                    // Extract text input characters
                    let chars_ns = msg_send_id(ns_event, sel_registerName(b"characters\0".as_ptr() as *const _));
                    if !chars_ns.is_null() {
                        let len_func: unsafe extern "C" fn(id, SEL) -> usize = std::mem::transmute(objc_msgSend as *const std::ffi::c_void);
                        let len = len_func(chars_ns, sel_registerName(b"length\0".as_ptr() as *const _));
                        if len > 0 {
                            let utf8_func: unsafe extern "C" fn(id, SEL) -> *const std::os::raw::c_char = std::mem::transmute(objc_msgSend as *const std::ffi::c_void);
                            let utf8_ptr = utf8_func(chars_ns, sel_registerName(b"UTF8String\0".as_ptr() as *const _));
                            if !utf8_ptr.is_null() {
                                let c_str = std::ffi::CStr::from_ptr(utf8_ptr);
                                if let Ok(s) = c_str.to_str() {
                                    for c in s.chars() {
                                        if !c.is_control() {
                                            push_event(Event::ReceivedCharacter(c));
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Some(Event::KeyDown { keycode, modifiers })
                } else {
                    Some(Event::KeyUp { keycode, modifiers })
                }
            }
            NSEventTypeLeftMouseDown
            | NSEventTypeLeftMouseUp
            | NSEventTypeRightMouseDown
            | NSEventTypeRightMouseUp => {
                let loc_sel = sel_registerName(b"locationInWindow\0".as_ptr() as *const _);
                let loc_func: unsafe extern "C" fn(id, SEL) -> NSPoint =
                    std::mem::transmute(objc_msgSend as *const std::ffi::c_void);
                let loc = loc_func(ns_event, loc_sel);

                let x = loc.x;
                let mut y = loc.y;

                let window = msg_send_id(
                    ns_event,
                    sel_registerName(b"window\0".as_ptr() as *const _),
                );
                if !window.is_null() {
                    let content_view = msg_send_id(
                        window,
                        sel_registerName(b"contentView\0".as_ptr() as *const _),
                    );
                    let frame_sel = sel_registerName(b"frame\0".as_ptr() as *const _);
                    let frame = msg_send_rect(content_view, frame_sel);
                    y = frame.size.height - loc.y;
                }

                let button = match event_type {
                    NSEventTypeLeftMouseDown | NSEventTypeLeftMouseUp => {
                        MouseButton::Left
                    }
                    NSEventTypeRightMouseDown | NSEventTypeRightMouseUp => {
                        MouseButton::Right
                    }
                    _ => MouseButton::Left,
                };

                if event_type == NSEventTypeLeftMouseDown
                    || event_type == NSEventTypeRightMouseDown
                {
                    Some(Event::MouseDown {
                        button,
                        x,
                        y,
                        modifiers,
                    })
                } else {
                    Some(Event::MouseUp {
                        button,
                        x,
                        y,
                        modifiers,
                    })
                }
            }
            NSEventTypeMouseMoved => {
                let loc_sel = sel_registerName(b"locationInWindow\0".as_ptr() as *const _);
                let loc_func: unsafe extern "C" fn(id, SEL) -> NSPoint =
                    std::mem::transmute(objc_msgSend as *const std::ffi::c_void);
                let loc = loc_func(ns_event, loc_sel);

                let x = loc.x;
                let mut y = loc.y;

                let window = msg_send_id(
                    ns_event,
                    sel_registerName(b"window\0".as_ptr() as *const _),
                );
                if !window.is_null() {
                    let content_view = msg_send_id(
                        window,
                        sel_registerName(b"contentView\0".as_ptr() as *const _),
                    );
                    let frame_sel = sel_registerName(b"frame\0".as_ptr() as *const _);
                    let frame = msg_send_rect(content_view, frame_sel);
                    y = frame.size.height - loc.y;
                }
                Some(Event::MouseMoved { x, y, modifiers })
            }
            NSEventTypeLeftMouseDragged | NSEventTypeRightMouseDragged => {
                let loc_sel = sel_registerName(b"locationInWindow\0".as_ptr() as *const _);
                let loc_func: unsafe extern "C" fn(id, SEL) -> NSPoint =
                    std::mem::transmute(objc_msgSend as *const std::ffi::c_void);
                let loc = loc_func(ns_event, loc_sel);

                let x = loc.x;
                let mut y = loc.y;

                let window = msg_send_id(
                    ns_event,
                    sel_registerName(b"window\0".as_ptr() as *const _),
                );
                if !window.is_null() {
                    let content_view = msg_send_id(
                        window,
                        sel_registerName(b"contentView\0".as_ptr() as *const _),
                    );
                    let frame_sel = sel_registerName(b"frame\0".as_ptr() as *const _);
                    let frame = msg_send_rect(content_view, frame_sel);
                    y = frame.size.height - loc.y;
                }
                let button = if event_type == NSEventTypeLeftMouseDragged {
                    MouseButton::Left
                } else {
                    MouseButton::Right
                };
                Some(Event::MouseDragged {
                    button,
                    x,
                    y,
                    modifiers,
                })
            }
            NSEventTypeScrollWheel => {
                let dx_sel = sel_registerName(b"scrollingDeltaX\0".as_ptr() as *const _);
                let dy_sel = sel_registerName(b"scrollingDeltaY\0".as_ptr() as *const _);
                let double_func: unsafe extern "C" fn(id, SEL) -> f64 =
                    std::mem::transmute(objc_msgSend as *const std::ffi::c_void);
                let delta_x = double_func(ns_event, dx_sel);
                let delta_y = double_func(ns_event, dy_sel);
                Some(Event::Scroll {
                    delta_x,
                    delta_y,
                    modifiers,
                })
            }
            _ => None,
        }
    }
}

static REGISTER_DELEGATE: std::sync::Once = std::sync::Once::new();

pub fn register_delegate_class() -> Class {
    let mut cls = std::ptr::null_mut();
    REGISTER_DELEGATE.call_once(|| unsafe {
        let superclass = objc_getClass(b"NSObject\0".as_ptr() as *const _);
        cls = objc_allocateClassPair(
            superclass,
            b"RustWindowDelegate\0".as_ptr() as *const _,
            0,
        );

        class_addMethod(
            cls,
            sel_registerName(b"windowShouldClose:\0".as_ptr() as *const _),
            std::mem::transmute(window_should_close as *const std::ffi::c_void),
            b"c@:@\0".as_ptr() as *const _,
        );

        class_addMethod(
            cls,
            sel_registerName(b"windowDidResize:\0".as_ptr() as *const _),
            std::mem::transmute(window_did_resize as *const std::ffi::c_void),
            b"v@:@\0".as_ptr() as *const _,
        );

        objc_registerClassPair(cls);
    });
    if cls.is_null() {
        unsafe { objc_getClass(b"RustWindowDelegate\0".as_ptr() as *const _) }
    } else {
        cls
    }
}

extern "C" fn window_should_close(_this: id, _cmd: SEL, _sender: id) -> BOOL {
    push_event(Event::CloseRequested);
    YES
}

extern "C" fn window_did_resize(_this: id, _cmd: SEL, notification: id) {
    unsafe {
        let window: id = msg_send_id(
            notification,
            sel_registerName(b"object\0".as_ptr() as *const _),
        );
        let content_view = msg_send_id(
            window,
            sel_registerName(b"contentView\0".as_ptr() as *const _),
        );
        let frame_sel = sel_registerName(b"frame\0".as_ptr() as *const _);
        let frame = msg_send_rect(content_view, frame_sel);

        let scale_sel = sel_registerName(b"backingScaleFactor\0".as_ptr() as *const _);
        let scale_func: unsafe extern "C" fn(id, SEL) -> f64 =
            std::mem::transmute(objc_msgSend as *const std::ffi::c_void);
        let scale = scale_func(window, scale_sel);

        let physical_width = (frame.size.width * scale) as usize;
        let physical_height = (frame.size.height * scale) as usize;

        // Retrieve and execute the repaint closure via thread-locals
        let callback_opt = REPAINT_CALLBACK.with(|c| c.get());
        let func_opt = REPAINT_FUNC.with(|f| f.get());

        if let (Some(ptr), Some(func)) = (callback_opt, func_opt) {
            let delegate = msg_send_id(window, sel_registerName(b"delegate\0".as_ptr() as *const _));
            let mut temp_window = std::mem::ManuallyDrop::new(Window::from_raw(
                window,
                content_view,
                delegate,
            ));
            func(ptr, &mut temp_window, physical_width, physical_height);
        }

        push_event(Event::Resized {
            width: frame.size.width,
            height: frame.size.height,
            physical_width,
            physical_height,
        });
    }
}

static REGISTER_VIEW: std::sync::Once = std::sync::Once::new();

pub fn register_view_class() -> Class {
    let mut cls = std::ptr::null_mut();
    REGISTER_VIEW.call_once(|| unsafe {
        let superclass = objc_getClass(b"NSView\0".as_ptr() as *const _);
        cls = objc_allocateClassPair(
            superclass,
            b"RustView\0".as_ptr() as *const _,
            0,
        );

        // Bind drag-and-drop destination methods directly to our custom view class
        class_addMethod(
            cls,
            sel_registerName(b"draggingEntered:\0".as_ptr() as *const _),
            std::mem::transmute(dragging_entered as *const std::ffi::c_void),
            b"Q@:@\0".as_ptr() as *const _,
        );

        class_addMethod(
            cls,
            sel_registerName(b"performDragOperation:\0".as_ptr() as *const _),
            std::mem::transmute(perform_drag_operation as *const std::ffi::c_void),
            b"c@:@\0".as_ptr() as *const _,
        );

        objc_registerClassPair(cls);
    });

    if cls.is_null() {
        unsafe { objc_getClass(b"RustView\0".as_ptr() as *const _) }
    } else {
        cls
    }
}

extern "C" fn dragging_entered(_this: id, _cmd: SEL, _sender: id) -> usize {
    1 // NSDragOperationGeneric
}

extern "C" fn perform_drag_operation(_this: id, _cmd: SEL, sender: id) -> BOOL {
    unsafe {
        let pb_sel = sel_registerName(b"draggingPasteboard\0".as_ptr() as *const _);
        let pb = msg_send_id(sender, pb_sel);
        if pb.is_null() {
            return NO;
        }

        // Read file URLs from the pasteboard
        let url_class = objc_getClass(b"NSURL\0".as_ptr() as *const _);
        let class_array_class = objc_getClass(b"NSArray\0".as_ptr() as *const _);
        let array_sel = sel_registerName(b"arrayWithObject:\0".as_ptr() as *const _);
        let array_func: unsafe extern "C" fn(id, SEL, id) -> id = std::mem::transmute(objc_msgSend as *const std::ffi::c_void);
        let classes = array_func(class_array_class, array_sel, url_class);

        let read_sel = sel_registerName(b"readObjectsForClasses:options:\0".as_ptr() as *const _);
        let read_func: unsafe extern "C" fn(id, SEL, id, id) -> id = std::mem::transmute(objc_msgSend as *const std::ffi::c_void);
        let urls = read_func(pb, read_sel, classes, std::ptr::null_mut());

        if urls.is_null() {
            return NO;
        }

        let count_sel = sel_registerName(b"count\0".as_ptr() as *const _);
        let count_func: unsafe extern "C" fn(id, SEL) -> usize = std::mem::transmute(objc_msgSend as *const std::ffi::c_void);
        let count = count_func(urls, count_sel);

        let mut file_paths = Vec::new();
        let object_at_index_sel = sel_registerName(b"objectAtIndex:\0".as_ptr() as *const _);
        let object_func: unsafe extern "C" fn(id, SEL, usize) -> id = std::mem::transmute(objc_msgSend as *const std::ffi::c_void);

        for i in 0..count {
            let url = object_func(urls, object_at_index_sel, i);
            if !url.is_null() {
                let path_sel = sel_registerName(b"path\0".as_ptr() as *const _);
                let path_ns = msg_send_id(url, path_sel);
                if !path_ns.is_null() {
                    let utf8_func: unsafe extern "C" fn(id, SEL) -> *const std::os::raw::c_char = std::mem::transmute(objc_msgSend as *const std::ffi::c_void);
                    let utf8_ptr = utf8_func(path_ns, sel_registerName(b"UTF8String\0".as_ptr() as *const _));
                    if !utf8_ptr.is_null() {
                        let c_str = std::ffi::CStr::from_ptr(utf8_ptr);
                        if let Ok(s) = c_str.to_str() {
                            file_paths.push(PathBuf::from(s));
                        }
                    }
                }
            }
        }

        if !file_paths.is_empty() {
            push_event(Event::DroppedFiles(file_paths));
            YES
        } else {
            NO
        }
    }
}

pub fn assert_main_thread() {
    unsafe {
        let thread_class = objc_getClass(b"NSThread\0".as_ptr() as *const _);
        let is_main_sel = sel_registerName(b"isMainThread\0".as_ptr() as *const _);
        let func: unsafe extern "C" fn(id, SEL) -> BOOL =
            std::mem::transmute(objc_msgSend as *const std::ffi::c_void);
        let is_main = func(thread_class, is_main_sel);
        if is_main == NO {
            panic!("AppKit functions must be called from the main thread!");
        }
    }
}
