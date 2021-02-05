use crate::primitive::Primitive;
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone)]
pub struct Object {
    primitive: Primitive,
}

impl Object {
    pub fn new(primitive: Primitive) -> Self {
        Object { primitive }
    }
}

pub trait DeepClone {
    fn deep_clone(&self) -> Self;
}

impl DeepClone for Object {
    fn deep_clone(&self) -> Self {
        Self::new(self.primitive.deep_clone())
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        self.primitive.eq(other)
    }
}

impl Eq for Object { }

impl Hash for Object {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.primitive.hash(state)
    }
}

impl Deref for Object {
    type Target = Primitive;

    fn deref(&self) -> &Self::Target {
        &self.primitive
    }
}

impl DerefMut for Object {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.primitive
    }
}
