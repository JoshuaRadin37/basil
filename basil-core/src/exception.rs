use crate::object::Object;
use crate::variable::Variable;

#[derive(Debug)]
pub struct Exception(Variable);

impl Exception {
    pub fn new(inner: Variable) -> Self {
        Exception(inner)
    }
}

pub type ExpressionResult = Result<Variable, Exception>;