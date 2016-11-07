use physics_sim::Object;
use cairo::prelude::*;
use cairo::Context;

pub trait Draw {
    fn draw(&self, &Context);
}

impl Draw for Object {
    fn draw(&self, ctxt: &Context) {

        unimplemented!()
    }
}

fn mass_to_color(mass: f64) -> f64 {
    unimplemented!()
}
