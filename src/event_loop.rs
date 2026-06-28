use crate::ffi::{self, id, Class, YES};
use crate::objc;
use crate::event::{Event, MouseButton, Modifiers, push_event, pop_all_events};

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
            
            let set_policy_sel = ffi::sel_registerName(b"setActivationPolicy:\0".as_ptr() as *const _);
            let set_policy: unsafe extern "C" fn(id, ffi::SEL, ffi::NSApplicationActivationPolicy) -> ffi::BOOL =
                std::mem::transmute(ffi::objc_msgSend as *const std::ffi::c_void);
            set_policy(ns_app, set_policy_sel, ffi::NSApplicationActivationPolicyRegular);
            
            // Register delegate class
            register_delegate_class();
            
            Self::setup_menu_bar(ns_app);
            
            // Finish launching the application to make it active
            objc::msg_send_id(ns_app, ffi::sel_registerName(b"finishLaunching\0".as_ptr() as *const _));
            
            EventLoop { ns_app }
        }
    }
    
    unsafe fn setup_menu_bar(ns_app: id) {
        unsafe {
            let alloc_sel = ffi::sel_registerName(b"alloc\0".as_ptr() as *const _);
            let init_sel = ffi::sel_registerName(b"init\0".as_ptr() as *const _);
            
            let main_menu = objc::msg_send_id(
                objc::msg_send_id(ffi::objc_getClass(b"NSMenu\0".as_ptr() as *const _), alloc_sel),
                init_sel,
            );
            
            let app_menu_item = objc::msg_send_id(
                objc::msg_send_id(ffi::objc_getClass(b"NSMenuItem\0".as_ptr() as *const _), alloc_sel),
                init_sel,
            );
            
            objc::msg_send_id_id_void(main_menu, ffi::sel_registerName(b"addItem:\0".as_ptr() as *const _), app_menu_item);
            
            let app_menu = objc::msg_send_id(
                objc::msg_send_id(ffi::objc_getClass(b"NSMenu\0".as_ptr() as *const _), alloc_sel),
                init_sel,
            );
            
            objc::msg_send_id_id_void(app_menu_item, ffi::sel_registerName(b"setSubmenu:\0".as_ptr() as *const _), app_menu);
            
            let quit_title = objc::nsstring("Quit");
            let quit_sel = ffi::sel_registerName(b"terminate:\0".as_ptr() as *const _);
            let key = objc::nsstring("q");
            let quit_item_alloc = objc::msg_send_id(ffi::objc_getClass(b"NSMenuItem\0".as_ptr() as *const _), alloc_sel);
            
            let init_quit_func: unsafe extern "C" fn(id, ffi::SEL, id, ffi::SEL, id) -> id =
                std::mem::transmute(ffi::objc_msgSend as *const std::ffi::c_void);
            let quit_item = init_quit_func(
                quit_item_alloc,
                ffi::sel_registerName(b"initWithTitle:action:keyEquivalent:\0".as_ptr() as *const _),
                quit_title,
                quit_sel,
                key,
            );
            
            objc::msg_send_id_id_void(app_menu, ffi::sel_registerName(b"addItem:\0".as_ptr() as *const _), quit_item);
            
            objc::msg_send_id_id_void(ns_app, ffi::sel_registerName(b"setMainMenu:\0".as_ptr() as *const _), main_menu);
        }
    }
    
    pub fn poll_events(&self) -> Vec<Event> {
        let mut events = Vec::new();
        unsafe {
            // Allocate an autorelease pool for this tick
            let pool_class = ffi::objc_getClass(b"NSAutoreleasePool\0".as_ptr() as *const _);
            let pool = objc::msg_send_id(
                objc::msg_send_id(pool_class, ffi::sel_registerName(b"alloc\0".as_ptr() as *const _)),
                ffi::sel_registerName(b"init\0".as_ptr() as *const _),
            );
            
            let date_class = ffi::objc_getClass(b"NSDate\0".as_ptr() as *const _);
            let distant_past = objc::msg_send_id(
                date_class,
                ffi::sel_registerName(b"distantPast\0".as_ptr() as *const _),
            );
            
            let mode = objc::nsstring("kCFRunLoopDefaultMode");
            
            let next_event_sel = ffi::sel_registerName(
                b"nextEventMatchingMask:untilDate:inMode:dequeue:\0".as_ptr() as *const _,
            );
            
            let next_event_func: unsafe extern "C" fn(
                id,
                ffi::SEL,
                u64,
                id,
                id,
                ffi::BOOL,
            ) -> id = std::mem::transmute(ffi::objc_msgSend as *const std::ffi::c_void);
            
            loop {
                let event = next_event_func(
                    self.ns_app,
                    next_event_sel,
                    ffi::NSEventMaskAny,
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
                objc::msg_send_id_id_void(
                    self.ns_app,
                    ffi::sel_registerName(b"sendEvent:\0".as_ptr() as *const _),
                    event,
                );
            }
            
            // Release the autorelease pool
            objc::msg_send_id(pool, ffi::sel_registerName(b"drain\0".as_ptr() as *const _));
        }
        
        // Append any events captured by the delegate callbacks (like close/resize)
        events.extend(pop_all_events());
        events
    }
}

unsafe fn translate_event(ns_event: id) -> Option<Event> {
    unsafe {
        let event_type_sel = ffi::sel_registerName(b"type\0".as_ptr() as *const _);
        let event_type_func: unsafe extern "C" fn(id, ffi::SEL) -> ffi::NSEventType =
            std::mem::transmute(ffi::objc_msgSend as *const std::ffi::c_void);
        let event_type = event_type_func(ns_event, event_type_sel);
        
        let modifier_flags_sel = ffi::sel_registerName(b"modifierFlags\0".as_ptr() as *const _);
        let modifier_flags_func: unsafe extern "C" fn(id, ffi::SEL) -> usize =
            std::mem::transmute(ffi::objc_msgSend as *const std::ffi::c_void);
        let flags = modifier_flags_func(ns_event, modifier_flags_sel);
        let modifiers = Modifiers::parse(flags);

        match event_type {
            ffi::NSEventTypeKeyDown | ffi::NSEventTypeKeyUp => {
                let key_code_sel = ffi::sel_registerName(b"keyCode\0".as_ptr() as *const _);
                let key_code_func: unsafe extern "C" fn(id, ffi::SEL) -> u16 =
                    std::mem::transmute(ffi::objc_msgSend as *const std::ffi::c_void);
                let keycode = key_code_func(ns_event, key_code_sel);
                
                if event_type == ffi::NSEventTypeKeyDown {
                    Some(Event::KeyDown { keycode, modifiers })
                } else {
                    Some(Event::KeyUp { keycode, modifiers })
                }
            }
            ffi::NSEventTypeLeftMouseDown | ffi::NSEventTypeLeftMouseUp
            | ffi::NSEventTypeRightMouseDown | ffi::NSEventTypeRightMouseUp => {
                let loc_sel = ffi::sel_registerName(b"locationInWindow\0".as_ptr() as *const _);
                let loc_func: unsafe extern "C" fn(id, ffi::SEL) -> ffi::NSPoint =
                    std::mem::transmute(ffi::objc_msgSend as *const std::ffi::c_void);
                let loc = loc_func(ns_event, loc_sel);
                
                let x = loc.x;
                let mut y = loc.y;
                
                let window = objc::msg_send_id(ns_event, ffi::sel_registerName(b"window\0".as_ptr() as *const _));
                if !window.is_null() {
                    let content_view = objc::msg_send_id(window, ffi::sel_registerName(b"contentView\0".as_ptr() as *const _));
                    let frame_sel = ffi::sel_registerName(b"frame\0".as_ptr() as *const _);
                    let frame = objc::msg_send_rect(content_view, frame_sel);
                    y = frame.size.height - loc.y;
                }
                
                let button = match event_type {
                    ffi::NSEventTypeLeftMouseDown | ffi::NSEventTypeLeftMouseUp => MouseButton::Left,
                    ffi::NSEventTypeRightMouseDown | ffi::NSEventTypeRightMouseUp => MouseButton::Right,
                    _ => MouseButton::Left,
                };
                
                if event_type == ffi::NSEventTypeLeftMouseDown || event_type == ffi::NSEventTypeRightMouseDown {
                    Some(Event::MouseDown { button, x, y, modifiers })
                } else {
                    Some(Event::MouseUp { button, x, y, modifiers })
                }
            }
            ffi::NSEventTypeMouseMoved => {
                let loc_sel = ffi::sel_registerName(b"locationInWindow\0".as_ptr() as *const _);
                let loc_func: unsafe extern "C" fn(id, ffi::SEL) -> ffi::NSPoint =
                    std::mem::transmute(ffi::objc_msgSend as *const std::ffi::c_void);
                let loc = loc_func(ns_event, loc_sel);
                
                let x = loc.x;
                let mut y = loc.y;
                
                let window = objc::msg_send_id(ns_event, ffi::sel_registerName(b"window\0".as_ptr() as *const _));
                if !window.is_null() {
                    let content_view = objc::msg_send_id(window, ffi::sel_registerName(b"contentView\0".as_ptr() as *const _));
                    let frame_sel = ffi::sel_registerName(b"frame\0".as_ptr() as *const _);
                    let frame = objc::msg_send_rect(content_view, frame_sel);
                    y = frame.size.height - loc.y;
                }
                Some(Event::MouseMoved { x, y, modifiers })
            }
            ffi::NSEventTypeLeftMouseDragged | ffi::NSEventTypeRightMouseDragged => {
                let loc_sel = ffi::sel_registerName(b"locationInWindow\0".as_ptr() as *const _);
                let loc_func: unsafe extern "C" fn(id, ffi::SEL) -> ffi::NSPoint =
                    std::mem::transmute(ffi::objc_msgSend as *const std::ffi::c_void);
                let loc = loc_func(ns_event, loc_sel);
                
                let x = loc.x;
                let mut y = loc.y;
                
                let window = objc::msg_send_id(ns_event, ffi::sel_registerName(b"window\0".as_ptr() as *const _));
                if !window.is_null() {
                    let content_view = objc::msg_send_id(window, ffi::sel_registerName(b"contentView\0".as_ptr() as *const _));
                    let frame_sel = ffi::sel_registerName(b"frame\0".as_ptr() as *const _);
                    let frame = objc::msg_send_rect(content_view, frame_sel);
                    y = frame.size.height - loc.y;
                }
                let button = if event_type == ffi::NSEventTypeLeftMouseDragged {
                    MouseButton::Left
                } else {
                    MouseButton::Right
                };
                Some(Event::MouseDragged { button, x, y, modifiers })
            }
            ffi::NSEventTypeScrollWheel => {
                let dx_sel = ffi::sel_registerName(b"scrollingDeltaX\0".as_ptr() as *const _);
                let dy_sel = ffi::sel_registerName(b"scrollingDeltaY\0".as_ptr() as *const _);
                let double_func: unsafe extern "C" fn(id, ffi::SEL) -> f64 =
                    std::mem::transmute(ffi::objc_msgSend as *const std::ffi::c_void);
                let delta_x = double_func(ns_event, dx_sel);
                let delta_y = double_func(ns_event, dy_sel);
                Some(Event::Scroll { delta_x, delta_y, modifiers })
            }
            _ => None,
        }
    }
}

static REGISTER_DELEGATE: std::sync::Once = std::sync::Once::new();

pub fn register_delegate_class() -> Class {
    let mut cls = std::ptr::null_mut();
    REGISTER_DELEGATE.call_once(|| {
        unsafe {
            let superclass = ffi::objc_getClass(b"NSObject\0".as_ptr() as *const _);
            cls = ffi::objc_allocateClassPair(superclass, b"RustWindowDelegate\0".as_ptr() as *const _, 0);
            
            ffi::class_addMethod(
                cls,
                ffi::sel_registerName(b"windowShouldClose:\0".as_ptr() as *const _),
                std::mem::transmute(window_should_close as *const std::ffi::c_void),
                b"c@:@\0".as_ptr() as *const _,
            );
            
            ffi::class_addMethod(
                cls,
                ffi::sel_registerName(b"windowDidResize:\0".as_ptr() as *const _),
                std::mem::transmute(window_did_resize as *const std::ffi::c_void),
                b"v@:@\0".as_ptr() as *const _,
            );
            
            ffi::objc_registerClassPair(cls);
        }
    });
    if cls.is_null() {
        unsafe { ffi::objc_getClass(b"RustWindowDelegate\0".as_ptr() as *const _) }
    } else {
        cls
    }
}

extern "C" fn window_should_close(_this: id, _cmd: ffi::SEL, _sender: id) -> ffi::BOOL {
    push_event(Event::CloseRequested);
    YES
}

extern "C" fn window_did_resize(_this: id, _cmd: ffi::SEL, notification: id) {
    unsafe {
        let window: id = objc::msg_send_id(notification, ffi::sel_registerName(b"object\0".as_ptr() as *const _));
        let content_view = objc::msg_send_id(window, ffi::sel_registerName(b"contentView\0".as_ptr() as *const _));
        let frame_sel = ffi::sel_registerName(b"frame\0".as_ptr() as *const _);
        let frame = objc::msg_send_rect(content_view, frame_sel);
        
        let scale_sel = ffi::sel_registerName(b"backingScaleFactor\0".as_ptr() as *const _);
        let scale_func: unsafe extern "C" fn(id, ffi::SEL) -> f64 =
            std::mem::transmute(ffi::objc_msgSend as *const std::ffi::c_void);
        let scale = scale_func(window, scale_sel);
        
        let physical_width = (frame.size.width * scale) as usize;
        let physical_height = (frame.size.height * scale) as usize;
        
        push_event(Event::Resized {
            width: frame.size.width,
            height: frame.size.height,
            physical_width,
            physical_height,
        });
    }
}
