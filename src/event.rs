use std::cell::RefCell;

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
    pub logo: bool, // Command key
}

impl Modifiers {
    pub fn parse(flags: usize) -> Self {
        Self {
            shift: (flags & (1 << 17)) != 0,
            ctrl: (flags & (1 << 18)) != 0,
            alt: (flags & (1 << 19)) != 0,
            logo: (flags & (1 << 20)) != 0,
        }
    }
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
    KeyDown { keycode: u16, modifiers: Modifiers },
    KeyUp { keycode: u16, modifiers: Modifiers },
    MouseDown { button: MouseButton, x: f64, y: f64, modifiers: Modifiers },
    MouseUp { button: MouseButton, x: f64, y: f64, modifiers: Modifiers },
    MouseMoved { x: f64, y: f64, modifiers: Modifiers },
    MouseDragged { button: MouseButton, x: f64, y: f64, modifiers: Modifiers },
    Scroll { delta_x: f64, delta_y: f64, modifiers: Modifiers },
}

thread_local! {
    static EVENT_QUEUE: RefCell<Vec<Event>> = const { RefCell::new(Vec::new()) };
}

pub fn push_event(event: Event) {
    EVENT_QUEUE.with(|q| q.borrow_mut().push(event));
}

pub fn pop_all_events() -> Vec<Event> {
    EVENT_QUEUE.with(|q| q.borrow_mut().split_off(0))
}
