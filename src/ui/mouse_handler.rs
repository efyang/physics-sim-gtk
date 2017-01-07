use sharedstate::SharedState;
use gdk::{EventButton, EventScroll, EventMotion};
use updater::UpdaterCommand;
use fpsinfo::DEFAULT_FPS;
use color::ObjectColor;

use super::data::UiData;
use super::state::*;
use physics_sim::*;

pub fn mouse_press_handler(data: &SharedState<UiData>, button: &EventButton) {}

pub fn mouse_release_handler(data: &SharedState<UiData>, button: &EventButton) {
    let ref mut data = *data.get_state_mut();
    match button.get_button() {
        // left click
        1 => {
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
        }
        _ => {}
    }
}

pub fn mouse_scroll_handler(data: &SharedState<UiData>, scroll: &EventScroll) {
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
}

pub fn mouse_motion_handler(data: &SharedState<UiData>, motion: &EventMotion) {
    let ref mut data = *data.get_state_mut();
    let (mx, my) = motion.get_position();
    data.input_info.mouse_x = mx;
    data.input_info.mouse_y = my;
}
