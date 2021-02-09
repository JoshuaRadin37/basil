use std::sync::{RwLock, Arc};
use crate::object::{Object, DeepClone};

#[derive(Debug, Clone)]
pub struct Variable {
    inner: Arc<RwLock<Object>>
}

impl Variable {

    /// Returns a clone of only the inner object of this variable
    pub fn inner_clone(&self) -> Variable {
        let read = self.inner.read().expect("Variable was poisoned");
        let o = read.clone();
        Variable::from(o)
    }

    /// Creates a deep clone of this variable
    pub fn deep_clone(&self) -> Variable {
        Variable::from(self.inner.read().expect("Item poisoned").deep_clone())
    }

    /// Gets the inner backing ARC
    pub(crate) fn to_inner(&self) -> &Arc<RwLock<Object>> {
        &self.inner
    }
}

pub trait IntoVariable {
    fn into_variable(self) -> Variable;
}

impl <V : Into<Variable>> IntoVariable for V {
    fn into_variable(self) -> Variable {
        self.into()
    }
}

impl From<Object> for Variable {
    fn from(o: Object) -> Self {
        Variable {
            inner: Arc::new(RwLock::new(o))
        }
    }
}