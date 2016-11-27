use super::editstate::EditState;

pub enum UiState {
    Normal,
    Edit(EditState),
    Stop,
}

impl Default for UiState {
    fn default() -> UiState {
        UiState::Normal
    }
}

// INCOMPLETE
