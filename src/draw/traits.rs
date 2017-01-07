use physics_sim::Object;
use cairo::Context;
use color::{ObjectColor, mass_to_color};
use coloruniverse::ColorUniverse;

pub trait DrawAll {
    fn draw_all(&self, &Context);
}

impl DrawAll for ColorUniverse {
    fn draw_all(&self, ctxt: &Context) {
        ctxt.set_operator(::cairo::Operator::Source);
        ctxt.set_source_rgb(0.0, 0.0, 0.0);
        ctxt.paint();

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
        let ctmp;
        let color = match *color {
            ObjectColor::UserSet(ref c) => c,
            ObjectColor::FromMass => {
                ctmp = mass_to_color(self.mass());
                &ctmp
            }
        };

        ctxt.arc(self.position().x,
                 self.position().y,
                 self.radius(),
                 0.,
                 2. * ::std::f64::consts::PI);
        color_func!(ctxt, set_source_rgb, color);
        ctxt.fill();
    }
}
