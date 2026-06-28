#![allow(non_upper_case_globals)]

use crate::event::*;
use crate::ffi::*;
use crate::objc::*;
use crate::vsync::VsyncTracker;

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

pub struct Window {
    pub(crate) ns_window: id,
    pub(crate) ns_view: id,
    pub(crate) ns_delegate: id,
    vsync: VsyncTracker,
    _marker: std::marker::PhantomData<*mut ()>,
}

static APP_INIT: std::sync::Once = std::sync::Once::new();

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

                let set_policy_sel =
                    sel_registerName(b"setActivationPolicy:\0".as_ptr() as *const _);
                let set_policy: unsafe extern "C" fn(
                    id,
                    SEL,
                    NSApplicationActivationPolicy,
                ) -> BOOL = std::mem::transmute(objc_msgSend as *const std::ffi::c_void);
                set_policy(ns_app, set_policy_sel, NSApplicationActivationPolicyRegular);

                // Register delegate class
                register_delegate_class();

                // Setup menu bar
                let alloc_sel = sel_registerName(b"alloc\0".as_ptr() as *const _);
                let init_sel = sel_registerName(b"init\0".as_ptr() as *const _);

                let main_menu = msg_send_id(
                    msg_send_id(objc_getClass(b"NSMenu\0".as_ptr() as *const _), alloc_sel),
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
                    msg_send_id(objc_getClass(b"NSMenu\0".as_ptr() as *const _), alloc_sel),
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
            let content_view = msg_send_id(
                self.ns_window,
                sel_registerName(b"contentView\0".as_ptr() as *const _),
            );
            let frame = msg_send_rect(
                content_view,
                sel_registerName(b"frame\0".as_ptr() as *const _),
            );
            (frame.size.width, frame.size.height)
        }
    }

    pub fn wait_for_vsync(&self) {
        self.vsync.wait_for_vsync();
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
        let modifiers = Modifiers::parse(flags);

        match event_type {
            NSEventTypeKeyDown | NSEventTypeKeyUp => {
                let key_code_sel = sel_registerName(b"keyCode\0".as_ptr() as *const _);
                let key_code_func: unsafe extern "C" fn(id, SEL) -> u16 =
                    std::mem::transmute(objc_msgSend as *const std::ffi::c_void);
                let keycode = key_code_func(ns_event, key_code_sel);

                if event_type == NSEventTypeKeyDown {
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

                let window =
                    msg_send_id(ns_event, sel_registerName(b"window\0".as_ptr() as *const _));
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
                    NSEventTypeLeftMouseDown | NSEventTypeLeftMouseUp => MouseButton::Left,
                    NSEventTypeRightMouseDown | NSEventTypeRightMouseUp => MouseButton::Right,
                    _ => MouseButton::Left,
                };

                if event_type == NSEventTypeLeftMouseDown || event_type == NSEventTypeRightMouseDown
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

                let window =
                    msg_send_id(ns_event, sel_registerName(b"window\0".as_ptr() as *const _));
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

                let window =
                    msg_send_id(ns_event, sel_registerName(b"window\0".as_ptr() as *const _));
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
        cls = objc_allocateClassPair(superclass, b"RustWindowDelegate\0".as_ptr() as *const _, 0);

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
            let delegate =
                msg_send_id(window, sel_registerName(b"delegate\0".as_ptr() as *const _));
            let mut temp_window = std::mem::ManuallyDrop::new(crate::window::Window::from_raw(
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
