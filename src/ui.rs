use gtk::prelude::*;
use gtk::Window;
use physics_sim::Universe;

use super::uistate::UiState;

pub enum IterationResult {
    Ok,
    Finished,
    Error(String),
}

pub struct Ui {
    state: UiState,
    window: Window,
    universe: Universe,
}

impl Ui {
    pub fn initialize() -> Ui {
        unimplemented!()
    }

    pub fn iterate(&mut self) -> IterationResult {
        unimplemented!()
    }
}
