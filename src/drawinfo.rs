use cairo::{Context, Matrix, MatrixTrait};

pub struct DrawInfo {
    x_size: f64,
    y_size: f64,
    x_scale: f64,
    y_scale: f64,
    x_shift: f64,
    y_shift: f64,
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
}

// INCOMPLETE
