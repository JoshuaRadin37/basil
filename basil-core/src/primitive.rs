use num_bigint::BigInt;
use num_rational::Rational;
use crate::object::{Object, DeepClone};
use std::collections::HashMap;
use crate::function::Function;
use crate::context::Context;
use crate::variable::Variable;

#[derive(Debug, Clone, Hash)]
pub enum Primitive {
    Integer(BigInt),
    Float(Rational),
    String(String),
    Boolean(bool),
    List(Vec<Object>),
    Dictionary(HashMap<Object, Variable>),
    Function(Function),
    None
}

impl Primitive {
    pub fn str(&mut self, context: Context) -> String {
        match self {
            Primitive::Integer(i) => {
                format!("{}", i)
            }
            Primitive::Float(f) => {
                format!("{}", f)
            }
            Primitive::String(s) => {
                s.clone()
            }
            Primitive::Boolean(b) => {
                format!("{}", b)
            }
            Primitive::List(l) => {
                format!("[{}]", l.iter_mut().map(|o| o.str(context)).join(", "))
            }
            Primitive::Dictionary(dict) => {
                let option = self.find_function("@str");
                if let Some(found) = option {
                    let mut new_context = context.push(Context::from_mapping(dict));
                    found
                }
            }
            Primitive::Function(f) => {
                format!("<function {}>", f.id())
            }
            Primitive::None => {
                format!("None")
            }
        }
    }

    fn find_function(&mut self, name: &str) -> Option<&mut Function> {
        let string_function = Primitive::from(name);
        let object = Object::new(string_function);
        let option = dict.get(&object);
        if let Some(found) = option {
            if let Primitive::Function(f) = &mut found {
                return Some(f)
            }
        }
        None
    }

    pub fn as_hashmap_string(&self) -> Option<String> {
        match self {
            Primitive::Integer(i) => { Some(format!("@{}", i)) }
            Primitive::String(str) => { Some(str.clone()) }
            _else => None
        }
    }
}

impl <S : AsRef<str>> From<S> for Primitive {
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
            Primitive::Dictionary(dict) => {
                Primitive::Dictionary(dict.iter()
                    .map(|(k, v)| (k.deep_clone(), v.deep_clone()))
                    .collect())
            }
            Primitive::Function(f) => {}
            Primitive::None => {}
            other => other.clone()
        }
    }
}

