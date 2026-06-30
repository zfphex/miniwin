use crate::common::*;
use crate::windows::*;


#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Key {
    Char(char),
    Function(u8),
    Enter,
    Space,
    Backspace,
    Escape,
    Control,
    Shift,
    Alt,
    Tab,
    Up,
    Down,
    Left,
    Right,
    LeftWindows,
    RightWindows,
    Menu,
    ScrollLock,
    PauseBreak,
    Insert,
    Home,
    Delete,
    End,
    PageUp,
    PageDown,
    Unknown(u16),
}

impl Key {
    // pub const fn into(self, modifiers: Modifiers) -> Option<Event> {
    //     Some(Event::Input(self, modifiers))
    // }
    pub const fn vk_code(&self) -> usize {
        match *self {
            Key::Enter => 0x0D,        // VK_RETURN
            Key::Space => 0x20,        // VK_SPACE
            Key::Backspace => 0x08,    // VK_BACK
            Key::Escape => 0x1B,       // VK_ESCAPE
            Key::Control => 0x11,      // VK_CONTROL
            Key::Shift => 0x10,        // VK_SHIFT
            Key::Alt => 0x12,          // VK_MENU
            Key::Tab => 0x09,          // VK_TAB
            Key::Up => 0x26,           // VK_UP
            Key::Down => 0x28,         // VK_DOWN
            Key::Left => 0x25,         // VK_LEFT
            Key::Right => 0x27,        // VK_RIGHT
            Key::LeftWindows => 0x5B,  // VK_LWIN
            Key::RightWindows => 0x5C, // VK_RWIN
            Key::Menu => 0x5D,         // VK_APPS
            Key::ScrollLock => 0x91,   // VK_SCROLL
            Key::PauseBreak => 0x13,   // VK_PAUSE
            Key::Insert => 0x2D,       // VK_INSERT
            Key::Home => 0x24,         // VK_HOME
            Key::Delete => 0x2E,       // VK_DELETE
            Key::End => 0x23,          // VK_END
            Key::PageUp => 0x21,       // VK_PRIOR
            Key::PageDown => 0x22,     // VK_NEXT
            Key::Char(c) => {
                let upper = c.to_ascii_uppercase();
                match upper {
                    'A'..='Z' | '0'..='9' => upper as usize,
                    '=' | '+' => 0xBB,  // VK_OEM_PLUS
                    '-' | '_' => 0xBD,  // VK_OEM_MINUS
                    ';' | ':' => 0xBA,  // VK_OEM_1
                    '/' | '?' => 0xBF,  // VK_OEM_2
                    '`' | '~' => 0xC0,  // VK_OEM_3
                    '[' | '{' => 0xDB,  // VK_OEM_4
                    '\\' | '|' => 0xDC, // VK_OEM_5
                    ']' | '}' => 0xDD,  // VK_OEM_6
                    '\'' | '"' => 0xDE, // VK_OEM_7
                    ',' | '<' => 0xBC,  // VK_OEM_COMMA
                    '.' | '>' => 0xBE,  // VK_OEM_PERIOD
                    _ => 0,             // Unmappable or unsupported char
                }
            }
            Key::Function(n) if n >= 1 && n <= 24 => (0x6F + n) as usize,
            Key::Function(_) => 0,
            Key::Unknown(vk) => vk as usize,
        }
    }
}

pub fn modifiers() -> Modifiers {
    unsafe {
        Modifiers {
            ctrl: GetKeyState(VK_CONTROL) & 0x80 > 0,
            shift: GetKeyState(VK_SHIFT) & 0x80 > 0,
            alt: GetKeyState(VK_MENU) & 0x80 > 0,
            logo: GetKeyState(VK_LWIN) & 0x80 > 0 || GetKeyState(VK_RWIN) & 0x80 > 0,
        }
    }
}
