use cairo::Context;
use sharedstate::SharedState;
use physics_sim::Vector;
use draw::*;
use color::*;
use std::collections::VecDeque;
use coloruniverse::CapVecDeque;
use super::data::UiData;
use super::state::*;
use physics_sim::*;

pub fn draw_handler(data: &SharedState<UiData>, ctxt: &Context) {
    let ref mut data = *data.get_state_mut();
    // draw background
    ctxt.set_operator(::cairo::Operator::Source);
    ctxt.set_source_rgb(0.0, 0.0, 0.0);
    ctxt.paint();
    // apply the drawing info
    data.draw_info.apply(ctxt);
    // draw grid
    data.draw_info.draw_grid(ctxt);
    // draw everything
    data.universe.draw_all(ctxt, &data.draw_info);
    // draw the mode

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
                            let tmp_object = Object::new(mass, Vector::default(), center_pt);
                            tmp_object.draw(ctxt, &data.draw_info, &ObjectColor::FromMass);

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
}
