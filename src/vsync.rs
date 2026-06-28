use crate::ffi::*;

pub struct VsyncTracker {
    display_link: CVDisplayLinkRef,
    semaphore: dispatch_semaphore_t,
}

unsafe extern "C" fn display_link_callback(
    _display_link: CVDisplayLinkRef,
    _in_now: *const std::ffi::c_void,
    _in_output_time: *const std::ffi::c_void,
    _flags_in: u64,
    _flags_out: *mut u64,
    context: *mut std::ffi::c_void,
) -> CVReturn {
    unsafe {
        dispatch_semaphore_signal(context as dispatch_semaphore_t);
    }
    0 // kCVReturnSuccess
}

impl VsyncTracker {
    pub fn new() -> Self {
        unsafe {
            let mut display_link = std::ptr::null_mut();
            let res = CVDisplayLinkCreateWithActiveCGDisplays(&mut display_link);
            if res != 0 || display_link.is_null() {
                panic!("Failed to create CVDisplayLink (result: {})", res);
            }

            let semaphore = dispatch_semaphore_create(0);
            if semaphore.is_null() {
                CVDisplayLinkRelease(display_link);
                panic!("Failed to create GCD semaphore");
            }

            let callback_res = CVDisplayLinkSetOutputCallback(display_link, display_link_callback, semaphore);
            if callback_res != 0 {
                dispatch_release(semaphore);
                CVDisplayLinkRelease(display_link);
                panic!("Failed to set CVDisplayLink output callback (result: {})", callback_res);
            }

            let start_res = CVDisplayLinkStart(display_link);
            if start_res != 0 {
                dispatch_release(semaphore);
                CVDisplayLinkRelease(display_link);
                panic!("Failed to start CVDisplayLink (result: {})", start_res);
            }

            Self {
                display_link,
                semaphore,
            }
        }
    }

    pub fn wait_for_vsync(&self) {
        unsafe {
            dispatch_semaphore_wait(self.semaphore, DISPATCH_TIME_FOREVER);
        }
    }
}

impl Drop for VsyncTracker {
    fn drop(&mut self) {
        unsafe {
            CVDisplayLinkStop(self.display_link);
            CVDisplayLinkRelease(self.display_link);
            dispatch_release(self.semaphore);
        }
    }
}
