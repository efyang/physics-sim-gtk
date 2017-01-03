use physics_sim::Point;

pub enum EditState {
    Mouse(MouseEditState),
    Input,
}

impl Default for EditState {
    fn default() -> EditState {
        EditState::Mouse(MouseEditState::SetPoint)
    }
}

pub enum MouseEditState {
    SetPoint,
    SetMass(Point),
    SetVelocity(f64, Point),
}

impl Default for MouseEditState {
    fn default() -> MouseEditState {
        MouseEditState::SetPoint
    }
}

// INCOMPLETE
