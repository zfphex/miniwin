use crate::*;

#[link(name = "dwmapi")]
#[link(name = "winmm")]
unsafe extern "system" {
    pub fn DwmFlush() -> i32;
    pub fn timeBeginPeriod(uPeriod: UINT) -> u32;
    pub fn timeEndPeriod(uPeriod: UINT) -> u32;
    pub fn CreateWaitableTimerA(
        lpTimerAttributes: *mut SECURITY_ATTRIBUTES,
        bManualReset: BOOL,
        lpTimerName: LPCSTR,
    ) -> HANDLE;
    pub fn SetWaitableTimer(
        hTimer: HANDLE,
        lpDueTime: *const i64,
        lPeriod: LONG,
        pfnCompletionRoutine: PTIMERAPCROUTINE,
        lpArgToCompletionRoutine: *mut c_void,
        fResume: BOOL,
    ) -> BOOL;
    pub fn WaitForSingleObject(hHandle: HANDLE, dwMilliseconds: DWORD) -> DWORD;
    pub fn QueryPerformanceCounter(lpPerformanceCount: *mut i64) -> BOOL;
    pub fn QueryPerformanceFrequency(lpFrequency: *mut i64) -> BOOL;
}

pub type PTIMERAPCROUTINE = Option<
    unsafe extern "system" fn(
        lpArgToCompletionRoutine: *mut c_void,
        dwTimerLowValue: DWORD,
        dwTimerHighValue: DWORD,
    ),
>;

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct SECURITY_ATTRIBUTES {
    pub nLength: DWORD,
    pub lpSecurityDescriptor: *mut c_void,
    pub bInheritHandle: BOOL,
}
