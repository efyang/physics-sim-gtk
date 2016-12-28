pub struct InputInfo {
    pub shift: bool,
    pub ctrl: bool,
    pub backspace: BackSpaceState,
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub mouse_x: f64,
    pub mouse_y: f64,
}

#[derive(Debug)]
pub enum BackSpaceState {
    NotPressed,
    PressedOnce,
    PressedTwice, // this is where it should reset the zoom level and be set back to NotPressed
}

impl BackSpaceState {
    pub fn next_state(&mut self) {
        match *self {
            BackSpaceState::NotPressed => *self = BackSpaceState::PressedOnce,
            BackSpaceState::PressedOnce => *self = BackSpaceState::PressedTwice,
            BackSpaceState::PressedTwice => *self = BackSpaceState::NotPressed,
        }
    }

    pub fn should_reset(&self) -> bool {
        match *self {
            BackSpaceState::PressedTwice => true,
            _ => false,
        }
    }
}

impl Default for InputInfo {
    fn default() -> InputInfo {
        InputInfo {
            shift: false,
            ctrl: false,
            backspace: BackSpaceState::NotPressed,
            up: false,
            down: false,
            left: false,
            right: false,
            mouse_x: 20.,
            mouse_y: 20.,
        }
    }
}

pub const MOUSE_MOVEMENT_BORDER_WIDTH: f64 = 25.;

impl InputInfo {
    pub fn mouse_within_any_side_border(&self, size_x: f64, size_y: f64) -> bool {
        self.mouse_left_move_border(size_x) || self.mouse_right_move_border(size_x) ||
        self.mouse_top_move_border(size_y) || self.mouse_bottom_move_border(size_y)
    }

    pub fn mouse_left_move_border(&self, size_x: f64) -> bool {
        (self.mouse_x >= 0. && self.mouse_x <= MOUSE_MOVEMENT_BORDER_WIDTH)
    }

    pub fn mouse_right_move_border(&self, size_x: f64) -> bool {
        (self.mouse_x >= size_x - MOUSE_MOVEMENT_BORDER_WIDTH && self.mouse_x <= size_x)
    }

    pub fn mouse_top_move_border(&self, size_y: f64) -> bool {
        (self.mouse_y >= 0. && self.mouse_y <= MOUSE_MOVEMENT_BORDER_WIDTH)
    }

    pub fn mouse_bottom_move_border(&self, size_y: f64) -> bool {
        (self.mouse_y >= size_y - MOUSE_MOVEMENT_BORDER_WIDTH && self.mouse_y <= size_y)
    }
}
