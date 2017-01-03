use gtk::prelude::*;
use gtk::{self, Window, WindowType, DrawingArea, Orientation};
use cairo::prelude::*;
use coloruniverse::ColorUniverse;
use uistate::UiState;
use sharedstate::SharedState;
use fpsinfo::*;
use drawinfo::DrawInfo;
use drawobject::{Draw, DrawAll};
use std::sync::mpsc::{Sender, Receiver, TryRecvError};
use updater::{UpdateSettings, UpdaterCommand, Updater};
use iteration_result::IterationResult;
use keys::InputInfo;
use gdk::enums::key;
use editstate::{EditState, MouseEditState};
use physics_sim::{Object, Point, Vector};
use color::{mass_to_color, ObjectColor};
use draw::draw_arrow_head;

pub struct Ui {
    fpsinfo: SharedState<FpsInfo>,
    state: SharedState<UiState>,
    drawarea: SharedState<DrawingArea>,
    universe: SharedState<ColorUniverse>,
    drawinfo: SharedState<DrawInfo>,
    universe_recv: SharedState<Receiver<ColorUniverse>>,
    update_settings: SharedState<UpdateSettings>,
    update_command_send: SharedState<Sender<UpdaterCommand>>,
    input_info: SharedState<InputInfo>,
    allow_mouse_movement: SharedState<bool>,
}

impl Ui {
    pub fn initialize() -> Ui {
        let (mut updater, universe_recv, update_command_send) =
            Updater::new(ColorUniverse::default());

        let window = default_window();
        let mainsplit = gtk::Box::new(Orientation::Vertical, 10);
        let drawarea = DrawingArea::new();
        let input_interface = gtk::Box::new(Orientation::Horizontal, 10);
        mainsplit.pack_start(&drawarea, true, true, 0);
        mainsplit.pack_end(&input_interface, false, true, 0);
        window.add(&mainsplit);
        window.show_all();

        let this = Ui {
            fpsinfo: SharedState::new(FpsInfo::default()),
            state: SharedState::new(UiState::default()),
            drawarea: SharedState::new(drawarea),
            universe: SharedState::new(ColorUniverse::default()),
            drawinfo: SharedState::new(DrawInfo::default()),
            universe_recv: SharedState::new(universe_recv),
            update_settings: SharedState::new(UpdateSettings::default()),
            update_command_send: SharedState::new(update_command_send),
            input_info: SharedState::new(InputInfo::default()),
            allow_mouse_movement: SharedState::new(true),
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
        let fpsinfo = self.fpsinfo.clone();
        let universe = self.universe.clone();
        let drawarea = self.drawarea.get_state();
        let drawinfo = self.drawinfo.clone();
        let uistate = self.state.clone();
        let input_info = self.input_info.clone();
        drawarea.set_size_request(800, 800);
        drawarea.connect_draw(move |drawarea, ctxt| {
            // apply the drawing info
            drawinfo.get_state_mut().apply(ctxt);
            // draw everything
            universe.get_state().draw_all(ctxt);
            // draw the mode;

            // draw the edit information(if its in edit mode)
            match *uistate.get_state() {
                UiState::Edit(ref editstate) => {
                    let ref input_info = *input_info.get_state();
                    let mouse_raw = drawinfo.get_state()
                        .get_actual_point(input_info.mouse_x, input_info.mouse_y);
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
                                    ctxt.set_line_width(drawinfo.get_state().get_actual_width(3.));
                                    ctxt.stroke();
                                    let y_dist = mouse.y - center_pt.y;
                                    let x_dist = mouse.x - center_pt.x;
                                    let line_angle = y_dist.atan2(x_dist);
                                    draw_arrow_head(ctxt,
                                                    mouse.x,
                                                    mouse.y,
                                                    line_angle,
                                                    30f64.to_radians(),
                                                    drawinfo.get_state().get_actual_width(10.),
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
            fpsinfo.get_state_mut().update_time();
            Inhibit(false)
        });
    }

    fn setup_key_callbacks(&self, window: &Window) {
        let drawarea = self.drawarea.get_state();
        drawarea.set_can_focus(true);

        {
            let input_info = self.input_info.clone();
            let drawinfo = self.drawinfo.clone();
            window.connect_key_press_event(move |_, key| {
                match key.get_keyval() {
                    key::Shift_L | key::Shift_R => {
                        input_info.get_state_mut().shift = true;
                    }
                    key::Control_L | key::Control_R => {
                        input_info.get_state_mut().ctrl = true;
                    }
                    key::Up => {
                        input_info.get_state_mut().up = true;
                    }
                    key::Down => {
                        input_info.get_state_mut().down = true;
                    }
                    key::Left => {
                        input_info.get_state_mut().left = true;
                    }
                    key::Right => {
                        input_info.get_state_mut().right = true;
                    }
                    _ => {
                        println!("keypressed");
                    }
                }

                Inhibit(false)
            });
        }

        {
            let update_command_send = self.update_command_send.clone();
            let uistate = self.state.clone();
            let input_info = self.input_info.clone();
            let drawinfo = self.drawinfo.clone();
            let allow_mouse_movement = self.allow_mouse_movement.clone();
            let universe = self.universe.clone();
            let universe_recv = self.universe_recv.clone();
            window.connect_key_release_event(move |_, key| {
                match key.get_keyval() {
                    key::P | key::p => {
                        let new_state = match *uistate.get_state() {
                            UiState::Paused => {
                                update_command_send.get_state()
                                    .send(UpdaterCommand::Unpause)
                                    .unwrap();
                                UiState::Normal
                            }
                            _ => {
                                update_command_send.get_state()
                                    .send(UpdaterCommand::Pause)
                                    .unwrap();
                                UiState::Paused
                            }
                        };
                        *uistate.get_state_mut() = new_state;
                    }
                    key::E | key::e => {
                        let new_state = match *uistate.get_state() {
                            UiState::Edit(_) => {
                                update_command_send.get_state()
                                    .send(UpdaterCommand::Unpause)
                                    .unwrap();
                                UiState::Normal
                            }
                            _ => {
                                update_command_send.get_state()
                                    .send(UpdaterCommand::Pause)
                                    .unwrap();
                                UiState::Edit(EditState::default())
                            }
                        };
                        *uistate.get_state_mut() = new_state;
                    }
                    key::R | key::r => {
                        *universe.get_state_mut() = ColorUniverse::default();
                        update_command_send.get_state_mut()
                            .send(UpdaterCommand::SetUniverse(universe.get_state().clone()))
                            .unwrap();
                        // clear receiver
                        let universe_recv = universe_recv.get_state();
                        let mut clear = false;
                        while !clear {
                            match universe_recv.try_recv() {
                                Ok(_) => {}
                                Err(TryRecvError::Empty) => clear = true,
                                Err(e) => println!("error: {:?}", e),
                            }
                        }
                    }
                    key::Shift_L | key::Shift_R => {
                        input_info.get_state_mut().shift = false;
                    }
                    key::Control_L | key::Control_R => {
                        input_info.get_state_mut().ctrl = false;
                    }
                    key::Up => {
                        input_info.get_state_mut().up = false;
                    }
                    key::Down => {
                        input_info.get_state_mut().down = false;
                    }
                    key::Left => {
                        input_info.get_state_mut().left = false;
                    }
                    key::Right => {
                        input_info.get_state_mut().right = false;
                    }
                    key::BackSpace => {
                        let ref mut backspace = input_info.get_state_mut().backspace;
                        backspace.next_state();
                        if backspace.should_reset() {
                            drawinfo.get_state_mut().reset_view();
                            backspace.next_state();
                        }
                    }
                    key::M | key::m => {
                        let new_allow = !*allow_mouse_movement.get_state();
                        *allow_mouse_movement.get_state_mut() = new_allow;
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
        let drawarea = self.drawarea.get_state();

        drawarea.add_events(::gdk_sys::GDK_BUTTON_PRESS_MASK.bits() as i32);
        drawarea.add_events(::gdk_sys::GDK_BUTTON_RELEASE_MASK.bits() as i32);
        drawarea.add_events(::gdk_sys::GDK_SCROLL_MASK.bits() as i32);
        drawarea.add_events(::gdk_sys::GDK_POINTER_MOTION_MASK.bits() as i32);

        drawarea.connect_button_press_event(|_, key| {
            println!("mouse press");
            Inhibit(false)
        });

        let uistate = self.state.clone();
        let input_info = self.input_info.clone();
        let drawinfo = self.drawinfo.clone();
        let update_settings = self.update_settings.clone();
        let update_command_send = self.update_command_send.clone();
        let universe = self.universe.clone();
        drawarea.connect_button_release_event(move |_, key| {
            let ref mut uistate = *uistate.get_state_mut();
            if let UiState::Edit(EditState::Mouse(ref mut mouse_edit_state)) = *uistate {
                let ref input_info = *input_info.get_state();
                let mouse_raw = drawinfo.get_state()
                    .get_actual_point(input_info.mouse_x, input_info.mouse_y);
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
                        let v_magnitude = distance /
                                          (update_settings.get_state().time() * DEFAULT_FPS);

                        let new_object =
                            Object::new(mass, Vector::new(v_magnitude, line_angle), point);

                        universe.get_state_mut().add_object(new_object, ObjectColor::FromMass);
                        update_command_send.get_state_mut()
                            .send(UpdaterCommand::SetUniverse(universe.get_state().clone()))
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
            let drawinfo = self.drawinfo.clone();
            let input_info = self.input_info.clone();
            drawarea.connect_scroll_event(move |_, scroll| {
                let ref input_info = *input_info.get_state();
                let (x, y) = scroll.get_position();
                match scroll.as_ref().direction {
                    ::gdk_sys::GdkScrollDirection::Up => {
                        if !(input_info.ctrl ^ input_info.shift) {
                            // either or none
                            drawinfo.get_state_mut().scale(x, y, 1.01, 1.01);
                        } else if input_info.ctrl {
                            drawinfo.get_state_mut().scale(x, y, 1.01, 1.);
                        } else if input_info.shift {
                            drawinfo.get_state_mut().scale(x, y, 1., 1.01);
                        }
                    }
                    ::gdk_sys::GdkScrollDirection::Down => {
                        if !(input_info.ctrl ^ input_info.shift) {
                            drawinfo.get_state_mut().scale(x, y, 0.99, 0.99);
                        } else if input_info.ctrl {
                            drawinfo.get_state_mut().scale(x, y, 0.99, 1.);
                        } else if input_info.shift {
                            drawinfo.get_state_mut().scale(x, y, 1., 0.99);
                        }
                    }
                    _ => {}
                }
                Inhibit(false)
            });
        }

        {
            let input_info = self.input_info.clone();
            drawarea.connect_motion_notify_event(move |_, motion| {
                let ref mut input_info = *input_info.get_state_mut();
                let (mx, my) = motion.get_position();
                input_info.mouse_x = mx;
                input_info.mouse_y = my;
                Inhibit(false)
            });
        }
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
            let allocation_size = drawarea.get_allocation();
            let (x_size, y_size) = (allocation_size.width, allocation_size.height);
            drawinfo.get_state_mut().set_size(x_size as f64, y_size as f64);
            drawarea.queue_draw();
        });
    }

    // fn setup_button_callbacks(buttons: ) {

    // }

    fn handle_input_iteration(&mut self) {
        let input_info = self.input_info.get_state();
        let ref mut drawinfo = *self.drawinfo.get_state_mut();
        let allow_mouse_movement = *self.allow_mouse_movement.get_state();

        // handle the mouse position (if its within borders then move view)
        let (x_size, y_size) = drawinfo.get_size();
        if allow_mouse_movement && input_info.mouse_within_any_side_border(x_size, y_size) {
            let max_movement = 20.;
            let (mut x_trans, mut y_trans) = (0., 0.);
            if let Some(distance) = input_info.mouse_top_move_border(y_size) {
                y_trans = max_movement * (1. - distance / ::keys::MOUSE_MOVEMENT_BORDER_WIDTH);
            } else if let Some(distance) = input_info.mouse_bottom_move_border(y_size) {
                y_trans = -max_movement * (1. - distance / ::keys::MOUSE_MOVEMENT_BORDER_WIDTH);
            }
            if let Some(distance) = input_info.mouse_left_move_border(x_size) {
                x_trans = max_movement * (1. - distance / ::keys::MOUSE_MOVEMENT_BORDER_WIDTH);
            } else if let Some(distance) = input_info.mouse_right_move_border(x_size) {
                x_trans = -max_movement * (1. - distance / ::keys::MOUSE_MOVEMENT_BORDER_WIDTH);
            }
            drawinfo.translate(x_trans, y_trans);
        } else {
            // handle the arrow keys
            if input_info.up {
                drawinfo.translate(0., 7.5);
            } else if input_info.down {
                drawinfo.translate(0., -7.5);
            }
            if input_info.left {
                drawinfo.translate(7.5, 0.);
            } else if input_info.right {
                drawinfo.translate(-7.5, 0.);
            }
        }
    }

    pub fn iterate(&mut self) -> IterationResult {
        self.handle_input_iteration();

        if self.fpsinfo.get_state().should_redraw() {
            self.drawarea.get_state().queue_draw();
        }

        // check the updater output
        match *self.state.get_state_mut() {
            UiState::Paused | UiState::Edit(_) => {
                println!("paused");
                // set the current universe
                self.update_command_send
                    .get_state_mut()
                    .send(UpdaterCommand::SetUniverse(self.universe.get_state().clone()))
                    .unwrap();
                // clear the receiver
                let mut clear = false;
                let universe_recv = self.universe_recv.get_state();
                while !clear {
                    match universe_recv.try_recv() {
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
                println!("not paused");
                match self.universe_recv.get_state().try_recv() {
                    Ok(new_universe) => *self.universe.get_state_mut() = new_universe,
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
