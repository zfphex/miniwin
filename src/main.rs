use winmac::ffi::*;

fn main() {
    unsafe {
        let cls_name = std::ffi::CString::new("NSApplication").unwrap();
        let cls = objc_getClass(cls_name.as_ptr());
        println!("NSApplication Class pointer: {:?}", cls);
        
        let sel_name = std::ffi::CString::new("sharedApplication").unwrap();
        let sel = sel_registerName(sel_name.as_ptr());
        println!("sharedApplication Selector: {:?}", sel);
    }
}
