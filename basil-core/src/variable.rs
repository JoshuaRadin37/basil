use std::sync::{RwLock, Arc};
use crate::object::Object;

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

    pub fn deep_clone(&self) -> Variable {

    }
}

pub trait IntoVariable {
    fn into_variable(self) -> Variable;
}

impl <V : Into<Variable>> IntoVariable for Variable {
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