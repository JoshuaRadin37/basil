use std::cell::RefCell;
use std::rc::Rc;

use crate::exception::Exception;
use crate::object::{DeepClone, Object};
use crate::primitive::Primitive;
use crate::ptr::Ptr;
use num_bigint::BigInt;
use num_traits::ToPrimitive;
use std::convert::TryFrom;
use std::fmt::{Debug, Formatter};

#[derive(Clone)]
pub struct Variable {
    inner: Ptr<Ptr<Object>>,
}

impl Variable {
    pub fn new<P: Into<Ptr<Object>>>(object: P) -> Variable {
        Self {
            inner: Ptr::new(object.into()),
        }
    }

    /// Returns a clone of only the inner object of this variable
    pub fn inner_clone(&self) -> Variable {
        let read = self
            .inner
            .try_get()
            .expect("Variable already mutably borrowed");
        let o = read.get().clone();
        Variable::new(o)
    }

    /// Creates a deep clone of this variable
    pub fn deep_clone(&self) -> Variable {
        Variable::new(
            self.inner
                .try_get()
                .expect("Variable already mutably borrowed")
                .get()
                .deep_clone(),
        )
    }

    /// Gets the inner backing ARC
    pub fn to_inner(&self) -> &Ptr<Ptr<Object>> {
        &self.inner
    }

    pub fn get_object(&self) -> Ptr<Object> {
        let ptr = self.inner.get();
        ptr.clone()
    }

    pub fn set_object<V: IntoVariable>(&mut self, var: V) {
        let var = var.into_variable();
        let ptr = var.inner.get().clone();
        *self.inner.get_mut() = ptr;
    }

    pub fn get_member<
        Hash: FnMut(&mut Object) -> u64,
        Eq: FnMut(&mut Object, &mut Object) -> bool,
    >(
        &self,
        mut member: Object,
        hash: Hash,
        eq: Eq,
    ) -> Result<Variable, Exception> {
        let inner = self.to_inner();
        let borrow = inner.get();
        let x = borrow.get();
        let dictionary = x
            .get_dictionary()
            .ok_or(Exception::from(format!("{:?} is not a dictionary", borrow)))?;

        // This is the variable representing what the dictionary is pointing to
        // in C terms, &(dict->member)
        //  We want to create a new pointer to the inner pointer
        let variable = dictionary
            .get(&mut member, hash, eq)
            .ok_or(Exception::from(format!(
                "{:?} is not a member of {:?}",
                member, self
            )))?;
        Ok(variable.clone())
    }

    pub fn get_member_or_create<
        Hash: FnMut(&mut Object) -> u64 + Clone,
        Eq: FnMut(&mut Object, &mut Object) -> bool + Clone,
    >(
        &self,
        mut member: Object,
        hash: Hash,
        eq: Eq,
    ) -> Result<Variable, Exception> {
        let inner = self.to_inner();
        let borrow = inner.get();
        let mut x = borrow.get_mut();
        let dictionary = x
            .get_dictionary_mut()
            .ok_or_else(|| Exception::from(format!("{:?} is not a dictionary", borrow)))?;
        // This is the variable representing what the dictionary is pointing to
        // in C terms, &(dict->member)
        //  We want to create a new pointer to the inner pointer
        if dictionary
            .get(&mut member, hash.clone(), eq.clone())
            .is_none()
        {
            dictionary.insert(
                member.clone(),
                Primitive::None.into_variable(),
                hash.clone(),
                eq.clone(),
            );
        }

        let variable = dictionary.get(&mut member, hash, eq).ok_or_else(|| {
            Exception::from(format!("{:?} is not a member of {:?}", member, self))
        })?;
        Ok(variable.clone())
    }

    /*
    pub fn get_member_mut<
        Hash: FnMut(&mut Object) -> u64,
        Eq: FnMut(&mut Object, &mut Object) -> bool,
    >(
        &self,
        mut member: Object,
        hash: Hash,
        eq: Eq,
    ) -> Result<&mut Variable, Exception> {
        unimplemented!();
        let inner = self.to_inner();
        let mut borrow = inner.borrow_mut();
        let exception = format!("{:?} is not a dictionary", borrow);
        let dictionary = borrow
            .get_dictionary_mut()
            .ok_or(Exception::from(exception))?;
        dictionary
            .get_mut(&mut member, hash, eq)
            .ok_or(Exception::from(format!(
                "{:?} is not a member of {:?}",
                member, self
            )))
    }

     */
}

pub trait IntoVariable {
    fn into_variable(self) -> Variable;
}

impl IntoVariable for Variable {
    fn into_variable(self) -> Variable {
        self
    }
}

impl From<Object> for Variable {
    fn from(o: Object) -> Self {
        Variable::new(Ptr::new(o))
    }
}

/*
impl<I: Into<Object>> From<I> for Variable {
    fn from(i: I) -> Self {
        Variable {
            inner: Rc::new(RefCell::new(i.into())),
        }
    }
}

 */

impl From<Variable> for Ptr<Object> {
    fn from(v: Variable) -> Self {
        v.get_object()
    }
}

impl From<&Variable> for Ptr<Object> {
    fn from(v: &Variable) -> Self {
        v.get_object()
    }
}

impl Debug for Variable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

impl IntoVariable for bool {
    fn into_variable(self) -> Variable {
        Primitive::Boolean(self).into_variable()
    }
}

macro_rules! primitive_into_variable {
    ($rust:ty, $prim:path) => {
        impl IntoVariable for $rust {
            fn into_variable(self) -> Variable {
                Variable::new(Object::from($prim(self)))
            }
        }
    };
    ($rust:ty, $prim:path, $func:path) => {
        impl IntoVariable for $rust {
            fn into_variable(self) -> Variable {
                Variable::new(Object::from($prim($func(self))))
            }
        }
    };
}

primitive_into_variable! {u8, Primitive::Integer, BigInt::from }
primitive_into_variable! {u16, Primitive::Integer, BigInt::from }
primitive_into_variable! {u32, Primitive::Integer, BigInt::from }
primitive_into_variable! {u64, Primitive::Integer, BigInt::from }
primitive_into_variable! {usize, Primitive::Integer, BigInt::from }

primitive_into_variable! {i8, Primitive::Integer, BigInt::from }
primitive_into_variable! {i16, Primitive::Integer, BigInt::from }
primitive_into_variable! {i32, Primitive::Integer, BigInt::from }
primitive_into_variable! {i64, Primitive::Integer, BigInt::from }
primitive_into_variable! {isize, Primitive::Integer, BigInt::from }

primitive_into_variable!(&str, Primitive::String, String::from);
primitive_into_variable!(String, Primitive::String);
primitive_into_variable!(&String, Primitive::String, String::clone);

impl TryFrom<Variable> for i8 {
    type Error = Exception;

    fn try_from(value: Variable) -> Result<Self, Self::Error> {
        let object = value.get_object();
        let object = object.get();
        if let Primitive::Integer(i) = object.as_primitive() {
            BigInt::to_i8(&i).ok_or(Exception::new(value))
        } else {
            Err(Exception::new(value))
        }
    }
}

impl TryFrom<Variable> for i16 {
    type Error = Exception;

    fn try_from(value: Variable) -> Result<Self, Self::Error> {
        let object = value.get_object();
        let object = object.get();
        if let Primitive::Integer(i) = object.as_primitive() {
            BigInt::to_i16(&i).ok_or(Exception::new(value))
        } else {
            Err(Exception::new(value))
        }
    }
}

impl TryFrom<Variable> for i32 {
    type Error = Exception;

    fn try_from(value: Variable) -> Result<Self, Self::Error> {
        let object = value.get_object();
        let object = object.get();
        if let Primitive::Integer(i) = object.as_primitive() {
            BigInt::to_i32(&i).ok_or(Exception::new(value))
        } else {
            Err(Exception::new(value))
        }
    }
}

impl TryFrom<Variable> for i64 {
    type Error = Exception;

    fn try_from(value: Variable) -> Result<Self, Self::Error> {
        let object = value.get_object();
        let object = object.get();
        if let Primitive::Integer(i) = object.as_primitive() {
            BigInt::to_i64(&i).ok_or(Exception::new(value))
        } else {
            Err(Exception::new(value))
        }
    }
}

impl TryFrom<Variable> for isize {
    type Error = Exception;

    fn try_from(value: Variable) -> Result<Self, Self::Error> {
        let object = value.get_object();
        let object = object.get();
        if let Primitive::Integer(i) = object.as_primitive() {
            BigInt::to_isize(i).ok_or(Exception::new(value))
        } else {
            Err(Exception::new(value))
        }
    }
}

impl TryFrom<Variable> for String {
    type Error = Exception;

    fn try_from(value: Variable) -> Result<Self, Self::Error> {
        let object = value.get_object();
        let object = object.get();
        if let Primitive::String(s) = object.as_primitive() {
            Ok(s.clone())
        } else {
            Err(Exception::new(value))
        }
    }
}

impl TryFrom<Variable> for bool {
    type Error = Exception;

    fn try_from(value: Variable) -> Result<Self, Self::Error> {
        let object = value.get_object();
        let object = object.get();
        if let Primitive::Boolean(b) = object.as_primitive() {
            Ok(*b)
        } else {
            Err(Exception::new(value))
        }
    }
}

impl IntoVariable for Result<Variable, Exception> {
    fn into_variable(self) -> Variable {
        self.unwrap()
    }
}
