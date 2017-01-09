use cairo::Context;

pub struct DrawInfo {
    x_size: f64,
    y_size: f64,
    x_scale: f64,
    y_scale: f64,
    x_shift: f64,
    y_shift: f64,
    draw_grid: bool,
}

impl Default for DrawInfo {
    fn default() -> DrawInfo {
        DrawInfo {
            x_size: 800.,
            y_size: 800.,
            x_scale: 0.1,
            y_scale: 0.1,
            x_shift: 400.,
            y_shift: 400.,
            draw_grid: true,
        }
    }
}

impl DrawInfo {
    pub fn get_actual_point(&self, x: f64, y: f64) -> (f64, f64) {
        //(x * self.x_scale + self.x_shift, y * self.y_scale + self.y_shift)
        ((x - self.x_shift) / self.x_scale, (y - self.y_shift) / self.y_scale)
    }

    // used exclusively for cairo line width
    pub fn get_actual_width(&self, width: f64) -> f64 {
        if self.x_scale < self.y_scale {
            width / self.x_scale
        } else {
            width / self.y_scale
        }
    }

    pub fn set_size(&mut self, x_size: f64, y_size: f64) {
        self.x_size = x_size;
        self.y_size = y_size;
    }

    pub fn get_size(&self) -> (f64, f64) {
        (self.x_size, self.y_size)
    }

    pub fn translate(&mut self, x_trans: f64, y_trans: f64) {
        self.x_shift += x_trans;
        self.y_shift += y_trans;
    }

    pub fn scale(&mut self,
                 scale_center_x: f64,
                 scale_center_y: f64,
                 x_factor: f64,
                 y_factor: f64) {
        self.x_scale *= x_factor;
        self.y_scale *= y_factor;
        self.x_shift -= (x_factor - 1.) * (scale_center_x - self.x_shift);
        self.y_shift -= (y_factor - 1.) * (scale_center_y - self.y_shift);
    }

    pub fn apply(&mut self, ctxt: &Context) {
        ctxt.identity_matrix();
        ctxt.translate(self.x_shift, self.y_shift);
        ctxt.scale(self.x_scale, self.y_scale);
    }

    pub fn reset_view(&mut self) {
        *self = DrawInfo {
            x_size: self.x_size,
            y_size: self.y_size,
            ..DrawInfo::default()
        };
    }

    pub fn toggle_grid(&mut self) {
        self.draw_grid = !self.draw_grid;
    }

    pub fn draw_grid(&self, ctxt: &Context) {
        if self.draw_grid {
            ctxt.set_source_rgba(1., 1., 1., 0.5);
            ctxt.set_line_width(self.get_actual_width(0.5));

            let (min_x, min_y) = self.get_actual_point(0., 0.);
            let (max_x, max_y) = self.get_actual_point(self.x_size, self.y_size);
            let x_range_min = (min_x / GRID_SPACING).ceil() as isize;
            let x_range_max = (max_x / GRID_SPACING).floor() as isize;
            let y_range_min = (min_y / GRID_SPACING).ceil() as isize;
            let y_range_max = (max_y / GRID_SPACING).floor() as isize;
            for x in (x_range_min..x_range_max + 1).map(|x| x as f64 * GRID_SPACING) {
                if x == 0. {
                    ctxt.stroke();
                    ctxt.move_to(x, min_y);
                    ctxt.line_to(x, max_y);
                    ctxt.set_line_width(self.get_actual_width(1.2));
                    ctxt.stroke();
                    ctxt.set_line_width(self.get_actual_width(0.5));
                } else {
                    ctxt.move_to(x, min_y);
                    ctxt.line_to(x, max_y);
                }
            }
            for y in (y_range_min..y_range_max + 1).map(|y| y as f64 * GRID_SPACING) {
                if y == 0. {
                    ctxt.stroke();
                    ctxt.move_to(min_x, y);
                    ctxt.line_to(max_x, y);
                    ctxt.set_line_width(self.get_actual_width(1.2));
                    ctxt.stroke();
                    ctxt.set_line_width(self.get_actual_width(0.5));
                } else {
                    ctxt.move_to(min_x, y);
                    ctxt.line_to(max_x, y);
                }
            }
            ctxt.stroke();
        }
    }
}

const GRID_SPACING: f64 = 1000.;
