unsafe extern "system" {
    pub fn DwmGetColorizationColor(pcrColorization: *mut u32, pfOpaqueBlend: *mut i32) -> i32;
}

pub fn accent_color() -> u32 {
    unsafe {
        let mut color = core::mem::zeroed();
        let mut blend = core::mem::zeroed();
        assert!(DwmGetColorizationColor(&mut color, &mut blend) == 0);
        let r = (color & 0xFF) as u8;
        let g = ((color >> 8) & 0xFF) as u8;
        let b = ((color >> 16) & 0xFF) as u8;
        //bgr format instead of rgb for some reason.
        (b as u32) << 16 | (g as u32) << 8 | (r as u32)
    }
}
