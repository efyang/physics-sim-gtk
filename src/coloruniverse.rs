use physics_sim::{Universe, Object, Point};
use super::color::ObjectColor;
use std::collections::VecDeque;
use std::ops::Index;

const MAX_POSITIONS_STORED: usize = 100000;
const ADD_POSITION_MULTIPLE: usize = 2;

#[derive(Clone)]
pub struct ColorUniverse {
    universe: Universe,
    colors: Vec<ObjectColor>,
    previous_positions: Vec<CapVecDeque>,
    update_counter: usize,
}

impl Default for ColorUniverse {
    fn default() -> ColorUniverse {
        ColorUniverse {
            universe: Universe::default(),
            colors: Vec::new(),
            previous_positions: Vec::new(),
            update_counter: 0,
        }
    }
}

impl ColorUniverse {
    pub fn objects(&self) -> &[Object] {
        self.universe.objects()
    }

    pub fn object_mapped<'a>
        (&'a self)
         -> impl Iterator<Item = (&'a Object, &'a ObjectColor, &'a CapVecDeque)> {
        self.universe
            .objects()
            .iter()
            .zip(self.colors.iter())
            .zip(self.previous_positions.iter())
            .map(|((o, c), v)| (o, c, v))
    }

    pub fn add_object(&mut self, object: Object, color: ObjectColor) {
        self.universe.add_object(object);
        self.colors.push(color);
        self.previous_positions.push(CapVecDeque::with_capacity(MAX_POSITIONS_STORED));
    }

    pub fn update_state_repeat(&mut self, time: f64, iterations: usize) {
        for index in self.universe.update_state_repeat(time, iterations) {
            self.colors.remove(index);
            self.previous_positions.remove(index);
        }
        self.update_counter += 1;
        if self.update_counter % ADD_POSITION_MULTIPLE == 0 {
            for i in 0..self.objects().len() {
                let pt = self.objects()[i].position().clone();
                self.previous_positions[i].add_element(pt);
            }
        }
    }
}

trait CircularBuffer {
    fn add_element(&mut self, element: Point);
}

impl CircularBuffer for CapVecDeque {
    fn add_element(&mut self, element: Point) {
        if self.len() == self.capacity() {
            self.pop_front();
            self.push_back(element);
        } else {
            self.push_back(element);
        }
    }
}

// ugly as hell workaround because normal vecdeque does not preserve capacity on clone
pub struct CapVecDeque {
    capacity: usize,
    inner: VecDeque<Point>,
}

impl CapVecDeque {
    pub fn new() -> CapVecDeque {
        CapVecDeque {
            capacity: 1,
            inner: VecDeque::new(),
        }
    }

    pub fn with_capacity(cap: usize) -> CapVecDeque {
        CapVecDeque {
            capacity: cap,
            inner: VecDeque::with_capacity(cap),
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn pop_front(&mut self) -> Option<Point> {
        self.inner.pop_front()
    }

    pub fn push_back(&mut self, elem: Point) {
        self.inner.push_back(elem)
    }
}

impl Index<usize> for CapVecDeque {
    type Output = Point;
    fn index(&self, index: usize) -> &Point {
        &self.inner[index]
    }
}

impl Clone for CapVecDeque {
    fn clone(&self) -> Self {
        CapVecDeque {
            capacity: self.capacity,
            inner: {
                let mut new = VecDeque::with_capacity(self.capacity);
                new.extend(self.inner.iter().cloned());
                new
            }
        }
    }
}
