use std::collections::HashMap;

use num_bigint::BigInt;
use num_rational::Rational;

use crate::class::Class;
use crate::dictionary::Dictionary;
use crate::exception::Exception;
use crate::function::Function;
use crate::object::{DeepClone, Object};
use crate::type_id::Implicit;
use crate::variable::{IntoVariable, Variable};
use std::collections::hash_map::DefaultHasher;
use std::convert::TryFrom;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub enum Primitive {
    None,
    Integer(BigInt),
    Float(Rational),
    String(String),
    Boolean(bool),
    List(Vec<Object>),
    Dictionary(Dictionary),
    Function(Function),
    Class(Class),
}

impl Primitive {
    pub fn implicit_type_id(&self) -> Implicit {
        match self {
            Primitive::None => Implicit::new(0),
            Primitive::Integer(_) => Implicit::new(1),
            Primitive::Float(_) => Implicit::new(2),
            Primitive::String(_) => Implicit::new(3),
            Primitive::Boolean(_) => Implicit::new(4),
            Primitive::List(_) => Implicit::new(5),
            Primitive::Dictionary(_) => Implicit::new(6),
            Primitive::Function(_) => Implicit::new(7),
            Primitive::Class(_) => Implicit::new(8),
        }
    }

    pub fn basic_eq(&mut self, other: &mut Self) -> bool {
        match (self, other) {
            (Primitive::Integer(left), Primitive::Integer(right)) => right == left,
            (Primitive::Float(left), Primitive::Float(right)) => right == left,
            (Primitive::String(left), Primitive::String(right)) => left == right,
            (Primitive::Boolean(left), Primitive::Boolean(right)) => left == right,
            (left, right) => (left as *const Primitive) == (right as *const Primitive),
        }
    }

    pub fn basic_hash(&mut self) -> u64 {
        let mut hasher = DefaultHasher::new();
        match self {
            Primitive::Integer(i) => i.hash(&mut hasher),
            Primitive::Float(f) => f.hash(&mut hasher),
            Primitive::String(s) => s.hash(&mut hasher),
            Primitive::Boolean(b) => b.hash(&mut hasher),
            _else => panic!("Cant hash on this primitive"),
        }
        hasher.finish()
    }

    pub fn is_function(&self) -> bool {
        if let Primitive::Function(_) = self {
            true
        } else {
            false
        }
    }

    pub fn get_dictionary(&self) -> Option<&Dictionary> {
        if let Primitive::Dictionary(dict) = self {
            Some(dict)
        } else {
            None
        }
    }

    pub fn get_dictionary_mut(&mut self) -> Option<&mut Dictionary> {
        if let Primitive::Dictionary(dict) = self {
            Some(dict)
        } else {
            None
        }
    }
}

impl<S: AsRef<str>> From<S> for Primitive {
    fn from(s: S) -> Self {
        Primitive::String(s.as_ref().to_string())
    }
}

impl DeepClone for Primitive {
    fn deep_clone(&self) -> Self {
        match self {
            Primitive::List(l) => {
                Primitive::List(l.iter().map(|inner| inner.deep_clone()).collect())
            }
            Primitive::Dictionary(dict) => Primitive::Dictionary(dict.deep_clone()),
            Primitive::Function(f) => {
                unimplemented!()
            }
            Primitive::None => Primitive::None,
            other => other.clone(),
        }
    }
}

impl TryFrom<&Primitive> for bool {
    type Error = Exception;

    fn try_from(value: &Primitive) -> Result<Self, Self::Error> {
        if let Primitive::Boolean(ret) = value {
            Ok(*ret)
        } else {
            Err("Not a boolean value")?
        }
    }
}
