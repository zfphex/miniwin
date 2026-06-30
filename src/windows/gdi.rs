use mini::profile;

use crate::*;

pub fn capture_virtual_screen() -> (Vec<u32>, i32, i32, i32, i32) {
    profile!();
    unsafe {
        let width = GetSystemMetrics(SM_CXVIRTUALSCREEN);
        let height = GetSystemMetrics(SM_CYVIRTUALSCREEN);
        let left = GetSystemMetrics(SM_XVIRTUALSCREEN);
        let top = GetSystemMetrics(SM_YVIRTUALSCREEN);

        //Get the device context for virtual screen
        let hdc_screen = GetDC(0);
        assert!(!hdc_screen.is_null());

        let hdc_mem = CreateCompatibleDC(hdc_screen);
        assert!(!hdc_mem.is_null());

        let hbm = CreateCompatibleBitmap(hdc_screen, width, height);
        assert!(!hbm.is_null());

        //Select the bitmap into the memory DC
        //I don't understand this function at all.
        let old_obj = SelectObject(hdc_mem, hbm as HANDLE);
        assert!(!old_obj.is_null());

        //Copy screen to bitmap
        assert!(BitBlt(hdc_mem, 0, 0, width, height, hdc_screen, left, top, SRCCOPY) != 0);

        let mut bitmap_info = BITMAPINFO::new(width, height);
        // let data_size = (width * height * 4) as usize;
        // let mut pixel_data: Vec<u8> = vec![0; data_size];

        let data_size = (width * height) as usize;
        let mut pixel_data: Vec<u32> = vec![0; data_size];

        //Copy from hbm to pixel_data
        assert!(
            GetDIBits(
                hdc_mem,
                hbm,
                0,
                height as u32,
                pixel_data.as_mut_ptr() as *mut _,
                &mut bitmap_info,
                DIB_RGB_COLORS,
            ) != 0
        );

        //Windows is a great garbage collector.
        //This takes like 10ms so why bother?

        // SelectObject(hdc_mem, old_obj);
        // DeleteObject(hbm as HANDLE);
        // DeleteDC(hdc_mem);
        // ReleaseDC(0, hdc_screen);

        (pixel_data, left, top, width, height)
    }
}

pub const SRCCOPY: u32 = 0x00CC0020;
pub const DEFAULT_CHARSET: DWORD = 1;
pub const OUT_OUTLINE_PRECIS: DWORD = 8;
pub const CLIP_DEFAULT_PRECIS: DWORD = 0;
pub const CLEARTYPE_QUALITY: DWORD = 5;
pub const TRANSPARENT: i32 = 1;
pub const RGN_AND: i32 = 1;

#[link(name = "Msimg32")]
extern "system" {
    pub fn AlphaBlend(
        hdcDest: *mut c_void,
        xoriginDest: i32,
        yoriginDest: i32,
        wDest: i32,
        hDest: i32,
        hdcSrc: *mut c_void,
        xoriginSrc: i32,
        yoriginSrc: i32,
        wSrc: i32,
        hSrc: i32,
        ftn: BLENDFUNCTION,
    ) -> i32;
}

#[link(name = "Gdi32")]
extern "system" {
    pub fn ChoosePixelFormat(hdc: *mut c_void, ppfd: *const PIXELFORMATDESCRIPTOR) -> i32;
    pub fn DescribePixelFormat(
        hdc: *mut c_void,
        iPixelFormat: i32,
        nBytes: u32,
        ppfd: *mut PIXELFORMATDESCRIPTOR,
    ) -> i32;
    pub fn SetPixelFormat(hdc: *mut c_void, format: i32, ppfd: *const PIXELFORMATDESCRIPTOR) -> i32;
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
    pub fn CreateFontA(
        cHeight: i32,
        cWidth: i32,
        cEscapement: i32,
        cOrientation: i32,
        cWeight: i32,
        bItalic: DWORD,
        bUnderline: DWORD,
        bStrikeOut: DWORD,
        iCharSet: DWORD,
        iOutPrecision: DWORD,
        iClipPrecision: DWORD,
        iQuality: DWORD,
        iPitchAndFamily: DWORD,
        pszFaceName: LPCSTR,
    ) -> *mut c_void;
    pub fn CreateDIBSection(
        hdc: *mut c_void,
        lpbmi: *const BITMAPINFO,
        usage: UINT,
        ppvBits: *mut *mut c_void,
        hSection: *mut c_void,
        offset: DWORD,
    ) -> *mut c_void;
    pub fn TextOutA(hdc: *mut c_void, x: i32, y: i32, lpString: LPCSTR, c: i32) -> i32;
    pub fn SetTextColor(hdc: *mut c_void, color: u32) -> u32;
    pub fn SetBkMode(hdc: *mut c_void, mode: i32) -> i32;
    pub fn CreateCompatibleDC(hdc: *mut c_void) -> *mut c_void;
    pub fn DeleteDC(hdc: *mut c_void) -> i32;
    pub fn SelectObject(hdc: *mut c_void, h: *mut c_void) -> *mut c_void;
    pub fn SetRect(lprc: *mut RECT, xLeft: i32, yTop: i32, xRight: i32, yBottom: i32) -> BOOL;
    pub fn BeginPath(hdc: *mut c_void) -> i32;
    pub fn EndPath(hdc: *mut c_void) -> i32;
    pub fn SelectClipPath(hdc: *mut c_void, mode: i32) -> BOOL;
    pub fn DeleteObject(ho: *mut c_void) -> BOOL;
    pub fn BitBlt(
        hdc: *mut c_void,
        x: i32,
        y: i32,
        cx: i32,
        cy: i32,
        hdcSrc: *mut c_void,
        x1: i32,
        y1: i32,
        rop: DWORD,
    ) -> BOOL;
    pub fn GetDIBits(
        hdc: *mut c_void,
        hbm: *mut c_void,
        start: UINT,
        cLines: UINT,
        lpvBits: *mut c_void,
        lpbmi: *mut BITMAPINFO,
        usage: UINT,
    ) -> i32;
    pub fn ReleaseDC(hWnd: HWND, hDC: *mut c_void) -> i32;
    pub fn CreateCompatibleBitmap(hdc: *mut c_void, cx: i32, cy: i32) -> *mut c_void;
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

pub const AC_SRC_OVER: u8 = 0x00;
pub const AC_SRC_ALPHA: u8 = 0x01;

pub const DIB_RGB_COLORS: DWORD = 0;
pub const DIB_PAL_COLORS: DWORD = 1;

#[repr(C)]
#[derive(Debug, Default)]
pub struct BLENDFUNCTION {
    pub BlendOp: u8,
    pub BlendFlags: u8,
    pub SourceConstantAlpha: u8,
    pub AlphaFormat: u8,
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
