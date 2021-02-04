use num_bigint::BigInt;
use num_rational::Rational;
use crate::object::Object;
use std::collections::HashMap;
use crate::function::Function;

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