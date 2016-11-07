use gtk::prelude::*;
use gtk::{self, Window, WindowType, DrawingArea, Orientation};
use cairo::prelude::*;
use physics_sim::Universe;
use super::uistate::UiState;
use sharedstate::SharedState;
use super::fpsinfo::FpsInfo;

pub enum IterationResult {
    Ok,
    Finished,
    Error(String),
}

pub struct Ui {
    fpsinfo: SharedState<FpsInfo>,
    state: SharedState<UiState>,
    drawarea: DrawingArea,
    universe: SharedState<Universe>,
}

impl Ui {
    pub fn initialize() -> Ui {
        initialize_gtk();

        let window = default_window();
        let mainsplit = gtk::Box::new(Orientation::Vertical, 10);
        let drawarea = DrawingArea::new();
        let input_interface = gtk::Box::new(Orientation::Horizontal, 10);
        mainsplit.add(&drawarea);
        mainsplit.add(&input_interface);
        window.add(&mainsplit);
        window.show_all();

        Ui {
            fpsinfo: SharedState::new(FpsInfo::default()),
            state: SharedState::new(UiState::default()),
            drawarea: drawarea,
            universe: SharedState::new(Universe::default()),
        }
    }

    pub fn setup_callbacks(&self) {
        let mut fpsinfo = self.fpsinfo.clone();
        self.drawarea.connect_draw(move |drawarea, ctxt| {
            // draw everything



            // get ready for next fps update
            fpsinfo.get_state_mut().update_time();
            unimplemented!();
            Inhibit(false)
        });
    }

    pub fn iterate(&mut self) -> IterationResult {
        if self.fpsinfo.get_state().should_redraw() {
            self.drawarea.queue_draw();
        }

        gtk::main_iteration();
        unimplemented!();
        IterationResult::Ok
    }
}

fn initialize_gtk() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
}

fn default_window() -> Window {
    let window = Window::new(WindowType::Toplevel);
    window.set_title("Physics Simulator");
    window.set_default_size(800, 800);
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    window
}
