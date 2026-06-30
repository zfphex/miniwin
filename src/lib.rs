#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "windows")]
pub use windows::*;

#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "macos")]
pub use macos::*;

pub trait Window {
    fn draw<F>(&mut self, render: F)
    where
        F: FnMut(&mut Self);
    fn event(&mut self) -> Option<Event>;
    fn update_buffer(&mut self, pixels: &[u32], width: usize, height: usize);
    fn scale_factor(&self) -> f64;
    fn content_size(&self) -> (usize, usize);
    fn wait_for_vsync(&self);
    fn set_cursor_visible(&self, visible: bool);
    fn set_cursor_grab(&self, grab: bool);
    fn set_cursor_icon(&self, icon: CursorIcon);
    fn get_clipboard_text(&self) -> Option<String>;
    fn set_clipboard_text(&self, text: &str);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowStyle {
    Standard,
    Borderless,
    Transparent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FullscreenMode {
    None,
    Workspace,
    MonitorFit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorIcon {
    Arrow,
    IBeam,
    PointingHand,
    ClosedHand,
    OpenHand,
    Crosshair,
    ResizeLeftRight,
    ResizeUpDown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u16),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Modifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub logo: bool, // Command/Windows key
}

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    CloseRequested,
    Resized {
        width: f64,
        height: f64,
        physical_width: usize,
        physical_height: usize,
    },
    KeyDown {
        keycode: u16,
        modifiers: Modifiers,
    },
    KeyUp {
        keycode: u16,
        modifiers: Modifiers,
    },
    MouseDown {
        button: MouseButton,
        x: f64,
        y: f64,
        modifiers: Modifiers,
    },
    MouseUp {
        button: MouseButton,
        x: f64,
        y: f64,
        modifiers: Modifiers,
    },
    MouseMoved {
        x: f64,
        y: f64,
        modifiers: Modifiers,
    },
    MouseDragged {
        button: MouseButton,
        x: f64,
        y: f64,
        modifiers: Modifiers,
    },
    Scroll {
        delta_x: f64,
        delta_y: f64,
        modifiers: Modifiers,
    },
    ReceivedCharacter(char),
    DroppedFiles(Vec<std::path::PathBuf>),
    Quit,
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl std::ops::AddAssign for Rect {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.width += rhs.width;
        self.height += rhs.height;
    }
}

impl Rect {
    pub const fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
    pub const fn x(mut self, x: usize) -> Self {
        self.x = x;
        self
    }
    pub const fn y(mut self, y: usize) -> Self {
        self.y = y;
        self
    }
    pub const fn width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }
    pub const fn height(mut self, height: usize) -> Self {
        self.height = height;
        self
    }
    pub const fn right(&self) -> usize {
        self.x + self.width
    }
    pub const fn bottom(&self) -> usize {
        self.y + self.height
    }
    pub const fn intersects(&self, other: Rect) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }
    pub const fn contains(&self, x: usize, y: usize) -> bool {
        x >= self.x && x < self.x + self.width && y >= self.y && y < self.y + self.height
    }
    pub fn intersection(&self, other: Rect) -> Rect {
        let x1 = self.x.max(other.x);
        let y1 = self.y.max(other.y);
        let x2 = (self.x + self.width).min(other.x + other.width);
        let y2 = (self.y + self.height).min(other.y + other.height);
        if x2 > x1 && y2 > y1 {
            Rect {
                x: x1,
                y: y1,
                width: (x2 - x1),
                height: (y2 - y1),
            }
        } else {
            Rect::new(0, 0, 0, 0)
        }
    }
    pub fn split_h(&self, left_width: usize) -> (Rect, Rect) {
        let total_w = (self.x + self.width).saturating_sub(self.x);
        let total_h = (self.y + self.height).saturating_sub(self.y);
        let left_w = left_width.min(total_w);
        let right_w = total_w.saturating_sub(left_w);
        let left_rect = Rect::new(self.x, self.y, left_w, total_h);
        let right_rect = Rect::new(self.x + left_w, self.y, right_w, total_h);
        (left_rect, right_rect)
    }
    pub fn split_v(&self, top_height: usize) -> (Rect, Rect) {
        let total_w = (self.x + self.width).saturating_sub(self.x);
        let total_h = (self.y + self.height).saturating_sub(self.y);
        let top_h = top_height.min(total_h);
        let bottom_h = total_h.saturating_sub(top_h);
        let top_rect = Rect::new(self.x, self.y, total_w, top_h);
        let bottom_rect = Rect::new(self.x, self.y + top_h, total_w, bottom_h);
        (top_rect, bottom_rect)
    }
    pub const fn inner(&self, w: usize, h: usize) -> Rect {
        Rect {
            x: self.x + w,
            y: self.y + h,
            width: self.width.saturating_sub(2 * w),
            height: self.height.saturating_sub(2 * h),
        }
    }
}
