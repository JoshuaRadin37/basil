use std::cell::RefCell;
use std::rc::Rc;

use crate::exception::Exception;
use crate::object::{DeepClone, Object};
use crate::primitive::Primitive;
use crate::ptr::Ptr;

#[derive(Debug, Clone)]
pub struct Variable {
    inner: Ptr<Ptr<Object>>,
}

impl Variable {
    /// Returns a clone of only the inner object of this variable
    pub fn inner_clone(&self) -> Variable {
        let read = self
            .inner
            .try_get()
            .expect("Variable already mutably borrowed");
        let o = read.get().clone();
        Variable::from(o)
    }

    /// Creates a deep clone of this variable
    pub fn deep_clone(&self) -> Variable {
        Variable::from(
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

    pub fn set_object<P: Into<Ptr<Object>>>(&mut self, object_ptr: P) {
        let ptr = object_ptr.into();
        self.inner = Ptr::new(ptr);
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

impl<V: Into<Variable>> IntoVariable for V {
    fn into_variable(self) -> Variable {
        self.into()
    }
}

/*
impl From<Object> for Variable {
    fn from(o: Object) -> Self {
        Variable {
            inner: Arc::new(RwLock::new(o)),
        }
    }
}

 */

/*
impl<I: Into<Object>> From<I> for Variable {
    fn from(i: I) -> Self {
        Variable {
            inner: Rc::new(RefCell::new(i.into())),
        }
    }
}

 */

impl<I: Into<Object>> From<I> for Variable {
    fn from(i: I) -> Self {
        Variable {
            inner: Ptr::new(Ptr::new(i.into())),
        }
    }
}

impl IntoVariable for bool {
    fn into_variable(self) -> Variable {
        Primitive::Boolean(self).into_variable()
    }
}

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
