use crate::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct MouseButtonState {
    pub pressed: bool,
    pub released: bool,
    pub initial_position: Option<Rect>,
    pub release_position: Option<Rect>,
}

impl MouseButtonState {
    pub const fn new() -> Self {
        Self {
            pressed: false,
            released: false,
            initial_position: None,
            release_position: None,
        }
    }

    pub fn is_pressed(&mut self) -> bool {
        if self.pressed {
            self.pressed = false;
            true
        } else {
            false
        }
    }

    pub fn is_released(&mut self) -> bool {
        if self.released {
            self.released = false;
            true
        } else {
            false
        }
    }

    pub fn clicked(&mut self, area: Rect) -> bool {
        if !self.released {
            return false;
        }

        self.released = false;

        let Some(initial) = self.initial_position else {
            return false;
        };

        let Some(release) = self.release_position else {
            return false;
        };

        initial.intersects(area) && release.intersects(area)
    }

    pub fn pressed(&mut self, pos: Rect) {
        self.pressed = true;
        self.released = false;
        self.initial_position = Some(pos);
    }

    pub fn released(&mut self, pos: Rect) {
        self.pressed = false;
        self.released = true;
        self.release_position = Some(pos);
    }
}

#[derive(Debug)]
pub struct InputState {
    pub current_keys: [bool; 256],
    pub previous_keys: [bool; 256],
    pub mouse_x: i32,
    pub mouse_y: i32,
    pub scroll_delta: f32,
}

impl InputState {
    pub const fn new() -> Self {
        Self {
            current_keys: [false; 256],
            previous_keys: [false; 256],
            mouse_x: 0,
            mouse_y: 0,
            scroll_delta: 0.0,
        }
    }

    pub fn is_down(&self, key: Key) -> bool {
        let vk_code = key.vk_code();
        self.current_keys[vk_code]
    }

    pub fn is_up(&self, key: Key) -> bool {
        let vk_code = key.vk_code();
        !self.current_keys[vk_code]
    }

    pub fn pressed(&self, key: Key) -> bool {
        let vk_code = key.vk_code();
        self.current_keys[vk_code] && !self.previous_keys[vk_code]
    }

    pub fn released(&self, key: Key) -> bool {
        let vk_code = key.vk_code();
        !self.current_keys[vk_code] && self.previous_keys[vk_code]
    }

    pub fn advance_frame(&mut self) {
        self.previous_keys.copy_from_slice(&self.current_keys);
        self.scroll_delta = 0.0;
    }

    pub fn set_key_down(&mut self, vk_code: usize) {
        if vk_code < 256 {
            self.current_keys[vk_code] = true;
        }
    }

    pub fn set_key_up(&mut self, vk_code: usize) {
        if vk_code < 256 {
            self.current_keys[vk_code] = false;
        }
    }
}
