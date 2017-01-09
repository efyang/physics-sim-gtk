use gtk::prelude::*;
use gtk::{self, Window, WindowType, DrawingArea, Orientation};
use sharedstate::SharedState;
use std::sync::mpsc::TryRecvError;
use updater::{UpdaterCommand, Updater};
use iteration_result::IterationResult;
use coloruniverse::ColorUniverse;
use input::MOUSE_MOVEMENT_BORDER_WIDTH;

mod state;
mod data;
mod draw_handler;
mod key_handler;
mod mouse_handler;

use self::data::UiData;
use self::draw_handler::*;
use self::key_handler::*;
use self::mouse_handler::*;
use self::state::*;

pub struct Ui {
    data: SharedState<UiData>,
    draw_area: DrawingArea,
}

impl Ui {
    pub fn initialize() -> Ui {
        let (mut updater, universe_recv, update_command_send) =
            Updater::new(ColorUniverse::default());

        let window = default_window();
        let mainsplit = gtk::Box::new(Orientation::Vertical, 10);
        let draw_area = DrawingArea::new();
        let input_interface = gtk::Box::new(Orientation::Vertical, 10);
        let time_scale = gtk::Scale::new_with_range(Orientation::Horizontal, 1., 100_000., 1000.);
        let accuracy_scale = gtk::Scale::new_with_range(Orientation::Horizontal, 1., 100_000., 1000.);
        input_interface.add(&time_scale);
        input_interface.add(&accuracy_scale);
        mainsplit.pack_start(&draw_area, true, true, 0);
        mainsplit.pack_end(&input_interface, false, true, 0);
        window.add(&mainsplit);
        window.show_all();

        let data = UiData::new(universe_recv, update_command_send);

        let this = Ui {
            data: SharedState::new(data),
            draw_area: draw_area,
        };

        this.setup_draw_callbacks();
        this.setup_mouse_callbacks();
        this.setup_key_callbacks(&window);
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
        let data = self.data.clone();
        self.draw_area.set_size_request(800, 800);
        self.draw_area.connect_draw(move |_, ctxt| {
            draw_handler(&data, ctxt);
            Inhibit(false)
        });
    }

    fn setup_key_callbacks(&self, window: &Window) {
        self.draw_area.set_can_focus(true);

        {
            let data = self.data.clone();
            window.connect_key_press_event(move |_, key| {
                key_press_handler(&data, key);
                Inhibit(false)
            });
        }

        {
            let data = self.data.clone();
            window.connect_key_release_event(move |_, key| {
                key_release_handler(&data, key);
                Inhibit(false)
            });
        }
    }

    fn setup_mouse_callbacks(&self) {
        self.draw_area.add_events(::gdk_sys::GDK_BUTTON_PRESS_MASK.bits() as i32);
        self.draw_area.add_events(::gdk_sys::GDK_BUTTON_RELEASE_MASK.bits() as i32);
        self.draw_area.add_events(::gdk_sys::GDK_SCROLL_MASK.bits() as i32);
        self.draw_area.add_events(::gdk_sys::GDK_POINTER_MOTION_MASK.bits() as i32);

        {
            let data = self.data.clone();
            self.draw_area.connect_button_press_event(move |_, button| {
                mouse_press_handler(&data, button);
                Inhibit(false)
            });
        }

        {
            let data = self.data.clone();
            self.draw_area.connect_button_release_event(move |_, button| {
                mouse_release_handler(&data, button);
                Inhibit(false)
            });
        }

        {
            let data = self.data.clone();
            self.draw_area.connect_scroll_event(move |_, scroll| {
                mouse_scroll_handler(&data, scroll);
                Inhibit(false)
            });
        }

        {
            let data = self.data.clone();
            self.draw_area.connect_motion_notify_event(move |_, motion| {
                mouse_motion_handler(&data, motion);
                Inhibit(false)
            });
        }
    }

    fn setup_window_callbacks(&self, window: &Window) {
        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        let data = self.data.clone();
        let draw_area = self.draw_area.clone();
        window.connect_check_resize(move |_| {
            let ref mut data = *data.get_state_mut();
            // set the new size
            let allocation_size = draw_area.get_allocation();
            let (x_size, y_size) = (allocation_size.width, allocation_size.height);
            data.draw_info.set_size(x_size as f64, y_size as f64);
            draw_area.queue_draw();
        });
    }

    // fn setup_button_callbacks(buttons: ) {

    // }

    fn handle_input_iteration(&mut self) {
        let ref mut data = *self.data.get_state_mut();

        // handle the mouse position (if its within borders then move view)
        let (x_size, y_size) = data.draw_info.get_size();
        if data.allow_mouse_movement &&
           data.input_info.mouse_within_any_side_border(x_size, y_size) {
            let max_movement = 20.;
            let (mut x_trans, mut y_trans) = (0., 0.);
            if let Some(distance) = data.input_info.mouse_top_move_border(y_size) {
                y_trans = max_movement * (1. - distance / MOUSE_MOVEMENT_BORDER_WIDTH);
            } else if let Some(distance) = data.input_info.mouse_bottom_move_border(y_size) {
                y_trans = -max_movement * (1. - distance / MOUSE_MOVEMENT_BORDER_WIDTH);
            }
            if let Some(distance) = data.input_info.mouse_left_move_border(x_size) {
                x_trans = max_movement * (1. - distance / MOUSE_MOVEMENT_BORDER_WIDTH);
            } else if let Some(distance) = data.input_info.mouse_right_move_border(x_size) {
                x_trans = -max_movement * (1. - distance / MOUSE_MOVEMENT_BORDER_WIDTH);
            }
            data.draw_info.translate(x_trans, y_trans);
        } else {
            // handle the arrow keys
            if data.input_info.up {
                data.draw_info.translate(0., 7.5);
            } else if data.input_info.down {
                data.draw_info.translate(0., -7.5);
            }
            if data.input_info.left {
                data.draw_info.translate(7.5, 0.);
            } else if data.input_info.right {
                data.draw_info.translate(-7.5, 0.);
            }
        }
    }

    pub fn iterate(&mut self) -> IterationResult {
        self.handle_input_iteration();

        let ref mut data = *self.data.get_state_mut();
        if data.fps_info.should_redraw() {
            self.draw_area.queue_draw();
        }

        // check the updater output
        match data.state {
            UiState::Paused | UiState::Edit(_) => {
                // set the current universe
                data.update_command_send
                    .send(UpdaterCommand::SetUniverse(data.universe.clone()))
                    .unwrap();
                // clear the receiver
                let mut clear = false;
                while !clear {
                    match data.universe_recv.try_recv() {
                        Ok(_) => {}
                        Err(TryRecvError::Empty) => clear = true,
                        Err(e) => {
                            // should never happen
                            return IterationResult::Error(format!("{}", e));
                        }
                    }
                }
            }
            _ => {
                match data.universe_recv.try_recv() {
                    Ok(new_universe) => {
                        data.universe = new_universe;
                        // tell the updater it has consumed a state
                        data.update_command_send.send(UpdaterCommand::UniverseConsumed).unwrap();
                    }
                    Err(TryRecvError::Empty) => {}
                    Err(e) => {
                        // should never happen
                        return IterationResult::Error(format!("{}", e));
                    }
                }
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
