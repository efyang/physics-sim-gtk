use std::rc::Rc;
use std::cell::{RefCell, RefMut};

pub struct SharedState<T> {
    inner: Rc<RefCell<T>>,
}

impl<T> SharedState<T> {
    pub fn new(var: T) -> SharedState<T> {
        SharedState {
            inner: Rc::new(RefCell::new(var))
        }
    }

    pub fn get_state(&self) -> T {
        unimplemented!()
    }

    pub fn get_state_mut(&mut self) -> RefMut<T> {
        self.inner.borrow_mut()
    }

    pub fn set_state(&self, new_state: T) {
        unimplemented!()
    }
}
