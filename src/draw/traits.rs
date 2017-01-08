use physics_sim::{Object, Point};
use cairo::Context;
use color::{ObjectColor, mass_to_color};
use coloruniverse::{ColorUniverse, CapVecDeque};
use super::info::DrawInfo;

pub trait DrawAll {
    fn draw_all(&self, &Context, &DrawInfo);
}

impl DrawAll for ColorUniverse {
    fn draw_all(&self, ctxt: &Context, info: &DrawInfo) {
        for (object, color, positions) in self.object_mapped() {
            object.draw(ctxt, info, color, positions);
        }
    }
}

pub trait Draw {
    fn draw(&self, &Context, &DrawInfo, &ObjectColor, positions: &CapVecDeque);
}

impl Draw for Object {
    fn draw(&self, ctxt: &Context, info: &DrawInfo, color: &ObjectColor, positions: &CapVecDeque) {
        let ctmp;
        let color = match *color {
            ObjectColor::UserSet(ref c) => c,
            ObjectColor::FromMass => {
                ctmp = mass_to_color(self.mass());
                &ctmp
            }
        };
        color_func!(ctxt, set_source_rgb, color);

        ctxt.set_line_width(info.get_actual_width(1.));
        // draw the path
        let current = self.position();
        draw_positions(&self.position(), ctxt, positions);
        ctxt.stroke();

        // draw the object
        ctxt.arc(self.position().x,
                 self.position().y,
                 self.radius(),
                 0.,
                 2. * ::std::f64::consts::PI);
        ctxt.fill();
    }
}

fn draw_positions(current: &Point, ctxt: &Context, positions: &CapVecDeque) {
    match positions.len() {
        0 => {}
        1 => {
            ctxt.line_to(positions[0].x, positions[0].y);
        }
        2 => {
            ctxt.curve_to(positions[0].x,
                          positions[0].y,
                          positions[0].x,
                          positions[0].y,
                          positions[1].x,
                          positions[1].y);
        }
        3 => {
            ctxt.curve_to(positions[0].x,
                          positions[0].y,
                          positions[1].x,
                          positions[1].y,
                          positions[2].x,
                          positions[2].y);
        }
        _ => {
            let max_mult = (positions.len() - 1) / 3;
            for i in (0..max_mult).map(|x| x * 3) {
                ctxt.curve_to(positions[i].x,
                              positions[i].y,
                              positions[i + 1].x,
                              positions[i + 1].y,
                              positions[i + 2].x,
                              positions[i + 2].y);
            }
            let remaining = {
                let mut new = CapVecDeque::with_capacity(positions.len() - max_mult * 3);
                for i in max_mult * 3..positions.len() {
                    new.push_back(positions[i])
                }
                new
            };
            draw_positions(&positions[max_mult * 3 - 1], ctxt, &remaining);
        }
    }
}
