use crate::*;
use std::collections::VecDeque;
pub const DEFAULT_DPI: f32 = 96.0;

pub fn create_window(
    title: &str,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    style: WindowStyle,
) -> std::pin::Pin<Box<Window>> {
    unsafe {
        if SetThreadDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2) == 0 {
            panic!("Only Windows 10 (1607) or later is supported.")
        };

        //Title must be null terminated.
        let title = std::ffi::CString::new(title).unwrap();

        let wnd_class = WNDCLASSA {
            style: 0,
            wnd_proc: Some(wnd_proc),
            cls_extra: 0,
            wnd_extra: 0,
            instance: 0,
            icon: 0,
            //Prevent cursor from changing when loading.
            cursor: LoadCursorW(null_mut(), IDC_ARROW) as isize,
            background: 0,
            menu_name: core::mem::zeroed(),
            class_name: title.as_ptr() as *const u8,
        };

        //Adjust the rect to fit exactly what the user requested.
        //Windows has padding and other weird nonsense when trying set the width and height.
        //Not needed anymore?

        // let mut rect = RECT {
        //     left: 0,
        //     top: 0,
        //     right: width as i32,
        //     bottom: height as i32,
        // };
        // AdjustWindowRectEx(&mut rect, style.style, 0, style.exstyle);

        RegisterClassA(&wnd_class);

        let (win_style, win_exstyle) = get_style_flags(style);

        let hwnd = CreateWindowExA(
            win_exstyle,
            title.as_ptr() as *const u8,
            title.as_ptr() as *const u8,
            win_style,
            if x == 0 { CW_USEDEFAULT } else { x },
            if y == 0 { CW_USEDEFAULT } else { y },
            // CW_USEDEFAULT,
            // CW_USEDEFAULT,
            //These are adjusted later for DPI scaling.
            width,
            height,
            0,
            0,
            0,
            null(),
        );

        //Get the display scale factor 1.0, 1.25, 1.5, 1.75, can also be custom.
        let scale = GetDpiForWindow(hwnd) as f32 / DEFAULT_DPI;
        let mut area = get_client_rect(hwnd);

        //Scale the size of the window to match the display scale.
        //AdjustWindowRect used to be needed, but isn't anymore, I'm not sure why?
        if scale != 1.0 {
            SetWindowPos(
                hwnd,
                0,
                area.x as i32,
                area.y as i32,
                (area.width as f32 * scale) as i32,
                (area.height as f32 * scale) as i32,
                SWP_FRAMECHANGED,
            );
            //Update the area since SetWindowPos will change it.
            area = get_client_rect(hwnd);
        }

        assert_ne!(hwnd, 0);
        let dc = GetDC(hwnd);

        // Construct window, initialize WGL, then pin.
        let window = Window {
            //Re-grab the area after calling SetWindowPos.
            area,
            hwnd,
            dc,
            display_scale: scale,
            buffer: vec![0u32; area.width * area.height],
            bitmap: BITMAPINFO::new(area.width as i32, area.height as i32),
            quit: false,
            mouse_position: Rect::default(),
            left_mouse: MouseButtonState::new(),
            right_mouse: MouseButtonState::new(),
            middle_mouse: MouseButtonState::new(),
            mouse_4: MouseButtonState::new(),
            mouse_5: MouseButtonState::new(),
            input: InputState::new(),
            tray: MouseButtonState::new(),
            hglrc: null_mut(),
            focused: true,
            needs_frame_advance: false,
            render_callback: core::ptr::null_mut(),
            render_executor: None,
            event_queue: VecDeque::new(),
        };

        //Safety: This *should* be pinned.
        let window = Box::pin(window);
        let addr = &*window as *const Window;
        let result = SetWindowLongPtrW(window.hwnd, GWLP_USERDATA, addr as isize);
        assert!(result <= 0);

        window
    }
}

#[derive(Debug)]
pub struct Window {
    pub hwnd: isize,
    pub display_scale: f32,
    //GDI related
    pub dc: *mut c_void,
    pub buffer: Vec<u32>,
    pub bitmap: BITMAPINFO,
    pub area: Rect,
    pub quit: bool,
    pub mouse_position: Rect,
    pub input: InputState,
    pub left_mouse: MouseButtonState,
    pub right_mouse: MouseButtonState,
    pub middle_mouse: MouseButtonState,
    pub mouse_4: MouseButtonState,
    pub mouse_5: MouseButtonState,
    pub tray: MouseButtonState,
    pub hglrc: HGLRC,
    pub focused: bool,
    needs_frame_advance: bool,
    pub render_callback: *mut core::ffi::c_void,
    pub render_executor: Option<unsafe fn(*mut core::ffi::c_void, &mut Window)>,
    pub event_queue: VecDeque<Event>,
}

impl Window {
    pub const unsafe fn empty() -> Self {
        Self {
            hwnd: 0,
            display_scale: 1.0,
            dc: unsafe { core::mem::zeroed() },
            buffer: Vec::new(),
            bitmap: BITMAPINFO {
                header: BITMAPINFOHEADER {
                    size: 0,
                    width: 0,
                    height: 0,
                    planes: 0,
                    bit_count: 0,
                    compression: 0,
                    size_image: 0,
                    x_pels_per_meter: 0,
                    y_pels_per_meter: 0,
                    clr_used: 0,
                    clr_important: 0,
                },
                colors: [RGBQUAD {
                    blue: 0,
                    green: 0,
                    red: 0,
                    reserved: 0,
                }; 1],
            },
            area: Rect {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            },
            quit: false,
            mouse_position: Rect {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            },
            left_mouse: MouseButtonState::new(),
            input: InputState::new(),
            right_mouse: MouseButtonState::new(),
            middle_mouse: MouseButtonState::new(),
            mouse_4: MouseButtonState::new(),
            mouse_5: MouseButtonState::new(),
            tray: MouseButtonState::new(),
            hglrc: unsafe { core::mem::zeroed() },
            focused: false,
            needs_frame_advance: false,
            render_callback: core::ptr::null_mut(),
            render_executor: None,
            event_queue: VecDeque::new(),
        }
    }

    /// Safety: Mutiple calls to this is unsafe.
    pub unsafe fn init_wgl_debug(&mut self) {
        mini::profile!();
        pub const WGL_CONTEXT_MAJOR_VERSION_ARB: i32 = 0x2091;
        pub const WGL_CONTEXT_MINOR_VERSION_ARB: i32 = 0x2092;
        pub const WGL_CONTEXT_FLAGS_ARB: i32 = 0x2094;
        pub const WGL_CONTEXT_PROFILE_MASK_ARB: i32 = 0x9126;

        pub const WGL_CONTEXT_DEBUG_BIT_ARB: i32 = 0x0001;
        pub const WGL_CONTEXT_CORE_PROFILE_BIT_ARB: i32 = 0x00000001;

        unsafe {
            let mut pfd = PIXELFORMATDESCRIPTOR::default();
            pfd.nSize = core::mem::size_of::<PIXELFORMATDESCRIPTOR>() as WORD;
            pfd.nVersion = 1;
            pfd.dwFlags = PFD_DRAW_TO_WINDOW | PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER;
            pfd.iPixelType = PFD_TYPE_RGBA;
            pfd.cColorBits = 32;
            pfd.cDepthBits = 24;
            pfd.cStencilBits = 8;
            pfd.iLayerType = PFD_MAIN_PLANE;

            let pixel_format = ChoosePixelFormat(self.dc, &pfd);
            assert!(pixel_format > 0);
            assert!(SetPixelFormat(self.dc, pixel_format, &pfd) != 0);

            let dummy_hglrc = wglCreateContext(self.dc);
            assert!(!dummy_hglrc.is_null());
            assert!(wglMakeCurrent(self.dc, dummy_hglrc) != 0,);

            let ptr = wglGetProcAddress(b"wglCreateContextAttribsARB\0".as_ptr() as *const i8);
            assert!(!ptr.is_null());

            let wgl_create_context_attribs_arb: unsafe extern "system" fn(
                *mut c_void,
                *mut c_void,
                *const i32,
            )
                -> *mut c_void = { core::mem::transmute(ptr) };

            let attribs = [
                WGL_CONTEXT_MAJOR_VERSION_ARB,
                4,
                WGL_CONTEXT_MINOR_VERSION_ARB,
                6,
                WGL_CONTEXT_PROFILE_MASK_ARB,
                WGL_CONTEXT_CORE_PROFILE_BIT_ARB,
                WGL_CONTEXT_FLAGS_ARB,
                WGL_CONTEXT_DEBUG_BIT_ARB,
                0,
            ];

            let hglrc = wgl_create_context_attribs_arb(self.dc, null_mut(), attribs.as_ptr());
            assert!(!hglrc.is_null(), "wglCreateContextAttribsARB failed");

            assert!(wglMakeCurrent(self.dc, hglrc) != 0);
            self.hglrc = hglrc;
        }
    }

    pub unsafe fn init_wgl(&mut self) {
        mini::profile!();
        unsafe {
            let mut pfd = PIXELFORMATDESCRIPTOR::default();
            pfd.nSize = core::mem::size_of::<PIXELFORMATDESCRIPTOR>() as WORD;
            pfd.nVersion = 1;
            pfd.dwFlags = PFD_DRAW_TO_WINDOW | PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER;
            pfd.iPixelType = PFD_TYPE_RGBA;
            pfd.cColorBits = 32;
            pfd.cDepthBits = 24;
            pfd.cStencilBits = 8;
            pfd.iLayerType = PFD_MAIN_PLANE;

            let pixel_format = ChoosePixelFormat(self.dc, &pfd);
            assert!(pixel_format > 0);
            assert!(SetPixelFormat(self.dc, pixel_format, &pfd) != 0);

            let hglrc = wglCreateContext(self.dc);
            assert!(!hglrc.is_null());
            assert!(wglMakeCurrent(self.dc, hglrc) != 0);
            self.hglrc = hglrc;
        }
    }

    pub fn get_wgl_proc_address(&self, name: &str) -> *const c_void {
        unsafe { wglGetProcAddress(std::ffi::CString::new(name).unwrap().as_ptr()) }
    }

    pub unsafe fn set_swap_interval(&self, interval: i32) {
        let ptr = unsafe { wglGetProcAddress("wglSwapIntervalEXT\0".as_ptr() as *const _) };
        assert!(!ptr.is_null());
        let func: unsafe extern "system" fn(i32) -> i32 = unsafe { core::mem::transmute(ptr) };
        unsafe { func(interval) };
    }

    pub fn swap_buffers(&self) {
        unsafe { SwapBuffers(self.dc) };
    }

    ///Updates the width and height based on the display scale.
    pub fn rescale_window(&self) {
        let area = self.client_area();
        let (width, height) = if self.display_scale == 1.0 {
            (
                area.width as f32 / self.display_scale,
                area.height as f32 / self.display_scale,
            )
        } else {
            (
                area.width as f32 * self.display_scale,
                area.height as f32 * self.display_scale,
            )
        };

        unsafe {
            SetWindowPos(
                self.hwnd,
                0,
                area.x as i32,
                area.y as i32,
                width as i32,
                height as i32,
                SWP_FRAMECHANGED,
            )
        };
    }

    pub const fn display_scale(&self) -> f32 {
        self.display_scale
    }

    pub fn set_title(&self, title: &str) {
        let title_c = std::ffi::CString::new(title).unwrap();
        unsafe {
            SetWindowTextA(self.hwnd, title_c.as_ptr() as *const u8);
        }
    }

    pub fn client_area(&self) -> Rect {
        let mut rect = RECT::default();
        let _ = unsafe { GetClientRect(self.hwnd, &mut rect) };
        Rect::from_windows(rect)
    }

    pub const fn width(&self) -> usize {
        self.area.width
    }

    pub const fn height(&self) -> usize {
        self.area.height
    }

    pub fn borderless(&mut self) {
        unsafe {
            SetWindowLongPtrA(self.hwnd, GWL_STYLE, get_style_flags(crate::common::WindowStyle::Borderless).0 as isize);

            //Update the window area without moving or resizing it.
            SetWindowPos(
                self.hwnd,
                0,
                0,
                0,
                0,
                0,
                SWP_FRAMECHANGED | SWP_NOMOVE | SWP_NOSIZE,
            );
        };
    }

    pub fn fullscreen(&mut self) {
        unsafe {
            let monitor = MonitorFromWindow(self.hwnd, MONITOR_DEFAULTTOPRIMARY);
            let mut monitor_info: MONITORINFO = std::mem::zeroed();
            monitor_info.cbSize = std::mem::size_of::<MONITORINFO>() as u32;
            assert!(GetMonitorInfoA(monitor, &mut monitor_info) != 0);

            let style = get_style_flags(crate::common::WindowStyle::Borderless).0 | WS_MAXIMIZE;
            SetWindowLongPtrA(self.hwnd, GWL_STYLE, style as isize);

            let x = monitor_info.rcMonitor.left;
            let y = monitor_info.rcMonitor.top;
            let width = monitor_info.rcMonitor.right - monitor_info.rcMonitor.left;
            let height = monitor_info.rcMonitor.bottom - monitor_info.rcMonitor.top;

            SetWindowPos(self.hwnd, 0, x, y, width, height, SWP_FRAMECHANGED);
        };
    }

    pub fn set_pos(&mut self, x: usize, y: usize, width: usize, height: usize, flags: u32) {
        unsafe {
            SetWindowPos(
                self.hwnd,
                0,
                x as i32,
                y as i32,
                width as i32,
                height as i32,
                flags,
            );
        }
    }

    pub fn reset_style(&mut self) {
        unsafe {
            SetWindowLongPtrA(self.hwnd, GWL_STYLE, get_style_flags(crate::common::WindowStyle::Standard).0 as isize);

            //Update the window area without moving or resizing it.
            SetWindowPos(
                self.hwnd,
                0,
                0,
                0,
                0,
                0,
                SWP_FRAMECHANGED | SWP_NOMOVE | SWP_NOSIZE,
            );
        };
    }

    pub fn translate_message(&mut self, msg: MSG, message_result: i32) -> Option<Event> {
        if message_result == 0 {
            return None;
        } else if message_result == -1 {
            let last_error = unsafe { GetLastError() };
            panic!("Error with `GetMessageA`, error code: {}", last_error);
        }

        match msg.message {
            WM_CHAR => {
                if let Some(c) = char::from_u32(msg.w_param as u32) {
                    if !c.is_control() {
                        return Some(Event::ReceivedCharacter(c));
                    }
                }
            }
            _ => {}
        };

        return None;
    }

    pub fn event(&mut self) -> Option<Event> {
        if self.needs_frame_advance {
            self.input.advance_frame();
            self.needs_frame_advance = false;
        }
        if let Some(event) = self.event_queue.pop_front() {
            return Some(event);
        }
        if self.quit {
            return Some(Event::Quit);
        }
        None
    }

    pub fn draw<F>(&mut self, mut render: F)
    where
        F: FnMut(&mut Self),
    {
        render(self);
        //TODO: Skip this for WGL.
        self.present();

        unsafe fn execute_render<F>(closure_ptr: *mut c_void, window: &mut Window)
        where
            F: FnMut(&mut Window),
        {
            let closure = &mut *(closure_ptr as *mut F);
            closure(window);
        }

        self.render_callback = &mut render as *mut F as *mut c_void;
        self.render_executor = Some(execute_render::<F>);

        unsafe {
            let mut msg = MSG::default();
            while PeekMessageA(&mut msg, self.hwnd, 0, 0, PM_REMOVE) != 0 {
                TranslateMessage(&msg);
                DispatchMessageA(&msg);
                if let Some(event) = self.translate_message(msg.clone(), 1) {
                    self.event_queue.push_back(event);
                }
            }
        }

        self.render_callback = core::ptr::null_mut();
        self.render_executor = None;
        self.needs_frame_advance = true;
    }

    pub fn present(&mut self) {
        unsafe {
            StretchDIBits(
                self.dc,
                0,
                0,
                self.area.width as i32,
                self.area.height as i32,
                0,
                0,
                self.area.width as i32,
                self.area.height as i32,
                self.buffer.as_mut_ptr() as *const c_void,
                &self.bitmap,
                0,
                SRCCOPY,
            );
        }
    }

    pub fn vsync(&self) {
        unsafe { DwmFlush() };
    }
}

pub fn get_style_flags(style: crate::common::WindowStyle) -> (u32, u32) {
    match style {
        crate::common::WindowStyle::Standard => (
            WS_CAPTION | WS_SYSMENU | WS_THICKFRAME | WS_MINIMIZEBOX | WS_MAXIMIZEBOX | WS_VISIBLE,
            0,
        ),
        crate::common::WindowStyle::Borderless | crate::common::WindowStyle::Transparent => (
            WS_POPUP | WS_VISIBLE,
            0,
        ),
    }
}

fn invoke_render_callback(window: &mut Window) {
    if window.render_callback.is_null() || window.render_executor.is_none() {
        return;
    }

    let cb_ptr = window.render_callback;
    let executor = window.render_executor;

    window.render_callback = core::ptr::null_mut();
    window.render_executor = None;

    if let Some(exec) = executor {
        unsafe { exec(cb_ptr, window) };
    }

    //TODO: Will need to support WGL use cases here too.
    window.present();

    // Restore the callback pointer
    window.render_callback = cb_ptr;
    window.render_executor = executor;
}

pub unsafe extern "system" fn wnd_proc(
    hwnd: isize,
    msg: u32,
    wparam: usize,
    lparam: isize,
) -> isize {
    if msg == WM_CREATE {
        set_dark_theme(hwnd);
        return 0;
    }

    let ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut Window;
    if ptr.is_null() {
        return DefWindowProcA(hwnd, msg, wparam, lparam);
    }

    //I'm not convinced this is the right way to do this.
    let window: &mut Window = &mut *ptr;

    //Clamp negative numbers to 0
    let mx = (lparam as i16).max(0) as usize;
    let my = ((lparam >> 16) as i16).max(0) as usize;

    let low = (lparam & 0xffff) as usize;
    let high = ((lparam >> 16) & 0xffff) as usize;

    // println!("{}", wm_code_name(msg));

    match msg {
        //We can choose not to destroy the window, for example with a save prompt.
        WM_CLOSE => {
            assert!(DestroyWindow(hwnd) != 0);
            return 0;
        }
        WM_DESTROY => {
            PostQuitMessage(0);
            window.quit = true;
            window.event_queue.push_back(Event::Quit);
            return 0;
        }
        WM_SIZE => {
            //TODO: How to skip this for no GDI use.
            let (width, height) = (low, high);
            window.buffer.clear();
            window.buffer.resize(width * height, 0);
            window.bitmap = BITMAPINFO::new(width as i32, height as i32);
            window.area = Rect::new(0, 0, width, height);
            invoke_render_callback(window);
            return 0;
        }
        WM_SIZING | WM_PAINT => {
            invoke_render_callback(window);
        }
        //https://learn.microsoft.com/en-us/windows/win32/hidpi/wm-dpichanged
        WM_DPICHANGED => {
            //The new display scale and DPI.
            let dpi = (wparam >> 16) & 0xffff;
            let scale = dpi as f32 / DEFAULT_DPI;

            //This is the recommended x, y, width and height.
            //The width and height is wrong so we ignore it.
            //X and Y seems right.
            let ptr = lparam as *mut RECT;
            assert!(!ptr.is_null());
            let rect = &(*ptr);

            let old = window.client_area();
            let original_width = old.width as f32 / window.display_scale;
            let original_height = old.height as f32 / window.display_scale;

            let (width, height) = if scale == 1.0 {
                (original_width, original_height)
            } else {
                (original_width * scale, original_height * scale)
            };

            mini::info!("Rescaling Window x: {}, y: {}, width: {}, height: {}, old_scale: {}, new_scale: {}", old.x, old.y, width.round(), height.round(), window.display_scale, scale);

            SetWindowPos(
                hwnd,
                0,
                rect.left,
                rect.top,
                width.round() as i32,
                height.round() as i32,
                SWP_NOZORDER | SWP_NOACTIVATE,
            );

            window.display_scale = scale;
            return 0;
        }
        WM_MOUSEMOVE => {
            window.mouse_position = Rect::new(mx, my, 1, 1);
        }
        WM_MOUSEWHEEL => {
            const WHEEL_DELTA: i16 = 120;
            let value = (wparam >> 16) as i16;
            let delta = value as f32 / WHEEL_DELTA as f32;
            window.input.scroll_delta = delta;
            return 0;
        }
        WM_KEYDOWN | WM_SYSKEYDOWN => {
            window.input.set_key_down(wparam);
        }
        WM_KEYUP | WM_SYSKEYUP => {
            window.input.set_key_up(wparam);
        }
        WM_LBUTTONDOWN | WM_RBUTTONDOWN | WM_MBUTTONDOWN | WM_XBUTTONDOWN => {
            SetCapture(hwnd);
            let rect = Rect::new(low, high, 1, 1);
            match msg {
                WM_LBUTTONDOWN => window.left_mouse.pressed(rect),
                WM_RBUTTONDOWN => window.right_mouse.pressed(rect),
                WM_MBUTTONDOWN => window.middle_mouse.pressed(rect),
                WM_XBUTTONDOWN => {
                    let button = ((wparam >> 16) & 0xffff) as usize;
                    if button == 1 {
                        window.mouse_4.pressed(rect);
                    } else if button == 2 {
                        window.mouse_5.pressed(rect);
                    }
                }
                _ => {}
            }
        }
        WM_LBUTTONUP | WM_RBUTTONUP | WM_MBUTTONUP | WM_XBUTTONUP => {
            // Only release capture if no other mouse buttons are currently being held down.
            if wparam as u32 & (MK_LBUTTON | MK_RBUTTON | MK_MBUTTON | MK_XBUTTON1 | MK_XBUTTON2)
                == 0
            {
                ReleaseCapture();
            }

            let rect = Rect::new(low, high, 1, 1);
            match msg {
                WM_LBUTTONUP => window.left_mouse.released(rect),
                WM_RBUTTONUP => window.right_mouse.released(rect),
                WM_MBUTTONUP => window.middle_mouse.released(rect),
                WM_XBUTTONUP => {
                    let button = ((wparam >> 16) & 0xffff) as usize;
                    if button == 1 {
                        window.mouse_4.released(rect);
                    } else if button == 2 {
                        window.mouse_5.released(rect);
                    }
                }
                _ => {}
            }
        }
        WM_KILLFOCUS => {
            window.focused = false;
            return 0;
        }
        WM_SETFOCUS => {
            window.focused = true;
            return 0;
        }
        WM_TRAYICON if low as u32 == WM_LBUTTONDOWN => {
            //Position doesn't really matter.
            window.tray.pressed(Rect::default());
            return 0;
        }
        _ => {}
    }

    DefWindowProcA(hwnd, msg, wparam, lparam)
}
