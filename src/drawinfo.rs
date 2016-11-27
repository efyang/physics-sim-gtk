use cairo::Context;

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
            x_scale: 1.,
            y_scale: 1.,
            x_shift: 0.,
            y_shift: 0.,
        }
    }
}

impl DrawInfo {
    pub fn set_size(&mut self, x_size: f64, y_size: f64) {
        self.x_size = x_size;
        self.y_size = y_size;
    }

    pub fn apply(&self, ctxt: &Context) {
        ctxt.identity_matrix();
        ctxt.scale(self.x_scale, self.y_scale);
        ctxt.translate(self.x_shift, self.y_shift);
    }
}

// INCOMPLETE
