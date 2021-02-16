use crate::object::Object;
use crate::primitive::Primitive;
use std::cell::{Ref, RefCell, RefMut};
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::rc::Rc;

#[derive(Clone)]
pub struct Ptr<T>(Rc<RefCell<T>>);

impl<T> Ptr<T> {
    pub fn new(val: T) -> Self {
        Ptr(Rc::new(RefCell::new(val)))
    }

    pub fn get(&self) -> Ref<T> {
        self.0.borrow()
    }

    pub fn get_mut(&self) -> RefMut<T> {
        self.0.borrow_mut()
    }

    pub fn try_get(&self) -> Option<Ref<T>> {
        self.0.try_borrow().ok()
    }

    pub fn try_get_mut(&self) -> Option<RefMut<T>> {
        self.0.try_borrow_mut().ok()
    }
}

impl<T> From<T> for Ptr<T> {
    fn from(o: T) -> Self {
        Self::new(o)
    }
}

impl From<bool> for Ptr<Object> {
    fn from(b: bool) -> Self {
        Ptr::from(Object::from(Primitive::Boolean(b)))
    }
}

impl<T: Debug> Debug for Ptr<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let get = self.get();
        write!(f, "*{:?}", *get)
    }
}
