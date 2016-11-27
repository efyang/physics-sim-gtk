use physics_sim::{Universe, Object};
use super::color::ObjectColor;

pub struct ColorUniverse {
    universe: Universe,
    colors: Vec<ObjectColor>
}

impl Default for ColorUniverse {
    fn default() -> ColorUniverse {
        ColorUniverse {
            universe: Universe::default(),
            colors: Vec::new(),
        }
    }
}

impl ColorUniverse {
    pub fn objects(&self) -> &[Object] {
        self.universe.objects()
    }

    pub fn object_mapped_colors<'a>(&'a self) -> impl Iterator<Item=(&'a Object, &'a ObjectColor)> {
        self.universe.objects().iter().zip(self.colors.iter())
    }

    pub fn add_object(&mut self, object: Object, color: ObjectColor) {
        self.universe.add_object(object);
        self.colors.push(color);
    }

    pub fn update_state_repeat(&mut self, time: f64, iterations: usize) {
        for index in self.universe.update_state_repeat(time, iterations) {
            self.colors.remove(index);
        }
    }

    pub fn update_state(&mut self, time: f64) {
        for index in self.universe.update_state(time) {
            self.colors.remove(index);
        }
    }

    // add function to update the universe with list of removal indices
}
