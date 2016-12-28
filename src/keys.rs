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
