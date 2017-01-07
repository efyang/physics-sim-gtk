#[macro_export]
macro_rules! color_func {
    ($this:expr, $func:ident, $color:expr) => {
        $this.$func($color.0, $color.1, $color.2);
    }
}

#[derive(Clone)]
pub struct Color(pub f64, pub f64, pub f64);

#[derive(Clone)]
pub enum ObjectColor {
    UserSet(Color),
    FromMass,
}

impl Into<ObjectColor> for Color {
    fn into(self) -> ObjectColor {
        ObjectColor::UserSet(self)
    }
}

const COLOR_MOD: usize = 300_000_000;
const COLOR_SEGMENT: usize = COLOR_MOD / 6;
pub fn mass_to_color(mass: f64) -> Color {
    let mmass = mass as usize % COLOR_MOD;
    let (r, b, g);
    // setup blue
    if mmass < COLOR_SEGMENT {
        r = 255;
        g = 51 + (255 - 51) * (mmass / COLOR_SEGMENT);
        b = 51;
    } else if mmass >= COLOR_SEGMENT && mmass < 2 * COLOR_SEGMENT {
        r = 255 - (255 - 51) * ((mmass - COLOR_SEGMENT) / COLOR_SEGMENT);
        g = 255;
        b = 51;
    } else if mmass >= COLOR_SEGMENT * 2 && mmass < 3 * COLOR_SEGMENT {
        r = 51;
        g = 255;
        b = 51 + (255 - 51) * ((mmass - COLOR_SEGMENT * 2) / COLOR_SEGMENT);
    } else if mmass >= COLOR_SEGMENT * 3 && mmass < 4 * COLOR_SEGMENT {
        r = 51;
        g = 255 - (255 - 51) * ((mmass - COLOR_SEGMENT * 3) / COLOR_SEGMENT);
        b = 255;
    } else if mmass >= COLOR_SEGMENT * 4 && mmass < 5 * COLOR_SEGMENT {
        r = 51 + (255 - 51) * ((mmass - COLOR_SEGMENT * 4) / COLOR_SEGMENT);
        g = 51;
        b = 255;
    } else {
        r = 255;
        g = 51;
        b = 255 - (255 - 51) * ((mmass - COLOR_SEGMENT * 5) / COLOR_SEGMENT);
    }

    Color(r as f64 / 255., g as f64 / 255., b as f64 / 255.)
}
