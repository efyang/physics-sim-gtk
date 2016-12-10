#[macro_export]
macro_rules! color_func {
    (this:expr, func:ident, color:expr) => {
        this.func(color.0, color.1, color.2);
    }
}

#[derive(Clone)]
pub struct Color(f64, f64, f64);

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
