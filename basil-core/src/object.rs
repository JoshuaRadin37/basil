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

    pub(crate) fn as_primitive(&self) -> &Primitive {
        &self.primitive
    }

    pub(crate) fn as_primitive_mut(&mut self) -> &mut Primitive {
        &mut self.primitive
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
