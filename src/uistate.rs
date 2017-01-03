use editstate::EditState;

pub enum UiState {
    Normal,
    Edit(EditState),
    Paused,
}

impl Default for UiState {
    fn default() -> UiState {
        UiState::Normal
    }
}

// INCOMPLETE
