use crate::dictionary::Dictionary;
use crate::exception::Exception;
use crate::primitive::Primitive;
use crate::type_id::{Explicit, TypeId};
use crate::variable::{IntoVariable, Variable};
use num_bigint::BigInt;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};

#[derive(Clone)]
pub struct Object {
    type_id: TypeId,
    primitive: Primitive,
}

impl Object {
    pub fn new(primitive: Primitive) -> Self {
        let id = primitive.implicit_type_id();
        Object {
            type_id: id.into(),
            primitive,
        }
    }

    pub fn construct_type_object(id: Explicit, backing_dict: Dictionary) -> Self {
        let primitive = Primitive::Dictionary(backing_dict);
        Object {
            type_id: id.into(),
            primitive,
        }
    }

    pub fn as_primitive(&self) -> &Primitive {
        &self.primitive
    }

    pub(crate) fn as_primitive_mut(&mut self) -> &mut Primitive {
        &mut self.primitive
    }

    pub fn type_id(&self) -> TypeId {
        self.type_id
    }

    pub fn basic_eq(&mut self, other: &mut Self) -> bool {
        self.as_primitive_mut().basic_eq(other.as_primitive_mut())
    }

    pub fn basic_hash(&mut self) -> u64 {
        self.as_primitive_mut().basic_hash()
    }

    pub fn is_class_object(&self) -> bool {
        if let TypeId::Explicit(Explicit(_)) = self.type_id {
            true
        } else {
            false
        }
    }
}

pub trait DeepClone {
    fn deep_clone(&self) -> Self;
}

impl DeepClone for Object {
    fn deep_clone(&self) -> Self {
        Self {
            type_id: self.type_id,
            primitive: self.primitive.deep_clone(),
        }
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

impl From<Primitive> for Object {
    fn from(p: Primitive) -> Self {
        Object::new(p)
    }
}

impl Debug for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let TypeId::Explicit(Explicit(e)) = self.type_id {
            write!(f, "Type {} ", e)?;
        }
        write!(f, "{:?}", self.primitive)
    }
}

impl IntoVariable for Object {
    fn into_variable(self) -> Variable {
        Variable::new(self)
    }
}

macro_rules! into_object {
    ($rust:ty, $prim:path) => {
        impl From<$rust> for Object {
            fn from(fr: $rust) -> Object {
                Object::from($prim(fr))
            }
        }
    };
    ($rust:ty, $prim:path, $func:path) => {
        impl From<$rust> for Object {
            fn from(fr: $rust) -> Object {
                Object::from($prim($func(fr)))
            }
        }
    };
}

into_object!(bool, Primitive::Boolean);

into_object!(u8, Primitive::Integer, BigInt::from);
into_object!(u16, Primitive::Integer, BigInt::from);
into_object!(u32, Primitive::Integer, BigInt::from);
into_object!(u64, Primitive::Integer, BigInt::from);
into_object!(usize, Primitive::Integer, BigInt::from);

into_object!(i8, Primitive::Integer, BigInt::from);
into_object!(i16, Primitive::Integer, BigInt::from);
into_object!(i32, Primitive::Integer, BigInt::from);
into_object!(i64, Primitive::Integer, BigInt::from);
into_object!(isize, Primitive::Integer, BigInt::from);

into_object!(&str, Primitive::String, String::from);
into_object!(String, Primitive::String);
into_object!(&String, Primitive::String, String::clone);
