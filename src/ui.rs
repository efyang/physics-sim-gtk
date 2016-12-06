use gtk::prelude::*;
use gtk::{self, Window, WindowType, DrawingArea, Orientation};
use cairo::prelude::*;
use coloruniverse::ColorUniverse;
use uistate::UiState;
use sharedstate::SharedState;
use fpsinfo::FpsInfo;
use drawinfo::DrawInfo;
use drawobject::DrawAll;

pub enum IterationResult {
    Ok,
    Finished,
    Error(String),
}

pub struct Ui {
    fpsinfo: SharedState<FpsInfo>,
    state: SharedState<UiState>,
    drawarea: SharedState<DrawingArea>,
    universe: SharedState<ColorUniverse>,
    drawinfo: SharedState<DrawInfo>,
    testvar: SharedState<f64>,
}

impl Ui {
    pub fn initialize() -> Ui {
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
            universe: SharedState::new(ColorUniverse::default()),
            drawinfo: SharedState::new(DrawInfo::default()),
            testvar: SharedState::new(0f64),
        };
        this.setup_draw_callbacks();
        this.setup_window_callbacks(&window);
        this
    }

    fn setup_draw_callbacks(&self) {
        let fpsinfo = self.fpsinfo.clone();
        let universe = self.universe.clone();
        let drawarea = self.drawarea.get_state();
        let drawinfo = self.drawinfo.clone();
        let testvar = self.testvar.clone();
        drawarea.set_size_request(800, 800);
        drawarea.connect_draw(move |drawarea, ctxt| {
            // apply the drawing info
            drawinfo.get_state().apply(ctxt);
            // draw everything
            universe.get_state().draw_all(ctxt);

            // NOTE: placeholder
            ctxt.set_operator(::cairo::Operator::Source);
            ctxt.set_source_rgb(0.0, 0.5, 0.0);
            ctxt.paint();

            ctxt.set_source_rgb(0.5, 0.5, 0.5);
            ctxt.rectangle(*testvar.get_state(), 400., 50., 50.);
            ctxt.fill();
            *testvar.get_state_mut() += 1.;

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

    //fn setup_button_callbacks(buttons: ) {

    //}

    pub fn iterate(&mut self) -> IterationResult {
        if self.fpsinfo.get_state().should_redraw() {
            println!("next frame");
            self.drawarea.get_state().queue_draw();
        }

        // check the updater output ring buffer
        
       
        IterationResult::Ok
    }
}


fn default_window() -> Window {
    let window = Window::new(WindowType::Toplevel);
    window.set_title("Physics Simulator");
    window.set_default_size(800, 800);
    window
}
