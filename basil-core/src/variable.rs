use std::sync::{RwLock, Arc};
use crate::object::Object;

pub struct Variable {
    inner: Arc<RwLock<Object>>
}

pub trait IntoVariable {
    fn into_variable(self) -> Variable;
}

impl <V : Into<Variable>> IntoVariable for Variable {
    fn into_variable(self) -> Variable {
        self.into()
    }
}