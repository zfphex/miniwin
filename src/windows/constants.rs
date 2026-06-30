use std::ffi::c_void;

pub const VK_LBUTTON: i32 = 0x01;
pub const VK_RBUTTON: i32 = 0x02;
pub const VK_CANCEL: i32 = 0x03;
pub const VK_MBUTTON: i32 = 0x04;
pub const VK_XBUTTON1: i32 = 0x05;
pub const VK_XBUTTON2: i32 = 0x06;
pub const VK_BACK: i32 = 0x08;
pub const VK_TAB: i32 = 0x09;
pub const VK_CLEAR: i32 = 0x0C;
pub const VK_RETURN: i32 = 0x0D;
pub const VK_SHIFT: i32 = 0x10;
pub const VK_CONTROL: i32 = 0x11;
pub const VK_MENU: i32 = 0x12;
pub const VK_PAUSE: i32 = 0x13;
pub const VK_CAPITAL: i32 = 0x14;
pub const VK_KANA: i32 = 0x15;
pub const VK_HANGEUL: i32 = 0x15;
pub const VK_HANGUL: i32 = 0x15;
pub const VK_JUNJA: i32 = 0x17;
pub const VK_FINAL: i32 = 0x18;
pub const VK_HANJA: i32 = 0x19;
pub const VK_KANJI: i32 = 0x19;
pub const VK_ESCAPE: i32 = 0x1B;
pub const VK_CONVERT: i32 = 0x1C;
pub const VK_NONCONVERT: i32 = 0x1D;
pub const VK_ACCEPT: i32 = 0x1E;
pub const VK_MODECHANGE: i32 = 0x1F;
pub const VK_SPACE: i32 = 0x20;
pub const VK_PRIOR: i32 = 0x21;
pub const VK_NEXT: i32 = 0x22;
pub const VK_END: i32 = 0x23;
pub const VK_HOME: i32 = 0x24;
pub const VK_LEFT: i32 = 0x25;
pub const VK_UP: i32 = 0x26;
pub const VK_RIGHT: i32 = 0x27;
pub const VK_DOWN: i32 = 0x28;
pub const VK_SELECT: i32 = 0x29;
pub const VK_PRINT: i32 = 0x2A;
pub const VK_EXECUTE: i32 = 0x2B;
pub const VK_SNAPSHOT: i32 = 0x2C;
pub const VK_INSERT: i32 = 0x2D;
pub const VK_DELETE: i32 = 0x2E;
pub const VK_HELP: i32 = 0x2F;
pub const VK_LWIN: i32 = 0x5B;
pub const VK_RWIN: i32 = 0x5C;
pub const VK_APPS: i32 = 0x5D;
pub const VK_SLEEP: i32 = 0x5F;
pub const VK_NUMPAD0: i32 = 0x60;
pub const VK_NUMPAD1: i32 = 0x61;
pub const VK_NUMPAD2: i32 = 0x62;
pub const VK_NUMPAD3: i32 = 0x63;
pub const VK_NUMPAD4: i32 = 0x64;
pub const VK_NUMPAD5: i32 = 0x65;
pub const VK_NUMPAD6: i32 = 0x66;
pub const VK_NUMPAD7: i32 = 0x67;
pub const VK_NUMPAD8: i32 = 0x68;
pub const VK_NUMPAD9: i32 = 0x69;
pub const VK_MULTIPLY: i32 = 0x6A;
pub const VK_ADD: i32 = 0x6B;
pub const VK_SEPARATOR: i32 = 0x6C;
pub const VK_SUBTRACT: i32 = 0x6D;
pub const VK_DECIMAL: i32 = 0x6E;
pub const VK_DIVIDE: i32 = 0x6F;
pub const VK_F1: i32 = 0x70;
pub const VK_F2: i32 = 0x71;
pub const VK_F3: i32 = 0x72;
pub const VK_F4: i32 = 0x73;
pub const VK_F5: i32 = 0x74;
pub const VK_F6: i32 = 0x75;
pub const VK_F7: i32 = 0x76;
pub const VK_F8: i32 = 0x77;
pub const VK_F9: i32 = 0x78;
pub const VK_F10: i32 = 0x79;
pub const VK_F11: i32 = 0x7A;
pub const VK_F12: i32 = 0x7B;
pub const VK_F13: i32 = 0x7C;
pub const VK_F14: i32 = 0x7D;
pub const VK_F15: i32 = 0x7E;
pub const VK_F16: i32 = 0x7F;
pub const VK_F17: i32 = 0x80;
pub const VK_F18: i32 = 0x81;
pub const VK_F19: i32 = 0x82;
pub const VK_F20: i32 = 0x83;
pub const VK_F21: i32 = 0x84;
pub const VK_F22: i32 = 0x85;
pub const VK_F23: i32 = 0x86;
pub const VK_F24: i32 = 0x87;
pub const VK_NAVIGATION_VIEW: i32 = 0x88;
pub const VK_NAVIGATION_MENU: i32 = 0x89;
pub const VK_NAVIGATION_UP: i32 = 0x8A;
pub const VK_NAVIGATION_DOWN: i32 = 0x8B;
pub const VK_NAVIGATION_LEFT: i32 = 0x8C;
pub const VK_NAVIGATION_RIGHT: i32 = 0x8D;
pub const VK_NAVIGATION_ACCEPT: i32 = 0x8E;
pub const VK_NAVIGATION_CANCEL: i32 = 0x8F;
pub const VK_NUMLOCK: i32 = 0x90;
pub const VK_SCROLL: i32 = 0x91;
pub const VK_OEM_NEC_EQUAL: i32 = 0x92;
pub const VK_OEM_FJ_JISHO: i32 = 0x92;
pub const VK_OEM_FJ_MASSHOU: i32 = 0x93;
pub const VK_OEM_FJ_TOUROKU: i32 = 0x94;
pub const VK_OEM_FJ_LOYA: i32 = 0x95;
pub const VK_OEM_FJ_ROYA: i32 = 0x96;
pub const VK_LSHIFT: i32 = 0xA0;
pub const VK_RSHIFT: i32 = 0xA1;
pub const VK_LCONTROL: i32 = 0xA2;
pub const VK_RCONTROL: i32 = 0xA3;
pub const VK_LMENU: i32 = 0xA4;
pub const VK_RMENU: i32 = 0xA5;
pub const VK_BROWSER_BACK: i32 = 0xA6;
pub const VK_BROWSER_FORWARD: i32 = 0xA7;
pub const VK_BROWSER_REFRESH: i32 = 0xA8;
pub const VK_BROWSER_STOP: i32 = 0xA9;
pub const VK_BROWSER_SEARCH: i32 = 0xAA;
pub const VK_BROWSER_FAVORITES: i32 = 0xAB;
pub const VK_BROWSER_HOME: i32 = 0xAC;
pub const VK_VOLUME_MUTE: i32 = 0xAD;
pub const VK_VOLUME_DOWN: i32 = 0xAE;
pub const VK_VOLUME_UP: i32 = 0xAF;
pub const VK_MEDIA_NEXT_TRACK: i32 = 0xB0;
pub const VK_MEDIA_PREV_TRACK: i32 = 0xB1;
pub const VK_MEDIA_STOP: i32 = 0xB2;
pub const VK_MEDIA_PLAY_PAUSE: i32 = 0xB3;
pub const VK_LAUNCH_MAIL: i32 = 0xB4;
pub const VK_LAUNCH_MEDIA_SELECT: i32 = 0xB5;
pub const VK_LAUNCH_APP1: i32 = 0xB6;
pub const VK_LAUNCH_APP2: i32 = 0xB7;
pub const VK_OEM_1: i32 = 0xBA;
pub const VK_OEM_PLUS: i32 = 0xBB;
pub const VK_OEM_COMMA: i32 = 0xBC;
pub const VK_OEM_MINUS: i32 = 0xBD;
pub const VK_OEM_PERIOD: i32 = 0xBE;
pub const VK_OEM_2: i32 = 0xBF;
pub const VK_OEM_3: i32 = 0xC0;
pub const VK_GAMEPAD_A: i32 = 0xC3;
pub const VK_GAMEPAD_B: i32 = 0xC4;
pub const VK_GAMEPAD_X: i32 = 0xC5;
pub const VK_GAMEPAD_Y: i32 = 0xC6;
pub const VK_GAMEPAD_RIGHT_SHOULDER: i32 = 0xC7;
pub const VK_GAMEPAD_LEFT_SHOULDER: i32 = 0xC8;
pub const VK_GAMEPAD_LEFT_TRIGGER: i32 = 0xC9;
pub const VK_GAMEPAD_RIGHT_TRIGGER: i32 = 0xCA;
pub const VK_GAMEPAD_DPAD_UP: i32 = 0xCB;
pub const VK_GAMEPAD_DPAD_DOWN: i32 = 0xCC;
pub const VK_GAMEPAD_DPAD_LEFT: i32 = 0xCD;
pub const VK_GAMEPAD_DPAD_RIGHT: i32 = 0xCE;
pub const VK_GAMEPAD_MENU: i32 = 0xCF;
pub const VK_GAMEPAD_VIEW: i32 = 0xD0;
pub const VK_GAMEPAD_LEFT_THUMBSTICK_BUTTON: i32 = 0xD1;
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_BUTTON: i32 = 0xD2;
pub const VK_GAMEPAD_LEFT_THUMBSTICK_UP: i32 = 0xD3;
pub const VK_GAMEPAD_LEFT_THUMBSTICK_DOWN: i32 = 0xD4;
pub const VK_GAMEPAD_LEFT_THUMBSTICK_RIGHT: i32 = 0xD5;
pub const VK_GAMEPAD_LEFT_THUMBSTICK_LEFT: i32 = 0xD6;
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_UP: i32 = 0xD7;
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_DOWN: i32 = 0xD8;
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_RIGHT: i32 = 0xD9;
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_LEFT: i32 = 0xDA;
pub const VK_OEM_4: i32 = 0xDB;
pub const VK_OEM_5: i32 = 0xDC;
pub const VK_OEM_6: i32 = 0xDD;
pub const VK_OEM_7: i32 = 0xDE;
pub const VK_OEM_8: i32 = 0xDF;
pub const VK_OEM_AX: i32 = 0xE1;
pub const VK_OEM_102: i32 = 0xE2;
pub const VK_ICO_HELP: i32 = 0xE3;
pub const VK_ICO_00: i32 = 0xE4;
pub const VK_PROCESSKEY: i32 = 0xE5;
pub const VK_ICO_CLEAR: i32 = 0xE6;
pub const VK_PACKET: i32 = 0xE7;
pub const VK_OEM_RESET: i32 = 0xE9;
pub const VK_OEM_JUMP: i32 = 0xEA;
pub const VK_OEM_PA1: i32 = 0xEB;
pub const VK_OEM_PA2: i32 = 0xEC;
pub const VK_OEM_PA3: i32 = 0xED;
pub const VK_OEM_WSCTRL: i32 = 0xEE;
pub const VK_OEM_CUSEL: i32 = 0xEF;
pub const VK_OEM_ATTN: i32 = 0xF0;
pub const VK_OEM_FINISH: i32 = 0xF1;
pub const VK_OEM_COPY: i32 = 0xF2;
pub const VK_OEM_AUTO: i32 = 0xF3;
pub const VK_OEM_ENLW: i32 = 0xF4;
pub const VK_OEM_BACKTAB: i32 = 0xF5;
pub const VK_ATTN: i32 = 0xF6;
pub const VK_CRSEL: i32 = 0xF7;
pub const VK_EXSEL: i32 = 0xF8;
pub const VK_EREOF: i32 = 0xF9;
pub const VK_PLAY: i32 = 0xFA;
pub const VK_ZOOM: i32 = 0xFB;
pub const VK_NONAME: i32 = 0xFC;
pub const VK_PA1: i32 = 0xFD;
pub const VK_OEM_CLEAR: i32 = 0xFE;

pub const WA_INACTIVE: u16 = 0;
pub const WA_ACTIVE: u16 = 1;
pub const WA_CLICKACTIVE: u16 = 2;

pub const WM_NULL: u32 = 0x0000;
pub const WM_CREATE: u32 = 0x0001;
pub const WM_DESTROY: u32 = 0x0002;
pub const WM_MOVE: u32 = 0x0003;
pub const WM_SIZE: u32 = 0x0005;
pub const WM_ACTIVATE: u32 = 0x0006;
pub const WM_SETFOCUS: u32 = 0x0007;
pub const WM_KILLFOCUS: u32 = 0x0008;
pub const WM_ENABLE: u32 = 0x000A;
pub const WM_SETREDRAW: u32 = 0x000B;
pub const WM_SETTEXT: u32 = 0x000C;
pub const WM_GETTEXT: u32 = 0x000D;
pub const WM_GETTEXTLENGTH: u32 = 0x000E;
pub const WM_PAINT: u32 = 0x000F;
pub const WM_CLOSE: u32 = 0x0010;
pub const WM_QUERYENDSESSION: u32 = 0x0011;
pub const WM_QUERYOPEN: u32 = 0x0013;
pub const WM_ENDSESSION: u32 = 0x0016;
pub const WM_QUIT: u32 = 0x0012;
pub const WM_ERASEBKGND: u32 = 0x0014;
pub const WM_SYSCOLORCHANGE: u32 = 0x0015;
pub const WM_SHOWWINDOW: u32 = 0x0018;
pub const WM_WININICHANGE: u32 = 0x001A;
pub const WM_SETTINGCHANGE: u32 = WM_WININICHANGE;
pub const WM_DEVMODECHANGE: u32 = 0x001B;
pub const WM_ACTIVATEAPP: u32 = 0x001C;
pub const WM_FONTCHANGE: u32 = 0x001D;
pub const WM_TIMECHANGE: u32 = 0x001E;
pub const WM_CANCELMODE: u32 = 0x001F;
pub const WM_SETCURSOR: u32 = 0x0020;
pub const WM_MOUSEACTIVATE: u32 = 0x0021;
pub const WM_CHILDACTIVATE: u32 = 0x0022;
pub const WM_QUEUESYNC: u32 = 0x0023;
pub const WM_GETMINMAXINFO: u32 = 0x0024;
pub const WM_PAINTICON: u32 = 0x0026;
pub const WM_ICONERASEBKGND: u32 = 0x0027;
pub const WM_NEXTDLGCTL: u32 = 0x0028;
pub const WM_SPOOLERSTATUS: u32 = 0x002A;
pub const WM_DRAWITEM: u32 = 0x002B;
pub const WM_MEASUREITEM: u32 = 0x002C;
pub const WM_DELETEITEM: u32 = 0x002D;
pub const WM_VKEYTOITEM: u32 = 0x002E;
pub const WM_CHARTOITEM: u32 = 0x002F;
pub const WM_SETFONT: u32 = 0x0030;
pub const WM_GETFONT: u32 = 0x0031;
pub const WM_SETHOTKEY: u32 = 0x0032;
pub const WM_GETHOTKEY: u32 = 0x0033;
pub const WM_QUERYDRAGICON: u32 = 0x0037;
pub const WM_COMPAREITEM: u32 = 0x0039;
pub const WM_GETOBJECT: u32 = 0x003D;
pub const WM_COMPACTING: u32 = 0x0041;
pub const WM_COMMNOTIFY: u32 = 0x0044;
pub const WM_WINDOWPOSCHANGING: u32 = 0x0046;
pub const WM_WINDOWPOSCHANGED: u32 = 0x0047;
pub const WM_POWER: u32 = 0x0048;
pub const WM_COPYDATA: u32 = 0x004A;
pub const WM_CANCELJOURNAL: u32 = 0x004B;
pub const WM_NOTIFY: u32 = 0x004E;
pub const WM_INPUTLANGCHANGEREQUEST: u32 = 0x0050;
pub const WM_INPUTLANGCHANGE: u32 = 0x0051;
pub const WM_TCARD: u32 = 0x0052;
pub const WM_HELP: u32 = 0x0053;
pub const WM_USERCHANGED: u32 = 0x0054;
pub const WM_NOTIFYFORMAT: u32 = 0x0055;
pub const WM_CONTEXTMENU: u32 = 0x007B;
pub const WM_STYLECHANGING: u32 = 0x007C;
pub const WM_STYLECHANGED: u32 = 0x007D;
pub const WM_DISPLAYCHANGE: u32 = 0x007E;
pub const WM_GETICON: u32 = 0x007F;
pub const WM_SETICON: u32 = 0x0080;
pub const WM_NCCREATE: u32 = 0x0081;
pub const WM_NCDESTROY: u32 = 0x0082;
pub const WM_NCCALCSIZE: u32 = 0x0083;
pub const WM_NCHITTEST: u32 = 0x0084;
pub const WM_NCPAINT: u32 = 0x0085;
pub const WM_NCACTIVATE: u32 = 0x0086;
pub const WM_GETDLGCODE: u32 = 0x0087;
pub const WM_SYNCPAINT: u32 = 0x0088;
pub const WM_NCMOUSEMOVE: u32 = 0x00A0;
pub const WM_NCLBUTTONDOWN: u32 = 0x00A1;
pub const WM_NCLBUTTONUP: u32 = 0x00A2;
pub const WM_NCLBUTTONDBLCLK: u32 = 0x00A3;
pub const WM_NCRBUTTONDOWN: u32 = 0x00A4;
pub const WM_NCRBUTTONUP: u32 = 0x00A5;
pub const WM_NCRBUTTONDBLCLK: u32 = 0x00A6;
pub const WM_NCMBUTTONDOWN: u32 = 0x00A7;
pub const WM_NCMBUTTONUP: u32 = 0x00A8;
pub const WM_NCMBUTTONDBLCLK: u32 = 0x00A9;
pub const WM_NCXBUTTONDOWN: u32 = 0x00AB;
pub const WM_NCXBUTTONUP: u32 = 0x00AC;
pub const WM_NCXBUTTONDBLCLK: u32 = 0x00AD;
pub const WM_INPUT_DEVICE_CHANGE: u32 = 0x00FE;
pub const WM_INPUT: u32 = 0x00FF;
pub const WM_KEYFIRST: u32 = 0x0100;
pub const WM_KEYDOWN: u32 = 0x0100;
pub const WM_KEYUP: u32 = 0x0101;
pub const WM_CHAR: u32 = 0x0102;
pub const WM_DEADCHAR: u32 = 0x0103;
pub const WM_SYSKEYDOWN: u32 = 0x0104;
pub const WM_SYSKEYUP: u32 = 0x0105;
pub const WM_SYSCHAR: u32 = 0x0106;
pub const WM_SYSDEADCHAR: u32 = 0x0107;
pub const WM_UNICHAR: u32 = 0x0109;
pub const WM_KEYLAST: u32 = 0x0109;
pub const WM_IME_STARTCOMPOSITION: u32 = 0x010D;
pub const WM_IME_ENDCOMPOSITION: u32 = 0x010E;
pub const WM_IME_COMPOSITION: u32 = 0x010F;
pub const WM_IME_KEYLAST: u32 = 0x010F;
pub const WM_INITDIALOG: u32 = 0x0110;
pub const WM_COMMAND: u32 = 0x0111;
pub const WM_SYSCOMMAND: u32 = 0x0112;
pub const WM_TIMER: u32 = 0x0113;
pub const WM_HSCROLL: u32 = 0x0114;
pub const WM_VSCROLL: u32 = 0x0115;
pub const WM_INITMENU: u32 = 0x0116;
pub const WM_INITMENUPOPUP: u32 = 0x0117;
pub const WM_GESTURE: u32 = 0x0119;
pub const WM_GESTURENOTIFY: u32 = 0x011A;
pub const WM_MENUSELECT: u32 = 0x011F;
pub const WM_MENUCHAR: u32 = 0x0120;
pub const WM_ENTERIDLE: u32 = 0x0121;
pub const WM_MENURBUTTONUP: u32 = 0x0122;
pub const WM_MENUDRAG: u32 = 0x0123;
pub const WM_MENUGETOBJECT: u32 = 0x0124;
pub const WM_UNINITMENUPOPUP: u32 = 0x0125;
pub const WM_MENUCOMMAND: u32 = 0x0126;
pub const WM_CHANGEUISTATE: u32 = 0x0127;
pub const WM_UPDATEUISTATE: u32 = 0x0128;
pub const WM_QUERYUISTATE: u32 = 0x0129;
pub const WM_CTLCOLORMSGBOX: u32 = 0x0132;
pub const WM_CTLCOLOREDIT: u32 = 0x0133;
pub const WM_CTLCOLORLISTBOX: u32 = 0x0134;
pub const WM_CTLCOLORBTN: u32 = 0x0135;
pub const WM_CTLCOLORDLG: u32 = 0x0136;
pub const WM_CTLCOLORSCROLLBAR: u32 = 0x0137;
pub const WM_CTLCOLORSTATIC: u32 = 0x0138;
pub const MN_GETHMENU: u32 = 0x01E1;
pub const WM_MOUSEFIRST: u32 = 0x0200;
pub const WM_MOUSEMOVE: u32 = 0x0200;
pub const WM_LBUTTONDOWN: u32 = 0x0201;
pub const WM_LBUTTONUP: u32 = 0x0202;
pub const WM_LBUTTONDBLCLK: u32 = 0x0203;
pub const WM_RBUTTONDOWN: u32 = 0x0204;
pub const WM_RBUTTONUP: u32 = 0x0205;
pub const WM_RBUTTONDBLCLK: u32 = 0x0206;
pub const WM_MBUTTONDOWN: u32 = 0x0207;
pub const WM_MBUTTONUP: u32 = 0x0208;
pub const WM_MBUTTONDBLCLK: u32 = 0x0209;
pub const WM_MOUSEWHEEL: u32 = 0x020A;
pub const WM_XBUTTONDOWN: u32 = 0x020B;
pub const WM_XBUTTONUP: u32 = 0x020C;
pub const WM_XBUTTONDBLCLK: u32 = 0x020D;
pub const WM_MOUSEHWHEEL: u32 = 0x020E;
pub const WM_MOUSELAST: u32 = 0x020E;
pub const WM_PARENTNOTIFY: u32 = 0x0210;
pub const WM_ENTERMENULOOP: u32 = 0x0211;
pub const WM_EXITMENULOOP: u32 = 0x0212;
pub const WM_NEXTMENU: u32 = 0x0213;
pub const WM_SIZING: u32 = 0x0214;
pub const WM_CAPTURECHANGED: u32 = 0x0215;
pub const WM_MOVING: u32 = 0x0216;
pub const WM_POWERBROADCAST: u32 = 0x0218;
pub const WM_DEVICECHANGE: u32 = 0x0219;
pub const WM_MDICREATE: u32 = 0x0220;
pub const WM_MDIDESTROY: u32 = 0x0221;
pub const WM_MDIACTIVATE: u32 = 0x0222;
pub const WM_MDIRESTORE: u32 = 0x0223;
pub const WM_MDINEXT: u32 = 0x0224;
pub const WM_MDIMAXIMIZE: u32 = 0x0225;
pub const WM_MDITILE: u32 = 0x0226;
pub const WM_MDICASCADE: u32 = 0x0227;
pub const WM_MDIICONARRANGE: u32 = 0x0228;
pub const WM_MDIGETACTIVE: u32 = 0x0229;
pub const WM_MDISETMENU: u32 = 0x0230;
pub const WM_ENTERSIZEMOVE: u32 = 0x0231;
pub const WM_EXITSIZEMOVE: u32 = 0x0232;
pub const WM_DROPFILES: u32 = 0x0233;
pub const WM_MDIREFRESHMENU: u32 = 0x0234;
pub const WM_POINTERDEVICECHANGE: u32 = 0x238;
pub const WM_POINTERDEVICEINRANGE: u32 = 0x239;
pub const WM_POINTERDEVICEOUTOFRANGE: u32 = 0x23A;
pub const WM_TOUCH: u32 = 0x0240;
pub const WM_NCPOINTERUPDATE: u32 = 0x0241;
pub const WM_NCPOINTERDOWN: u32 = 0x0242;
pub const WM_NCPOINTERUP: u32 = 0x0243;
pub const WM_POINTERUPDATE: u32 = 0x0245;
pub const WM_POINTERDOWN: u32 = 0x0246;
pub const WM_POINTERUP: u32 = 0x0247;
pub const WM_POINTERENTER: u32 = 0x0249;
pub const WM_POINTERLEAVE: u32 = 0x024A;
pub const WM_POINTERACTIVATE: u32 = 0x024B;
pub const WM_POINTERCAPTURECHANGED: u32 = 0x024C;
pub const WM_TOUCHHITTESTING: u32 = 0x024D;
pub const WM_POINTERWHEEL: u32 = 0x024E;
pub const WM_POINTERHWHEEL: u32 = 0x024F;
pub const DM_POINTERHITTEST: u32 = 0x0250;
pub const WM_POINTERROUTEDTO: u32 = 0x0251;
pub const WM_POINTERROUTEDAWAY: u32 = 0x0252;
pub const WM_POINTERROUTEDRELEASED: u32 = 0x0253;
pub const WM_IME_SETCONTEXT: u32 = 0x0281;
pub const WM_IME_NOTIFY: u32 = 0x0282;
pub const WM_IME_CONTROL: u32 = 0x0283;
pub const WM_IME_COMPOSITIONFULL: u32 = 0x0284;
pub const WM_IME_SELECT: u32 = 0x0285;
pub const WM_IME_CHAR: u32 = 0x0286;
pub const WM_IME_REQUEST: u32 = 0x0288;
pub const WM_IME_KEYDOWN: u32 = 0x0290;
pub const WM_IME_KEYUP: u32 = 0x0291;
pub const WM_MOUSEHOVER: u32 = 0x02A1;
pub const WM_MOUSELEAVE: u32 = 0x02A3;
pub const WM_NCMOUSEHOVER: u32 = 0x02A0;
pub const WM_NCMOUSELEAVE: u32 = 0x02A2;
pub const WM_WTSSESSION_CHANGE: u32 = 0x02B1;
pub const WM_TABLET_FIRST: u32 = 0x02c0;
pub const WM_TABLET_LAST: u32 = 0x02df;
pub const WM_DPICHANGED: u32 = 0x02E0;
pub const WM_DPICHANGED_BEFOREPARENT: u32 = 0x02E2;
pub const WM_DPICHANGED_AFTERPARENT: u32 = 0x02E3;
pub const WM_GETDPISCALEDSIZE: u32 = 0x02E4;
pub const WM_CUT: u32 = 0x0300;
pub const WM_COPY: u32 = 0x0301;
pub const WM_PASTE: u32 = 0x0302;
pub const WM_CLEAR: u32 = 0x0303;
pub const WM_UNDO: u32 = 0x0304;
pub const WM_RENDERFORMAT: u32 = 0x0305;
pub const WM_RENDERALLFORMATS: u32 = 0x0306;
pub const WM_DESTROYCLIPBOARD: u32 = 0x0307;
pub const WM_DRAWCLIPBOARD: u32 = 0x0308;
pub const WM_PAINTCLIPBOARD: u32 = 0x0309;
pub const WM_VSCROLLCLIPBOARD: u32 = 0x030A;
pub const WM_SIZECLIPBOARD: u32 = 0x030B;
pub const WM_ASKCBFORMATNAME: u32 = 0x030C;
pub const WM_CHANGECBCHAIN: u32 = 0x030D;
pub const WM_HSCROLLCLIPBOARD: u32 = 0x030E;
pub const WM_QUERYNEWPALETTE: u32 = 0x030F;
pub const WM_PALETTEISCHANGING: u32 = 0x0310;
pub const WM_PALETTECHANGED: u32 = 0x0311;
pub const WM_HOTKEY: u32 = 0x0312;
pub const WM_PRINT: u32 = 0x0317;
pub const WM_PRINTCLIENT: u32 = 0x0318;
pub const WM_APPCOMMAND: u32 = 0x0319;
pub const WM_THEMECHANGED: u32 = 0x031A;
pub const WM_CLIPBOARDUPDATE: u32 = 0x031D;
pub const WM_DWMCOMPOSITIONCHANGED: u32 = 0x031E;
pub const WM_DWMNCRENDERINGCHANGED: u32 = 0x031F;
pub const WM_DWMCOLORIZATIONCOLORCHANGED: u32 = 0x0320;
pub const WM_DWMWINDOWMAXIMIZEDCHANGE: u32 = 0x0321;
pub const WM_DWMSENDICONICTHUMBNAIL: u32 = 0x0323;
pub const WM_DWMSENDICONICLIVEPREVIEWBITMAP: u32 = 0x0326;
pub const WM_GETTITLEBARINFOEX: u32 = 0x033F;
pub const WM_HANDHELDFIRST: u32 = 0x0358;
pub const WM_HANDHELDLAST: u32 = 0x035F;
pub const WM_AFXFIRST: u32 = 0x0360;
pub const WM_AFXLAST: u32 = 0x037F;
pub const WM_PENWINFIRST: u32 = 0x0380;
pub const WM_PENWINLAST: u32 = 0x038F;
pub const WM_APP: u32 = 0x8000;
pub const WM_USER: u32 = 0x0400;

pub const MK_LBUTTON: u32 = 0x0001;
pub const MK_RBUTTON: u32 = 0x0002;
pub const MK_SHIFT: u32 = 0x0004;
pub const MK_CONTROL: u32 = 0x0008;
pub const MK_MBUTTON: u32 = 0x0010;
pub const MK_XBUTTON1: u32 = 0x0020;
pub const MK_XBUTTON2: u32 = 0x0040;

pub const GWLP_WNDPROC: i32 = -4;
pub const GWLP_HINSTANCE: i32 = -6;
pub const GWLP_HWNDPARENT: i32 = -8;
pub const GWLP_USERDATA: i32 = -21;
pub const GWLP_ID: i32 = -12;

pub const SM_CXSCREEN: i32 = 0;
pub const SM_CYSCREEN: i32 = 1;
pub const SM_CXVSCROLL: i32 = 2;
pub const SM_CYHSCROLL: i32 = 3;
pub const SM_CYCAPTION: i32 = 4;
pub const SM_CXBORDER: i32 = 5;
pub const SM_CYBORDER: i32 = 6;
pub const SM_CXDLGFRAME: i32 = 7;
pub const SM_CYDLGFRAME: i32 = 8;
pub const SM_CYVTHUMB: i32 = 9;
pub const SM_CXHTHUMB: i32 = 10;
pub const SM_CXICON: i32 = 11;
pub const SM_CYICON: i32 = 12;
pub const SM_CXCURSOR: i32 = 13;
pub const SM_CYCURSOR: i32 = 14;
pub const SM_CYMENU: i32 = 15;
pub const SM_CXFULLSCREEN: i32 = 16;
pub const SM_CYFULLSCREEN: i32 = 17;
pub const SM_CYKANJIWINDOW: i32 = 18;
pub const SM_MOUSEPRESENT: i32 = 19;
pub const SM_CYVSCROLL: i32 = 20;
pub const SM_CXHSCROLL: i32 = 21;
pub const SM_DEBUG: i32 = 22;
pub const SM_SWAPBUTTON: i32 = 23;
pub const SM_RESERVED1: i32 = 24;
pub const SM_RESERVED2: i32 = 25;
pub const SM_RESERVED3: i32 = 26;
pub const SM_RESERVED4: i32 = 27;
pub const SM_CXMIN: i32 = 28;
pub const SM_CYMIN: i32 = 29;
pub const SM_CXSIZE: i32 = 30;
pub const SM_CYSIZE: i32 = 31;
pub const SM_CXFRAME: i32 = 32;
pub const SM_CYFRAME: i32 = 33;
pub const SM_CXMINTRACK: i32 = 34;
pub const SM_CYMINTRACK: i32 = 35;
pub const SM_CXDOUBLECLK: i32 = 36;
pub const SM_CYDOUBLECLK: i32 = 37;
pub const SM_CXICONSPACING: i32 = 38;
pub const SM_CYICONSPACING: i32 = 39;
pub const SM_MENUDROPALIGNMENT: i32 = 40;
pub const SM_PENWINDOWS: i32 = 41;
pub const SM_DBCSENABLED: i32 = 42;
pub const SM_CMOUSEBUTTONS: i32 = 43;
pub const SM_CXFIXEDFRAME: i32 = SM_CXDLGFRAME;
pub const SM_CYFIXEDFRAME: i32 = SM_CYDLGFRAME;
pub const SM_CXSIZEFRAME: i32 = SM_CXFRAME;
pub const SM_CYSIZEFRAME: i32 = SM_CYFRAME;
pub const SM_SECURE: i32 = 44;
pub const SM_CXEDGE: i32 = 45;
pub const SM_CYEDGE: i32 = 46;
pub const SM_CXMINSPACING: i32 = 47;
pub const SM_CYMINSPACING: i32 = 48;
pub const SM_CXSMICON: i32 = 49;
pub const SM_CYSMICON: i32 = 50;
pub const SM_CYSMCAPTION: i32 = 51;
pub const SM_CXSMSIZE: i32 = 52;
pub const SM_CYSMSIZE: i32 = 53;
pub const SM_CXMENUSIZE: i32 = 54;
pub const SM_CYMENUSIZE: i32 = 55;
pub const SM_ARRANGE: i32 = 56;
pub const SM_CXMINIMIZED: i32 = 57;
pub const SM_CYMINIMIZED: i32 = 58;
pub const SM_CXMAXTRACK: i32 = 59;
pub const SM_CYMAXTRACK: i32 = 60;
pub const SM_CXMAXIMIZED: i32 = 61;
pub const SM_CYMAXIMIZED: i32 = 62;
pub const SM_NETWORK: i32 = 63;
pub const SM_CLEANBOOT: i32 = 67;
pub const SM_CXDRAG: i32 = 68;
pub const SM_CYDRAG: i32 = 69;
pub const SM_SHOWSOUNDS: i32 = 70;
pub const SM_CXMENUCHECK: i32 = 71;
pub const SM_CYMENUCHECK: i32 = 72;
pub const SM_SLOWMACHINE: i32 = 73;
pub const SM_MIDEASTENABLED: i32 = 74;
pub const SM_MOUSEWHEELPRESENT: i32 = 75;
pub const SM_XVIRTUALSCREEN: i32 = 76;
pub const SM_YVIRTUALSCREEN: i32 = 77;
pub const SM_CXVIRTUALSCREEN: i32 = 78;
pub const SM_CYVIRTUALSCREEN: i32 = 79;
pub const SM_CMONITORS: i32 = 80;
pub const SM_SAMEDISPLAYFORMAT: i32 = 81;
pub const SM_IMMENABLED: i32 = 82;
pub const SM_CXFOCUSBORDER: i32 = 83;
pub const SM_CYFOCUSBORDER: i32 = 84;
pub const SM_TABLETPC: i32 = 86;
pub const SM_MEDIACENTER: i32 = 87;
pub const SM_STARTER: i32 = 88;
pub const SM_SERVERR2: i32 = 89;
pub const SM_MOUSEHORIZONTALWHEELPRESENT: i32 = 91;
pub const SM_CXPADDEDBORDER: i32 = 92;
pub const SM_DIGITIZER: i32 = 94;
pub const SM_MAXIMUMTOUCHES: i32 = 95;
pub const SM_CMETRICS: i32 = 97;
pub const SM_REMOTESESSION: i32 = 0x1000;
pub const SM_SHUTTINGDOWN: i32 = 0x2000;
pub const SM_REMOTECONTROL: i32 = 0x2001;
pub const SM_CARETBLINKINGENABLED: i32 = 0x2002;
pub const SM_CONVERTIBLESLATEMODE: i32 = 0x2003;
pub const SM_SYSTEMDOCKED: i32 = 0x2004;

pub const HTERROR: isize = -2;
pub const HTTRANSPARENT: isize = -1;
pub const HTNOWHERE: isize = 0;
pub const HTCLIENT: isize = 1;
pub const HTCAPTION: isize = 2;
pub const HTSYSMENU: isize = 3;
pub const HTGROWBOX: isize = 4;
pub const HTSIZE: isize = HTGROWBOX;
pub const HTMENU: isize = 5;
pub const HTHSCROLL: isize = 6;
pub const HTVSCROLL: isize = 7;
pub const HTMINBUTTON: isize = 8;
pub const HTMAXBUTTON: isize = 9;
pub const HTLEFT: isize = 10;
pub const HTRIGHT: isize = 11;
pub const HTTOP: isize = 12;
pub const HTTOPLEFT: isize = 13;
pub const HTTOPRIGHT: isize = 14;
pub const HTBOTTOM: isize = 15;
pub const HTBOTTOMLEFT: isize = 16;
pub const HTBOTTOMRIGHT: isize = 17;
pub const HTBORDER: isize = 18;
pub const HTREDUCE: isize = HTMINBUTTON;
pub const HTZOOM: isize = HTMAXBUTTON;
pub const HTSIZEFIRST: isize = HTLEFT;
pub const HTSIZELAST: isize = HTBOTTOMRIGHT;
pub const HTOBJECT: isize = 19;
pub const HTCLOSE: isize = 20;
pub const HTHELP: isize = 21;

pub const PM_NOREMOVE: u32 = 0x0000;
pub const PM_REMOVE: u32 = 0x0001;
pub const PM_NOYIELD: u32 = 0x0002;

pub const WS_BORDER: u32 = 0x00800000;
pub const WS_CAPTION: u32 = 0x00C00000;
pub const WS_CHILD: u32 = 0x40000000;
pub const WS_CHILDWINDOW: u32 = WS_CHILD;
pub const WS_CLIPCHILDREN: u32 = 0x02000000;
pub const WS_CLIPSIBLINGS: u32 = 0x04000000;
pub const WS_DISABLED: u32 = 0x08000000;
pub const WS_DLGFRAME: u32 = 0x00400000;
pub const WS_GROUP: u32 = 0x00020000;
pub const WS_HSCROLL: u32 = 0x00100000;
pub const WS_ICONIC: u32 = WS_MINIMIZE;
pub const WS_MAXIMIZE: u32 = 0x01000000;
pub const WS_MAXIMIZEBOX: u32 = 0x00010000;
pub const WS_MINIMIZE: u32 = 0x20000000;
pub const WS_MINIMIZEBOX: u32 = 0x00020000;
pub const WS_OVERLAPPED: u32 = 0x00000000;
pub const WS_OVERLAPPEDWINDOW: u32 =
    WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_THICKFRAME | WS_MINIMIZEBOX | WS_MAXIMIZEBOX;
pub const WS_POPUP: u32 = 0x80000000;
pub const WS_POPUPWINDOW: u32 = WS_POPUP | WS_BORDER | WS_SYSMENU;
pub const WS_SIZEBOX: u32 = WS_THICKFRAME;
pub const WS_SYSMENU: u32 = 0x00080000;
pub const WS_TABSTOP: u32 = 0x00010000;
pub const WS_THICKFRAME: u32 = 0x00040000;
pub const WS_TILED: u32 = WS_OVERLAPPED;
pub const WS_TILEDWINDOW: u32 = WS_OVERLAPPEDWINDOW;
pub const WS_VISIBLE: u32 = 0x10000000;
pub const WS_VSCROLL: u32 = 0x00200000;

pub const WS_EX_DLGMODALFRAME: u32 = 0x00000001;
pub const WS_EX_NOPARENTNOTIFY: u32 = 0x00000004;
pub const WS_EX_TOPMOST: u32 = 0x00000008;
pub const WS_EX_ACCEPTFILES: u32 = 0x00000010;
pub const WS_EX_TRANSPARENT: u32 = 0x00000020;
pub const WS_EX_MDICHILD: u32 = 0x00000040;
pub const WS_EX_TOOLWINDOW: u32 = 0x00000080;
pub const WS_EX_WINDOWEDGE: u32 = 0x00000100;
pub const WS_EX_CLIENTEDGE: u32 = 0x00000200;
pub const WS_EX_CONTEXTHELP: u32 = 0x00000400;
pub const WS_EX_RIGHT: u32 = 0x00001000;
pub const WS_EX_LEFT: u32 = 0x00000000;
pub const WS_EX_RTLREADING: u32 = 0x00002000;
pub const WS_EX_LTRREADING: u32 = 0x00000000;
pub const WS_EX_LEFTSCROLLBAR: u32 = 0x00004000;
pub const WS_EX_RIGHTSCROLLBAR: u32 = 0x00000000;
pub const WS_EX_CONTROLPARENT: u32 = 0x00010000;
pub const WS_EX_STATICEDGE: u32 = 0x00020000;
pub const WS_EX_APPWINDOW: u32 = 0x00040000;
pub const WS_EX_OVERLAPPEDWINDOW: u32 = WS_EX_WINDOWEDGE | WS_EX_CLIENTEDGE;
pub const WS_EX_PALETTEWINDOW: u32 = WS_EX_WINDOWEDGE | WS_EX_TOOLWINDOW | WS_EX_TOPMOST;
pub const WS_EX_LAYERED: u32 = 0x00080000;
pub const WS_EX_NOINHERITLAYOUT: u32 = 0x00100000;
pub const WS_EX_NOREDIRECTIONBITMAP: u32 = 0x00200000;
pub const WS_EX_LAYOUTRTL: u32 = 0x00400000;
pub const WS_EX_COMPOSITED: u32 = 0x02000000;
pub const WS_EX_NOACTIVATE: u32 = 0x08000000;

pub const CS_BYTEALIGNCLIENT: u32 = 0x1000;
pub const CS_BYTEALIGNWINDOW: u32 = 0x2000;
pub const CS_CLASSDC: u32 = 0x0040;
pub const CS_DBLCLKS: u32 = 0x0008;
pub const CS_DROPSHADOW: u32 = 0x00020000;
pub const CS_GLOBALCLASS: u32 = 0x4000;
pub const CS_HREDRAW: u32 = 0x0002;
pub const CS_NOCLOSE: u32 = 0x0200;
pub const CS_OWNDC: u32 = 0x0020;
pub const CS_PARENTDC: u32 = 0x0080;
pub const CS_SAVEBITS: u32 = 0x0800;
pub const CS_VREDRAW: u32 = 0x0001;

pub const CW_USEDEFAULT: i32 = -2147483648i32;

pub const IDC_ARROW: *const u16 = 32512 as *const u16;
pub const IDC_IBEAM: *const u16 = 32513 as *const u16;
pub const IDC_WAIT: *const u16 = 32514 as *const u16;
pub const IDC_CROSS: *const u16 = 32515 as *const u16;
pub const IDC_UPARROW: *const u16 = 32516 as *const u16;
pub const IDC_SIZE: *const u16 = 32640 as *const u16;
pub const IDC_ICON: *const u16 = 32641 as *const u16;
pub const IDC_SIZENWSE: *const u16 = 32642 as *const u16;
pub const IDC_SIZENESW: *const u16 = 32643 as *const u16;
pub const IDC_SIZEWE: *const u16 = 32644 as *const u16;
pub const IDC_SIZENS: *const u16 = 32645 as *const u16;
pub const IDC_SIZEALL: *const u16 = 32646 as *const u16;
pub const IDC_NO: *const u16 = 32648 as *const u16;
pub const IDC_HAND: *const u16 = 32649 as *const u16;
pub const IDC_APPSTARTING: *const u16 = 32650 as *const u16;
pub const IDC_HELP: *const u16 = 32651 as *const u16;

pub const SW_HIDE: i32 = 0;
pub const SW_SHOWNORMAL: i32 = 1;
pub const SW_NORMAL: i32 = 1;
pub const SW_SHOWMINIMIZED: i32 = 2;
pub const SW_SHOWMAXIMIZED: i32 = 3;
pub const SW_MAXIMIZE: i32 = 3;
pub const SW_SHOWNOACTIVATE: i32 = 4;
pub const SW_SHOW: i32 = 5;
pub const SW_MINIMIZE: i32 = 6;
pub const SW_SHOWMINNOACTIVE: i32 = 7;
pub const SW_SHOWNA: i32 = 8;
pub const SW_RESTORE: i32 = 9;
pub const SW_SHOWDEFAULT: i32 = 10;
pub const SW_FORCEMINIMIZE: i32 = 11;
pub const SW_MAX: i32 = 11;

pub const GWL_EXSTYLE: i32 = -20;
pub const GWL_STYLE: i32 = -16;

pub const SWP_NOSIZE: u32 = 0x0001;
pub const SWP_NOMOVE: u32 = 0x0002;
pub const SWP_NOZORDER: u32 = 0x0004;
pub const SWP_NOREDRAW: u32 = 0x0008;
pub const SWP_NOACTIVATE: u32 = 0x0010;
pub const SWP_FRAMECHANGED: u32 = 0x0020;
pub const SWP_SHOWWINDOW: u32 = 0x0040;
pub const SWP_HIDEWINDOW: u32 = 0x0080;
pub const SWP_NOCOPYBITS: u32 = 0x0100;
pub const SWP_NOOWNERZORDER: u32 = 0x0200;
pub const SWP_NOSENDCHANGING: u32 = 0x0400;
pub const SWP_DRAWFRAME: u32 = SWP_FRAMECHANGED;
pub const SWP_NOREPOSITION: u32 = SWP_NOOWNERZORDER;
pub const SWP_DEFERERASE: u32 = 0x2000;
pub const SWP_ASYNCWINDOWPOS: u32 = 0x4000;

pub const DPI_AWARENESS_UNAWARE: *mut c_void = -1isize as *mut c_void;
pub const DPI_AWARENESS_SYSTEM_AWARE: *mut c_void = -2isize as *mut c_void;
pub const DPI_AWARENESS_PER_MONITOR_AWARE: *mut c_void = -3isize as *mut c_void;
pub const DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2: *mut c_void = -4isize as *mut c_void;
pub const DPI_AWARENESS_CONTEXT_UNAWARE_GDISCALED: *mut c_void = -5isize as *mut c_void;

pub const LWA_ALPHA: u32 = 0x00000002;
pub const LWA_COLORKEY: u32 = 0x00000001;

///https://rust-tutorials.github.io/triangle-from-scratch/appendix/utf16_literals.html
///
///```
///let wstring = utf!("UTF-16");
///```
#[macro_export]
macro_rules! utf16 {
    ($text:expr) => {{
        pub const fn break_off_code_point(utf8: &[u8]) -> Option<(u32, &[u8])> {
            match utf8 {
                [a @ 0b00000000..=0b01111111, rest @ ..] => {
                    // one byte
                    Some((*a as u32, rest))
                }
                [a @ 0b11000000..=0b11011111, b, rest @ ..] => {
                    // two bytes
                    let lead = (*a & 0b11111) as u32;
                    let trail = (*b & 0b111111) as u32;
                    Some((lead << 6 | trail, rest))
                }
                [a @ 0b11100000..=0b11101111, b, c, rest @ ..] => {
                    // three bytes
                    let lead = (*a & 0b1111) as u32;
                    let trail1 = (*b & 0b111111) as u32;
                    let trail2 = (*c & 0b111111) as u32;
                    let out = lead << 12 | trail1 << 6 | trail2;
                    Some((out, rest))
                }
                [a @ 0b11110000..=0b11110111, b, c, d, rest @ ..] => {
                    // four bytes
                    let lead = (*a & 0b111) as u32;
                    let trail1 = (*b & 0b111111) as u32;
                    let trail2 = (*c & 0b111111) as u32;
                    let trail3 = (*d & 0b111111) as u32;
                    let out = lead << 18 | trail1 << 12 | trail2 << 6 | trail3;
                    Some((out, rest))
                }
                [..] => None, /* default */
            }
        }
        pub const fn count_utf16_code_units(s: &str) -> usize {
            let mut bytes = s.as_bytes();
            let mut len = 0;
            while let Some((u, rest)) = break_off_code_point(bytes) {
                len += if u <= 0xFFFF { 1 } else { 2 };
                bytes = rest;
            }
            len
        }
        // Here we pick a name highly unlikely to exist in the scope
        // that $text came from, which prevents a potential const eval cycle error.
        const __A1B2C3D4_CONST_EVAL_LOOP_BREAK: &str = $text;
        const UTF8: &str = __A1B2C3D4_CONST_EVAL_LOOP_BREAK;
        const OUT_BUFFER_LEN: usize = count_utf16_code_units(UTF8);
        const UTF16: [u16; OUT_BUFFER_LEN] = {
            let mut buffer = [0u16; OUT_BUFFER_LEN];
            let mut bytes = UTF8.as_bytes();
            let mut i = 0;
            while let Some((u, rest)) = break_off_code_point(bytes) {
                if u <= 0xFFFF {
                    buffer[i] = u as u16;
                    i += 1;
                } else {
                    let code = u - 0x1_0000;
                    buffer[i] = 0xD800 | ((code >> 10) as u16);
                    buffer[i + 1] = 0xDC00 | ((code & 0x3FF) as u16);
                    i += 2;
                }
                bytes = rest;
            }
            buffer
        };
        UTF16
    }};
}

pub fn wm_code_name(msg: u32) -> &'static str {
    for &(code, name) in WM_NAMES {
        if code == msg {
            return name;
        }
    }

    match msg {
        0x0000..=0x03FF => "WM_RESERVED_SYSTEM",
        0x0400..=0x7FFF => "WM_USER_RANGE",
        0x8000..=0xBFFF => "WM_APP_RANGE",
        0xC000..=0xFFFF => "WM_REGISTERED_STRING_RANGE",
        _ => "WM_RESERVED_GT_0xFFFF",
    }
}

pub static WM_NAMES: &[(u32, &str)] = &[
    (0x0000, "WM_NULL"),
    (0x0001, "WM_CREATE"),
    (0x0002, "WM_DESTROY"),
    (0x0003, "WM_MOVE"),
    (0x0005, "WM_SIZE"),
    (0x0006, "WM_ACTIVATE"),
    (0x0007, "WM_SETFOCUS"),
    (0x0008, "WM_KILLFOCUS"),
    (0x000A, "WM_ENABLE"),
    (0x000B, "WM_SETREDRAW"),
    (0x000C, "WM_SETTEXT"),
    (0x000D, "WM_GETTEXT"),
    (0x000E, "WM_GETTEXTLENGTH"),
    (0x000F, "WM_PAINT"),
    (0x0010, "WM_CLOSE"),
    (0x0011, "WM_QUERYENDSESSION"),
    (0x0012, "WM_QUIT"),
    (0x0013, "WM_QUERYOPEN"),
    (0x0014, "WM_ERASEBKGND"),
    (0x0015, "WM_SYSCOLORCHANGE"),
    (0x0016, "WM_ENDSESSION"),
    (0x0018, "WM_SHOWWINDOW"),
    (0x001A, "WM_SETTINGCHANGE"),
    (0x001A, "WM_WININICHANGE"),
    (0x001B, "WM_DEVMODECHANGE"),
    (0x001C, "WM_ACTIVATEAPP"),
    (0x001D, "WM_FONTCHANGE"),
    (0x001E, "WM_TIMECHANGE"),
    (0x001F, "WM_CANCELMODE"),
    (0x0020, "WM_SETCURSOR"),
    (0x0021, "WM_MOUSEACTIVATE"),
    (0x0022, "WM_CHILDACTIVATE"),
    (0x0023, "WM_QUEUESYNC"),
    (0x0024, "WM_GETMINMAXINFO"),
    (0x0026, "WM_PAINTICON"),
    (0x0027, "WM_ICONERASEBKGND"),
    (0x0028, "WM_NEXTDLGCTL"),
    (0x002A, "WM_SPOOLERSTATUS"),
    (0x002B, "WM_DRAWITEM"),
    (0x002C, "WM_MEASUREITEM"),
    (0x002D, "WM_DELETEITEM"),
    (0x002E, "WM_VKEYTOITEM"),
    (0x002F, "WM_CHARTOITEM"),
    (0x0030, "WM_SETFONT"),
    (0x0031, "WM_GETFONT"),
    (0x0032, "WM_SETHOTKEY"),
    (0x0033, "WM_GETHOTKEY"),
    (0x0037, "WM_QUERYDRAGICON"),
    (0x0039, "WM_COMPAREITEM"),
    (0x003D, "WM_GETOBJECT"),
    (0x0041, "WM_COMPACTING"),
    (0x0044, "WM_COMMNOTIFY"),
    (0x0046, "WM_WINDOWPOSCHANGING"),
    (0x0047, "WM_WINDOWPOSCHANGED"),
    (0x0048, "WM_POWER"),
    (0x004A, "WM_COPYDATA"),
    (0x004B, "WM_CANCELJOURNAL"),
    (0x004E, "WM_NOTIFY"),
    (0x0050, "WM_INPUTLANGCHANGEREQUEST"),
    (0x0051, "WM_INPUTLANGCHANGE"),
    (0x0052, "WM_TCARD"),
    (0x0053, "WM_HELP"),
    (0x0054, "WM_USERCHANGED"),
    (0x0055, "WM_NOTIFYFORMAT"),
    (0x007B, "WM_CONTEXTMENU"),
    (0x007C, "WM_STYLECHANGING"),
    (0x007D, "WM_STYLECHANGED"),
    (0x007E, "WM_DISPLAYCHANGE"),
    (0x007F, "WM_GETICON"),
    (0x0080, "WM_SETICON"),
    (0x0081, "WM_NCCREATE"),
    (0x0082, "WM_NCDESTROY"),
    (0x0083, "WM_NCCALCSIZE"),
    (0x0084, "WM_NCHITTEST"),
    (0x0085, "WM_NCPAINT"),
    (0x0086, "WM_NCACTIVATE"),
    (0x0087, "WM_GETDLGCODE"),
    (0x0088, "WM_SYNCPAINT"),
    (0x00A0, "WM_NCMOUSEMOVE"),
    (0x00A1, "WM_NCLBUTTONDOWN"),
    (0x00A2, "WM_NCLBUTTONUP"),
    (0x00A3, "WM_NCLBUTTONDBLCLK"),
    (0x00A4, "WM_NCRBUTTONDOWN"),
    (0x00A5, "WM_NCRBUTTONUP"),
    (0x00A6, "WM_NCRBUTTONDBLCLK"),
    (0x00A7, "WM_NCMBUTTONDOWN"),
    (0x00A8, "WM_NCMBUTTONUP"),
    (0x00A9, "WM_NCMBUTTONDBLCLK"),
    (0x00AB, "WM_NCXBUTTONDOWN"),
    (0x00AC, "WM_NCXBUTTONUP"),
    (0x00AD, "WM_NCXBUTTONDBLCLK"),
    (0x00FE, "WM_INPUT_DEVICE_CHANGE"),
    (0x00FF, "WM_INPUT"),
    (0x0100, "WM_KEYDOWN"),
    (0x0100, "WM_KEYFIRST"),
    (0x0101, "WM_KEYUP"),
    (0x0102, "WM_CHAR"),
    (0x0103, "WM_DEADCHAR"),
    (0x0104, "WM_SYSKEYDOWN"),
    (0x0105, "WM_SYSKEYUP"),
    (0x0106, "WM_SYSCHAR"),
    (0x0107, "WM_SYSDEADCHAR"),
    (0x0109, "WM_UNICHAR"),
    (0x0109, "WM_KEYLAST"),
    (0x010D, "WM_IME_STARTCOMPOSITION"),
    (0x010E, "WM_IME_ENDCOMPOSITION"),
    (0x010F, "WM_IME_COMPOSITION"),
    (0x010F, "WM_IME_KEYLAST"),
    (0x0110, "WM_INITDIALOG"),
    (0x0111, "WM_COMMAND"),
    (0x0112, "WM_SYSCOMMAND"),
    (0x0113, "WM_TIMER"),
    (0x0114, "WM_HSCROLL"),
    (0x0115, "WM_VSCROLL"),
    (0x0116, "WM_INITMENU"),
    (0x0117, "WM_INITMENUPOPUP"),
    (0x0119, "WM_GESTURE"),
    (0x011A, "WM_GESTURENOTIFY"),
    (0x011F, "WM_MENUSELECT"),
    (0x0120, "WM_MENUCHAR"),
    (0x0121, "WM_ENTERIDLE"),
    (0x0122, "WM_MENURBUTTONUP"),
    (0x0123, "WM_MENUDRAG"),
    (0x0124, "WM_MENUGETOBJECT"),
    (0x0125, "WM_UNINITMENUPOPUP"),
    (0x0126, "WM_MENUCOMMAND"),
    (0x0127, "WM_CHANGEUISTATE"),
    (0x0128, "WM_UPDATEUISTATE"),
    (0x0129, "WM_QUERYUISTATE"),
    (0x0132, "WM_CTLCOLORMSGBOX"),
    (0x0133, "WM_CTLCOLOREDIT"),
    (0x0134, "WM_CTLCOLORLISTBOX"),
    (0x0135, "WM_CTLCOLORBTN"),
    (0x0136, "WM_CTLCOLORDLG"),
    (0x0137, "WM_CTLCOLORSCROLLBAR"),
    (0x0138, "WM_CTLCOLORSTATIC"),
    (0x0200, "WM_MOUSEMOVE"),
    (0x0200, "WM_MOUSEFIRST"),
    (0x0201, "WM_LBUTTONDOWN"),
    (0x0202, "WM_LBUTTONUP"),
    (0x0203, "WM_LBUTTONDBLCLK"),
    (0x0204, "WM_RBUTTONDOWN"),
    (0x0205, "WM_RBUTTONUP"),
    (0x0206, "WM_RBUTTONDBLCLK"),
    (0x0207, "WM_MBUTTONDOWN"),
    (0x0208, "WM_MBUTTONUP"),
    (0x0209, "WM_MBUTTONDBLCLK"),
    (0x020A, "WM_MOUSEWHEEL"),
    (0x020B, "WM_XBUTTONDOWN"),
    (0x020C, "WM_XBUTTONUP"),
    (0x020D, "WM_XBUTTONDBLCLK"),
    (0x020E, "WM_MOUSEHWHEEL"),
    (0x020E, "WM_MOUSELAST"),
    (0x0210, "WM_PARENTNOTIFY"),
    (0x0211, "WM_ENTERMENULOOP"),
    (0x0212, "WM_EXITMENULOOP"),
    (0x0213, "WM_NEXTMENU"),
    (0x0214, "WM_SIZING"),
    (0x0215, "WM_CAPTURECHANGED"),
    (0x0216, "WM_MOVING"),
    (0x0218, "WM_POWERBROADCAST"),
    (0x0219, "WM_DEVICECHANGE"),
    (0x0220, "WM_MDICREATE"),
    (0x0221, "WM_MDIDESTROY"),
    (0x0222, "WM_MDIACTIVATE"),
    (0x0223, "WM_MDIRESTORE"),
    (0x0224, "WM_MDINEXT"),
    (0x0225, "WM_MDIMAXIMIZE"),
    (0x0226, "WM_MDITILE"),
    (0x0227, "WM_MDICASCADE"),
    (0x0228, "WM_MDIICONARRANGE"),
    (0x0229, "WM_MDIGETACTIVE"),
    (0x0230, "WM_MDISETMENU"),
    (0x0231, "WM_ENTERSIZEMOVE"),
    (0x0232, "WM_EXITSIZEMOVE"),
    (0x0233, "WM_DROPFILES"),
    (0x0234, "WM_MDIREFRESHMENU"),
    (0x0238, "WM_POINTERDEVICECHANGE"),
    (0x0239, "WM_POINTERDEVICEINRANGE"),
    (0x023A, "WM_POINTERDEVICEOUTOFRANGE"),
    (0x0240, "WM_TOUCH"),
    (0x0241, "WM_NCPOINTERUPDATE"),
    (0x0242, "WM_NCPOINTERDOWN"),
    (0x0243, "WM_NCPOINTERUP"),
    (0x0245, "WM_POINTERUPDATE"),
    (0x0246, "WM_POINTERDOWN"),
    (0x0247, "WM_POINTERUP"),
    (0x0249, "WM_POINTERENTER"),
    (0x024A, "WM_POINTERLEAVE"),
    (0x024B, "WM_POINTERACTIVATE"),
    (0x024C, "WM_POINTERCAPTURECHANGED"),
    (0x024D, "WM_TOUCHHITTESTING"),
    (0x024E, "WM_POINTERWHEEL"),
    (0x024F, "WM_POINTERHWHEEL"),
    (0x0251, "WM_POINTERROUTEDTO"),
    (0x0252, "WM_POINTERROUTEDAWAY"),
    (0x0253, "WM_POINTERROUTEDRELEASED"),
    (0x0281, "WM_IME_SETCONTEXT"),
    (0x0282, "WM_IME_NOTIFY"),
    (0x0283, "WM_IME_CONTROL"),
    (0x0284, "WM_IME_COMPOSITIONFULL"),
    (0x0285, "WM_IME_SELECT"),
    (0x0286, "WM_IME_CHAR"),
    (0x0288, "WM_IME_REQUEST"),
    (0x0290, "WM_IME_KEYDOWN"),
    (0x0291, "WM_IME_KEYUP"),
    (0x02A0, "WM_NCMOUSEHOVER"),
    (0x02A1, "WM_MOUSEHOVER"),
    (0x02A2, "WM_NCMOUSELEAVE"),
    (0x02A3, "WM_MOUSELEAVE"),
    (0x02B1, "WM_WTSSESSION_CHANGE"),
    (0x02C0, "WM_TABLET_FIRST"),
    (0x02DF, "WM_TABLET_LAST"),
    (0x02E0, "WM_DPICHANGED"),
    (0x02E2, "WM_DPICHANGED_BEFOREPARENT"),
    (0x02E3, "WM_DPICHANGED_AFTERPARENT"),
    (0x02E4, "WM_GETDPISCALEDSIZE"),
    (0x0300, "WM_CUT"),
    (0x0301, "WM_COPY"),
    (0x0302, "WM_PASTE"),
    (0x0303, "WM_CLEAR"),
    (0x0304, "WM_UNDO"),
    (0x0305, "WM_RENDERFORMAT"),
    (0x0306, "WM_RENDERALLFORMATS"),
    (0x0307, "WM_DESTROYCLIPBOARD"),
    (0x0308, "WM_DRAWCLIPBOARD"),
    (0x0309, "WM_PAINTCLIPBOARD"),
    (0x030A, "WM_VSCROLLCLIPBOARD"),
    (0x030B, "WM_SIZECLIPBOARD"),
    (0x030C, "WM_ASKCBFORMATNAME"),
    (0x030D, "WM_CHANGECBCHAIN"),
    (0x030E, "WM_HSCROLLCLIPBOARD"),
    (0x030F, "WM_QUERYNEWPALETTE"),
    (0x0310, "WM_PALETTEISCHANGING"),
    (0x0311, "WM_PALETTECHANGED"),
    (0x0312, "WM_HOTKEY"),
    (0x0317, "WM_PRINT"),
    (0x0318, "WM_PRINTCLIENT"),
    (0x0319, "WM_APPCOMMAND"),
    (0x031A, "WM_THEMECHANGED"),
    (0x031D, "WM_CLIPBOARDUPDATE"),
    (0x031E, "WM_DWMCOMPOSITIONCHANGED"),
    (0x031F, "WM_DWMNCRENDERINGCHANGED"),
    (0x0320, "WM_DWMCOLORIZATIONCOLORCHANGED"),
    (0x0321, "WM_DWMWINDOWMAXIMIZEDCHANGE"),
    (0x0323, "WM_DWMSENDICONICTHUMBNAIL"),
    (0x0326, "WM_DWMSENDICONICLIVEPREVIEWBITMAP"),
    (0x033F, "WM_GETTITLEBARINFOEX"),
    (0x0345, "WM_TOOLTIPDISMISS"),
    (0x0358, "WM_HANDHELDFIRST"),
    (0x035F, "WM_HANDHELDLAST"),
    (0x0360, "WM_AFXFIRST"),
    (0x037F, "WM_AFXLAST"),
    (0x0380, "WM_PENWINFIRST"),
    (0x038F, "WM_PENWINLAST"),
    (0x0400, "WM_USER"),
    (0x8000, "WM_APP"),
];
