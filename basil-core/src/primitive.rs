use num_bigint::BigInt;
use num_rational::Rational;
use crate::object::{Object, DeepClone};
use std::collections::HashMap;
use crate::function::Function;

#[derive(Debug, Clone)]
pub enum Primitive {
    Integer(BigInt),
    Float(Rational),
    String(String),
    Boolean(bool),
    List(Vec<Object>),
    Dictionary(HashMap<Object, Object>),
    Function(Function),
    None
}


impl DeepClone for Primitive {
    fn deep_clone(&self) -> Self {
        match self {
            Primitive::List(l) => {
                Primitive::List(l.iter().map(|inner| inner.deep_clone()).collect())
            }
            Primitive::Dictionary(dict) => {}
            Primitive::Function(_) => {}
            Primitive::None => {}
            other => other.clone()
        }
    }
}