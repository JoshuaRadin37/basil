use std::cell::RefCell;
use std::rc::Rc;

use crate::exception::Exception;
use crate::object::{DeepClone, Object};
use crate::primitive::Primitive;

#[derive(Debug, Clone)]
pub struct Variable {
    inner: Rc<RefCell<Object>>,
}

impl Variable {
    /// Returns a clone of only the inner object of this variable
    pub fn inner_clone(&self) -> Variable {
        let read = self
            .inner
            .try_borrow()
            .expect("Variable already mutably borrowed");
        let o = read.clone();
        Variable::from(o)
    }

    /// Creates a deep clone of this variable
    pub fn deep_clone(&self) -> Variable {
        Variable::from(
            self.inner
                .try_borrow()
                .expect("Variable already mutably borrowed")
                .deep_clone(),
        )
    }

    /// Gets the inner backing ARC
    pub fn to_inner(&self) -> &Rc<RefCell<Object>> {
        &self.inner
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
        let borrow = inner.borrow();
        let dictionary = borrow
            .get_dictionary()
            .ok_or(Exception::from(format!("{:?} is not a dictionary", borrow)))?;
        dictionary
            .get(&mut member, hash, eq)
            .ok_or(Exception::from(format!(
                "{:?} is not a member of {:?}",
                member, self
            )))
            .map(|reference| reference.clone())
    }

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
            inner: Rc::new(RefCell::new(i.into())),
        }
    }
}

impl IntoVariable for bool {
    fn into_variable(self) -> Variable {
        Primitive::Boolean(self).into_variable()
    }
}
