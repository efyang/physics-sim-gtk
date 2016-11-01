use gtk::prelude::*;
use gtk::{self, Window, WindowType, DrawingArea, Orientation};
use physics_sim::Universe;
use super::uistate::UiState;
use sharedstate::SharedState;

pub enum IterationResult {
    Ok,
    Finished,
    Error(String),
}

pub struct Ui {
    state: SharedState<UiState>,
    drawarea: DrawingArea,
    universe: SharedState<Universe>,
}

impl Ui {
    pub fn initialize() -> Ui {
        let window = Window::new(WindowType::Toplevel);
        window.set_title("Physics Simulator");
        window.set_default_size(800, 800);
        let mainsplit = gtk::Box::new(Orientation::Vertical, 10);
        let drawarea = DrawingArea::new();
        let input_interface = gtk::Box::new(Orientation::Horizontal, 10);
        mainsplit.add(&drawarea);
        mainsplit.add(&input_interface);
        window.add(&mainsplit);

        Ui {
            state: SharedState::new(UiState::default()),
            drawarea: drawarea,
            universe: SharedState::new(Universe::default()),
        }
    }

    pub fn iterate(&mut self) -> IterationResult {

        unimplemented!()
    }
}
