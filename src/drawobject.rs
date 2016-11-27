use physics_sim::{Object, Universe};
use cairo::prelude::*;
use cairo::Context;
use color::ObjectColor;
use coloruniverse::ColorUniverse;

pub trait DrawAll {
    fn draw_all(&self, &Context);
}

impl DrawAll for ColorUniverse {
    fn draw_all(&self, ctxt: &Context) {
        for (object, color) in self.object_mapped_colors() {
            object.draw(ctxt, color);
        }
    }
}

pub trait Draw {
    fn draw(&self, &Context, &ObjectColor);
}

impl Draw for Object {
    fn draw(&self, ctxt: &Context, color: &ObjectColor) {

        //unimplemented!()
    }
}

fn mass_to_color(mass: f64) -> f64 {
    unimplemented!()
}
