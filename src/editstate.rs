pub enum EditState {
    Mouse,
    Input,
}

impl Default for EditState {
    fn default() -> EditState {
        EditState::Mouse
    }
}

pub enum MouseEditState {
    SetPoint,
    SetMass,
    SetVelocity,
}

impl Default for MouseEditState {
    fn default() -> MouseEditState {
        MouseEditState::SetPoint
    }
}
