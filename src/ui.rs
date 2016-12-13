use gtk::prelude::*;
use gtk::{self, Window, WindowType, DrawingArea, Orientation};
use cairo::prelude::*;
use coloruniverse::ColorUniverse;
use uistate::UiState;
use sharedstate::SharedState;
use fpsinfo::FpsInfo;
use drawinfo::DrawInfo;
use drawobject::DrawAll;
use std::sync::mpsc::{Sender, Receiver};
use updater::{UpdateSettings, UpdaterCommand, Updater};
use iteration_result::IterationResult;

pub struct Ui {
    fpsinfo: SharedState<FpsInfo>,
    state: SharedState<UiState>,
    drawarea: SharedState<DrawingArea>,
    universe: SharedState<ColorUniverse>,
    drawinfo: SharedState<DrawInfo>,
    universe_recv: SharedState<Receiver<ColorUniverse>>,
    update_settings: SharedState<UpdateSettings>,
    update_command_send: SharedState<Sender<UpdaterCommand>>,
}

impl Ui {
    pub fn initialize() -> Ui {
        let setup_tmp_universe = {
            let mut universe = ColorUniverse::default();

            universe.add_object(::physics_sim::Object::new(5_000_000.,
                                                           ::physics_sim::Vector::new(0.001111, (-180f64).to_radians()),
                                                           ::physics_sim::Point::new(0., 500.)),
                                                           ::color::ObjectColor::FromMass);
            universe.add_object(::physics_sim::Object::new(5_000_000.,
                                                           ::physics_sim::Vector::new(0.001111, (-60f64).to_radians()),
                                                           ::physics_sim::Point::new(-353.5539, -353.5539)),
                                                           ::color::ObjectColor::FromMass);
            universe.add_object(::physics_sim::Object::new(5_000_000.,
                                                           ::physics_sim::Vector::new(0.001111, (60f64).to_radians()),
                                                           ::physics_sim::Point::new(353.5539, -353.5539)),
                                                           ::color::ObjectColor::FromMass);
            universe
        };

        // let (mut updater, universe_recv, update_command_send) =
        // Updater::new(ColorUniverse::default());
        let (mut updater, universe_recv, update_command_send) =
            Updater::new(setup_tmp_universe.clone());

        let window = default_window();
        let mainsplit = gtk::Box::new(Orientation::Vertical, 10);
        let drawarea = DrawingArea::new();
        let input_interface = gtk::Box::new(Orientation::Horizontal, 10);
        mainsplit.add(&drawarea);
        mainsplit.add(&input_interface);
        window.add(&mainsplit);
        window.show_all();

        let this = Ui {
            fpsinfo: SharedState::new(FpsInfo::default()),
            state: SharedState::new(UiState::default()),
            drawarea: SharedState::new(drawarea),
            // universe: SharedState::new(ColorUniverse::default()),
            universe: SharedState::new(setup_tmp_universe),
            drawinfo: SharedState::new(DrawInfo::default()),
            universe_recv: SharedState::new(universe_recv),
            update_settings: SharedState::new(UpdateSettings::default()),
            update_command_send: SharedState::new(update_command_send),
        };
        this.setup_draw_callbacks();
        this.setup_window_callbacks(&window);
        ::std::thread::spawn(move || {
            loop {
                match updater.iterate() {
                    IterationResult::Error(e) => {
                        println!("{}", e);
                        break;
                    }
                    IterationResult::Finished => break,
                    IterationResult::Ok => {}
                }
            }
        });
        this
    }

    fn setup_draw_callbacks(&self) {
        let fpsinfo = self.fpsinfo.clone();
        let universe = self.universe.clone();
        let drawarea = self.drawarea.get_state();
        let drawinfo = self.drawinfo.clone();
        drawarea.set_size_request(800, 800);
        drawarea.connect_draw(move |drawarea, ctxt| {
            // apply the drawing info
            drawinfo.get_state().apply(ctxt);
            // draw everything
            universe.get_state().draw_all(ctxt);

            // NOTE: placeholder
            // ctxt.set_operator(::cairo::Operator::Source);
            // ctxt.set_source_rgb(0.0, 0.5, 0.0);
            // ctxt.paint();

            // get ready for next fps update
            fpsinfo.get_state_mut().update_time();
            Inhibit(false)
        });
    }

    fn setup_window_callbacks(&self, window: &Window) {
        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        let drawinfo = self.drawinfo.clone();
        let drawarea = self.drawarea.clone();
        window.connect_check_resize(move |_| {
            // set the new size
            let drawarea = drawarea.get_state();
            let (x_size, y_size) = drawarea.get_size_request();
            drawinfo.get_state_mut().set_size(x_size as f64, y_size as f64);
            drawarea.queue_draw();
        });
    }

    // fn setup_button_callbacks(buttons: ) {

    // }

    pub fn iterate(&mut self) -> IterationResult {
        if self.fpsinfo.get_state().should_redraw() {
            self.drawarea.get_state().queue_draw();
        }

        // check the updater output
        match self.universe_recv.get_state().recv() {
            Ok(new_universe) => *self.universe.get_state_mut() = new_universe,
            Err(e) => {
                // should never happen
                return IterationResult::Error(format!("{}", e));
            }
        }

        IterationResult::Ok
    }
}


fn default_window() -> Window {
    let window = Window::new(WindowType::Toplevel);
    window.set_title("Physics Simulator");
    window.set_default_size(800, 800);
    window
}
