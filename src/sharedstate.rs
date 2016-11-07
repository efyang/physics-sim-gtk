use std::rc::Rc;
use std::cell::{RefCell, RefMut, Ref};

#[derive(Clone)]
pub struct SharedState<T> {
    inner: Rc<RefCell<T>>,
}

impl<T> SharedState<T> {
    pub fn new(var: T) -> SharedState<T> {
        SharedState {
            inner: Rc::new(RefCell::new(var))
        }
    }

    pub fn get_state(&self) -> Ref<T> {
        self.inner.borrow()
    }

    pub fn get_state_mut(&self) -> RefMut<T> {
        self.inner.borrow_mut()
    }

    pub fn set_state(&self, new_state: T) {
        *self.inner.borrow_mut() = new_state;
    }
}
