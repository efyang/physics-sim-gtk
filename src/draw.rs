use cairo::Context;
use color::Color;

pub fn draw_arrow_head(ctxt: &Context,
                       head_x: f64,
                       head_y: f64,
                       line_angle: f64,
                       head_angle: f64,
                       head_length: f64,
                       r: f64,
                       g: f64,
                       b: f64,
                       a: f64) {
    let angle_1 = (line_angle + ::std::f64::consts::PI + head_angle);
    let (s1, c1) = angle_1.sin_cos();
    let (x1, y1) = (head_x + head_length * c1, head_y + head_length * s1);
    let angle_2 = (line_angle + ::std::f64::consts::PI - head_angle);
    let (s2, c2) = angle_2.sin_cos();
    let (x2, y2) = (head_x + head_length * c2, head_y + head_length * s2);
    ctxt.new_path();
    ctxt.move_to(head_x, head_y);
    ctxt.line_to(x1, y1);
    ctxt.line_to(x2, y2);
    ctxt.set_source_rgba(r, g, b, a);
    ctxt.fill();
}
