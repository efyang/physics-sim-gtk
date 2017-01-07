use gtk::prelude::*;
use gtk::{self, Window, WindowType, DrawingArea, Orientation};
use sharedstate::SharedState;
use draw::*;
use std::sync::mpsc::TryRecvError;
use updater::{UpdaterCommand, Updater};
use iteration_result::IterationResult;
use gdk::enums::key;
use editstate::{EditState, MouseEditState};
use physics_sim::{Object, Point, Vector};
use color::{mass_to_color, ObjectColor};
use uidata::UiData;
use coloruniverse::ColorUniverse;
use uistate::UiState;
use input::MOUSE_MOVEMENT_BORDER_WIDTH;
use fpsinfo::DEFAULT_FPS;

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
        let input_interface = gtk::Box::new(Orientation::Horizontal, 10);
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
            let ref mut data = *data.get_state_mut();
            // apply the drawing info
            data.draw_info.apply(ctxt);
            // draw everything
            data.universe.draw_all(ctxt);
            // draw the mode;

            // draw the edit information(if its in edit mode)
            match data.state {
                UiState::Edit(ref editstate) => {
                    let mouse_raw = data.draw_info
                        .get_actual_point(data.input_info.mouse_x, data.input_info.mouse_y);
                    let mouse = Point::new(mouse_raw.0, mouse_raw.1);
                    match *editstate {
                        EditState::Mouse(ref mouse_edit_state) => {
                            match *mouse_edit_state {
                                MouseEditState::SetPoint => {
                                    ctxt.arc(mouse.x, mouse.y, 5., 0., 2. * ::std::f64::consts::PI);
                                    ctxt.set_source_rgba(1., 1., 1., 0.4);
                                    ctxt.fill();
                                }
                                MouseEditState::SetMass(center_pt) => {
                                    let radius = mouse.distance_to(&center_pt);
                                    let mass = (radius.powi(3) * ::std::f64::consts::PI) / 0.75;
                                    ctxt.arc(center_pt.x,
                                             center_pt.y,
                                             radius,
                                             0.,
                                             2. * ::std::f64::consts::PI);
                                    let color = mass_to_color(mass);
                                    ctxt.set_source_rgba(color.0, color.1, color.2, 0.4);
                                    ctxt.fill();
                                }
                                MouseEditState::SetVelocity(mass, center_pt) => {
                                    // draw object
                                    let tmp_object =
                                        Object::new(mass, Vector::default(), center_pt);
                                    tmp_object.draw(ctxt, &ObjectColor::FromMass);

                                    // draw potential velocity vector
                                    ctxt.new_path();
                                    ctxt.move_to(center_pt.x, center_pt.y);
                                    ctxt.line_to(mouse.x, mouse.y);
                                    ctxt.set_source_rgba(1., 1., 1., 0.4);
                                    ctxt.set_line_width(data.draw_info.get_actual_width(3.));
                                    ctxt.stroke();
                                    let y_dist = mouse.y - center_pt.y;
                                    let x_dist = mouse.x - center_pt.x;
                                    let line_angle = y_dist.atan2(x_dist);
                                    draw_arrow_head(ctxt,
                                                    mouse.x,
                                                    mouse.y,
                                                    line_angle,
                                                    30f64.to_radians(),
                                                    data.draw_info.get_actual_width(10.),
                                                    1.,
                                                    1.,
                                                    1.,
                                                    0.4);
                                }
                            }
                        }
                        EditState::Input => {}
                    }
                }
                _ => {}
            }

            // get ready for next fps update
            data.fps_info.update_time();
            Inhibit(false)
        });
    }

    fn setup_key_callbacks(&self, window: &Window) {
        self.draw_area.set_can_focus(true);

        {
            let data = self.data.clone();
            window.connect_key_press_event(move |_, key| {
                let ref mut data = *data.get_state_mut();
                match key.get_keyval() {
                    key::Shift_L | key::Shift_R => {
                        data.input_info.shift = true;
                    }
                    key::Control_L | key::Control_R => {
                        data.input_info.ctrl = true;
                    }
                    key::Up => {
                        data.input_info.up = true;
                    }
                    key::Down => {
                        data.input_info.down = true;
                    }
                    key::Left => {
                        data.input_info.left = true;
                    }
                    key::Right => {
                        data.input_info.right = true;
                    }
                    _ => {
                        println!("keypressed");
                    }
                }

                Inhibit(false)
            });
        }

        {
            let data = self.data.clone();
            window.connect_key_release_event(move |_, key| {
                let ref mut data = *data.get_state_mut();
                match key.get_keyval() {
                    key::P | key::p => {
                        let new_state = match data.state {
                            UiState::Paused => {
                                data.update_command_send
                                    .send(UpdaterCommand::Unpause)
                                    .unwrap();
                                UiState::Normal
                            }
                            _ => {
                                data.update_command_send
                                    .send(UpdaterCommand::Pause)
                                    .unwrap();
                                UiState::Paused
                            }
                        };
                        data.state = new_state;
                    }
                    key::E | key::e => {
                        let new_state = match data.state {
                            UiState::Edit(_) => {
                                data.update_command_send
                                    .send(UpdaterCommand::Unpause)
                                    .unwrap();
                                UiState::Normal
                            }
                            _ => {
                                data.update_command_send
                                    .send(UpdaterCommand::Pause)
                                    .unwrap();
                                UiState::Edit(EditState::default())
                            }
                        };
                        data.state = new_state;
                    }
                    key::R | key::r => {
                        data.universe = ColorUniverse::default();
                        data.update_command_send
                            .send(UpdaterCommand::SetUniverse(data.universe.clone()))
                            .unwrap();
                        // clear receiver
                        let mut clear = false;
                        while !clear {
                            match data.universe_recv.try_recv() {
                                Ok(_) => {}
                                Err(TryRecvError::Empty) => clear = true,
                                Err(e) => println!("error: {:?}", e),
                            }
                        }
                    }
                    key::Shift_L | key::Shift_R => {
                        data.input_info.shift = false;
                    }
                    key::Control_L | key::Control_R => {
                        data.input_info.ctrl = false;
                    }
                    key::Up => {
                        data.input_info.up = false;
                    }
                    key::Down => {
                        data.input_info.down = false;
                    }
                    key::Left => {
                        data.input_info.left = false;
                    }
                    key::Right => {
                        data.input_info.right = false;
                    }
                    key::BackSpace => {
                        let ref mut backspace = data.input_info.backspace;
                        backspace.next_state();
                        if backspace.should_reset() {
                            data.draw_info.reset_view();
                            backspace.next_state();
                        }
                    }
                    key::M | key::m => {
                        data.allow_mouse_movement = !data.allow_mouse_movement;
                    }
                    _ => {
                        println!("keypress");
                    }
                }
                Inhibit(false)
            });
        }
    }

    fn setup_mouse_callbacks(&self) {
        self.draw_area.add_events(::gdk_sys::GDK_BUTTON_PRESS_MASK.bits() as i32);
        self.draw_area.add_events(::gdk_sys::GDK_BUTTON_RELEASE_MASK.bits() as i32);
        self.draw_area.add_events(::gdk_sys::GDK_SCROLL_MASK.bits() as i32);
        self.draw_area.add_events(::gdk_sys::GDK_POINTER_MOTION_MASK.bits() as i32);

        self.draw_area.connect_button_press_event(|_, key| {
            println!("mouse press");
            Inhibit(false)
        });

        let data = self.data.clone();
        self.draw_area.connect_button_release_event(move |_, key| {
            let ref mut data = *data.get_state_mut();
            if let UiState::Edit(EditState::Mouse(ref mut mouse_edit_state)) = data.state {
                let mouse_raw = data.draw_info
                    .get_actual_point(data.input_info.mouse_x, data.input_info.mouse_y);
                let mouse = Point::new(mouse_raw.0, mouse_raw.1);
                *mouse_edit_state = match *mouse_edit_state {
                    MouseEditState::SetPoint => MouseEditState::SetMass(mouse),
                    MouseEditState::SetMass(point) => {
                        let radius = mouse.distance_to(&point);
                        let mass = (radius.powi(3) * ::std::f64::consts::PI) / 0.75;
                        MouseEditState::SetVelocity(mass, point)
                    }
                    MouseEditState::SetVelocity(mass, point) => {
                        let y_dist = mouse.y - point.y;
                        let x_dist = mouse.x - point.x;
                        let line_angle = y_dist.atan2(x_dist);
                        let distance = mouse.distance_to(&point);
                        let v_magnitude = distance / (data.update_settings.time() * DEFAULT_FPS);

                        let new_object =
                            Object::new(mass, Vector::new(v_magnitude, line_angle), point);

                        data.universe.add_object(new_object, ObjectColor::FromMass);
                        data.update_command_send
                            .send(UpdaterCommand::SetUniverse(data.universe.clone()))
                            .unwrap();
                        // go back to initial state
                        MouseEditState::SetPoint
                    }
                }
            }
            println!("mouse release");
            Inhibit(false)
        });

        {
            let data = self.data.clone();
            self.draw_area.connect_scroll_event(move |_, scroll| {
                let ref mut data = *data.get_state_mut();
                let (x, y) = scroll.get_position();
                match scroll.as_ref().direction {
                    ::gdk_sys::GdkScrollDirection::Up => {
                        if !(data.input_info.ctrl ^ data.input_info.shift) {
                            // either or none
                            data.draw_info.scale(x, y, 1.01, 1.01);
                        } else if data.input_info.ctrl {
                            data.draw_info.scale(x, y, 1.01, 1.);
                        } else if data.input_info.shift {
                            data.draw_info.scale(x, y, 1., 1.01);
                        }
                    }
                    ::gdk_sys::GdkScrollDirection::Down => {
                        if !(data.input_info.ctrl ^ data.input_info.shift) {
                            data.draw_info.scale(x, y, 0.99, 0.99);
                        } else if data.input_info.ctrl {
                            data.draw_info.scale(x, y, 0.99, 1.);
                        } else if data.input_info.shift {
                            data.draw_info.scale(x, y, 1., 0.99);
                        }
                    }
                    _ => {}
                }
                Inhibit(false)
            });
        }

        {
            let data = self.data.clone();
            self.draw_area.connect_motion_notify_event(move |_, motion| {
                let ref mut data = *data.get_state_mut();
                let (mx, my) = motion.get_position();
                data.input_info.mouse_x = mx;
                data.input_info.mouse_y = my;
                Inhibit(false)
            });
        }
    }

    fn setup_window_callbacks(&self, window: &Window) {
        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        // let data = self.data.clone();
        // window.connect_check_resize(move |_| {
        // set the new size
        // let allocation_size = data.draw_area.get_allocation();
        // let (x_size, y_size) = (allocation_size.width, allocation_size.height);
        // data.draw_area.set_size(x_size as f64, y_size as f64);
        // drawarea.queue_draw();
        // );
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
                    Ok(new_universe) => data.universe = new_universe,
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
