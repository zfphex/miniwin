use std::cell::RefCell;

use Event;

thread_local! {
    static EVENT_QUEUE: RefCell<Vec<Event>> = const { RefCell::new(Vec::new()) };
}

pub fn push_event(event: Event) {
    EVENT_QUEUE.with(|q| q.borrow_mut().push(event));
}

pub fn pop_all_events() -> Vec<Event> {
    EVENT_QUEUE.with(|q| q.borrow_mut().split_off(0))
}
