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
