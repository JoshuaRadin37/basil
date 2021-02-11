use std::collections::HashMap;

use num_bigint::BigInt;
use num_rational::Rational;

use crate::context::Context;
use crate::dictionary::Dictionary;
use crate::function::Function;
use crate::object::{DeepClone, Object};
use crate::variable::Variable;
use crate::{Executor, FullExecutor};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub enum Primitive {
    Integer(BigInt),
    Float(Rational),
    String(String),
    Boolean(bool),
    List(Vec<Object>),
    Dictionary(Dictionary),
    Function(Function),
    None,
}

impl Primitive {
    pub fn str<E: FullExecutor>(&mut self, context: &mut Context, executor: &mut E) -> String {
        match self {
            Primitive::Integer(i) => {
                format!("{}", i)
            }
            Primitive::Float(f) => {
                format!("{}", f)
            }
            Primitive::String(s) => s.clone(),
            Primitive::Boolean(b) => {
                format!("{}", b)
            }
            Primitive::List(l) => {
                format!(
                    "[{}]",
                    l.iter_mut()
                        .map(|o| o.str(context, executor))
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
            Primitive::Dictionary(dict) => {
                let option = self.find_function("@str");
                if let Some(found) = option {
                    let mut new_context =
                        context.push(&mut Context::from_mapping(&dict.iter(), executor));
                    if let Primitive::String(string) = &*executor.run(&mut new_context, found) {
                        return string.to_owned();
                    } else {
                        panic!("@str didn't return a string")
                    }
                } else {
                    let ret = dict
                        .iter_mut()
                        .map(|(key, val)| {
                            format!(
                                "{:?}:{}",
                                key,
                                val.to_inner()
                                    .write()
                                    .expect("Variable in dictionary poisoned")
                                    .str(context, executor)
                            )
                        })
                        .collect::<Vec<String>>()
                        .join(", ");
                    format! {"{{{}}}", ret}
                }
            }
            Primitive::Function(f) => {
                format!("<function 0x{:x}>", f.id())
            }
            Primitive::None => {
                format!("None")
            }
        }
    }

    fn find_function(&mut self, name: &str) -> Option<&mut Function> {
        if let Primitive::Dictionary(dict) = self {
            let string_function = Primitive::from(name);
            let object = Object::new(string_function);
            let option = dict.get_mut(&object);
            if let Some(found) = option {
                if let Primitive::Function(f) = found {
                    return Some(f);
                }
            }
        }
        None
    }

    pub fn as_hashmap_string(&self) -> Option<String> {
        match self {
            Primitive::Integer(i) => Some(format!("@{}", i)),
            Primitive::String(str) => Some(str.clone()),
            _else => None,
        }
    }

    pub fn get_hash<E: Executor<Function>>(
        &mut self,
        context: &mut Context,
        executor: &mut E,
    ) -> Option<u64> {
        let mut hasher = DefaultHasher::new();
        self.hash_on(context, executor, &mut hasher);
        Some(hasher.finish())
    }

    pub fn hash_on<E: Executor<Function>, H: Hasher>(
        &mut self,
        context: &mut Context,
        executor: &mut E,
        mut hasher: &mut H,
    ) {
        match self {
            Primitive::Integer(b) => b.hash(&mut hasher),
            Primitive::Float(b) => b.hash(&mut hasher),
            Primitive::String(b) => b.hash(&mut hasher),
            Primitive::Boolean(b) => b.hash(&mut hasher),
            Primitive::List(objects) => {
                for object in objects {
                    if let Some(hash) = object.hash_on(context, executor, hasher) {
                        hash.hash(&mut hasher)
                    } else {
                        panic!("can't hash something in this list")
                    }
                }
            }
            Primitive::Dictionary(dict) => {
                let option = self.find_function("@hash");
                if let Some(found) = option {
                    let mut new_context = context.push(Context::from_mapping(dict));
                    if let Primitive::Integer(int) = &*executor.run(&mut new_context, found) {
                        if let Some(ret) = int.to_u64() {
                            ret.hash(hasher)
                        } else {
                            panic!("@hash needed to return a valid u64 integer")
                        }
                    }
                }
                panic!("dictionaries can't be hashed")
            }
            r#else => panic!("{:?} can't be hashed", r#else),
        }
    }

    pub fn eq<E: Executor<Function>>(
        &mut self,
        other: &mut Object,
        context: &mut Context,
        executor: &mut E,
    ) -> bool {
        match (self, other.as_primitive_mut()) {
            (Primitive::Integer(left), Primitive::Integer(right)) => right == left,
            (Primitive::Float(left), Primitive::Float(right)) => right == left,
            (Primitive::String(left), Primitive::String(right)) => left == right,
            (Primitive::Boolean(left), Primitive::Boolean(right)) => left == right,
            (Primitive::List(left), Primitive::List(right)) => {
                if left.len() != right.len() {
                    false
                } else {
                    let mut zip = left.iter_mut().zip(right.iter_mut());
                    for (left, right) in zip {
                        if !left.eq(right, context, executor) {
                            return false;
                        }
                    }
                    true
                }
            }
            (left, right) => (left as *const Primitive) == (right as *const Primitive),
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
            Primitive::Dictionary(dict) => Primitive::Dictionary(
                dict.iter()
                    .map(|(k, v)| (k.deep_clone(), v.deep_clone()))
                    .collect(),
            ),
            Primitive::Function(f) => {}
            Primitive::None => {}
            other => other.clone(),
        }
    }
}
