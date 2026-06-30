use crate::*;

#[link(name = "Gdi32")]
extern "system" {
    pub fn ChoosePixelFormat(hdc: *mut c_void, ppfd: *const PIXELFORMATDESCRIPTOR) -> i32;
    pub fn SetPixelFormat(hdc: *mut c_void, format: i32, ppfd: *const PIXELFORMATDESCRIPTOR)
        -> i32;
    pub fn SwapBuffers(hdc: *mut c_void) -> i32;
    pub fn StretchDIBits(
        hdc: *mut c_void,
        XDest: i32,
        YDest: i32,
        nDestWidth: i32,
        nDestHeight: i32,
        XSrc: i32,
        YSrc: i32,
        nSrcWidth: i32,
        nSrcHeight: i32,
        lpBits: *const c_void,
        lpBitsInfo: *const BITMAPINFO,
        iUsage: UINT,
        dwRop: DWORD,
    ) -> i32;
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct BITMAPINFOHEADER {
    pub size: DWORD,
    pub width: LONG,
    pub height: LONG,
    pub planes: WORD,
    pub bit_count: WORD,
    pub compression: DWORD,
    pub size_image: DWORD,
    pub x_pels_per_meter: LONG,
    pub y_pels_per_meter: LONG,
    pub clr_used: DWORD,
    pub clr_important: DWORD,
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct BITMAPINFO {
    pub header: BITMAPINFOHEADER,
    pub colors: [RGBQUAD; 1],
}

impl BITMAPINFO {
    #[inline]
    pub const fn new(width: i32, height: i32) -> BITMAPINFO {
        BITMAPINFO {
            header: BITMAPINFOHEADER {
                size: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                width,
                height: -height,
                planes: 1,
                bit_count: 32,
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
            }],
        }
    }
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct RGBQUAD {
    pub blue: u8,
    pub green: u8,
    pub red: u8,
    pub reserved: u8,
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct PIXELFORMATDESCRIPTOR {
    pub nSize: WORD,
    pub nVersion: WORD,
    pub dwFlags: DWORD,
    pub iPixelType: u8,
    pub cColorBits: u8,
    pub cRedBits: u8,
    pub cRedShift: u8,
    pub cGreenBits: u8,
    pub cGreenShift: u8,
    pub cBlueBits: u8,
    pub cBlueShift: u8,
    pub cAlphaBits: u8,
    pub cAlphaShift: u8,
    pub cAccumBits: u8,
    pub cAccumRedBits: u8,
    pub cAccumGreenBits: u8,
    pub cAccumBlueBits: u8,
    pub cAccumAlphaBits: u8,
    pub cDepthBits: u8,
    pub cStencilBits: u8,
    pub cAuxBuffers: u8,
    pub iLayerType: u8,
    pub bReserved: u8,
    pub dwLayerMask: DWORD,
    pub dwVisibleMask: DWORD,
    pub dwDamageMask: DWORD,
}

pub const PFD_DRAW_TO_WINDOW: DWORD = 0x0000_0004;
pub const PFD_SUPPORT_OPENGL: DWORD = 0x0000_0020;
pub const PFD_DOUBLEBUFFER: DWORD = 0x0000_0001;
pub const PFD_TYPE_RGBA: u8 = 0;
pub const PFD_MAIN_PLANE: u8 = 0;
