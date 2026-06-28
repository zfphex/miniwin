use std::cell::RefCell;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u16),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    CloseRequested,
    Resized { width: f64, height: f64 },
    KeyDown { keycode: u16, modifiers: u32 },
    KeyUp { keycode: u16, modifiers: u32 },
    MouseDown { button: MouseButton, x: f64, y: f64 },
    MouseUp { button: MouseButton, x: f64, y: f64 },
    MouseMoved { x: f64, y: f64 },
    MouseDragged { button: MouseButton, x: f64, y: f64 },
    Scroll { delta_x: f64, delta_y: f64 },
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
