use crate::primitive::Primitive;

#[derive(Debug, Clone)]
pub struct Object {
    primitive: Primitive
}

pub trait DeepClone {
    fn deep_clone(&self) -> Self;
}

impl DeepClone for Object {
    fn deep_clone(&self) -> Self {

    }
}