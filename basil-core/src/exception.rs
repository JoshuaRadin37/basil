use crate::object::Object;
use crate::primitive::Primitive;
use crate::variable::Variable;

#[derive(Debug)]
pub struct Exception(Variable);

impl Exception {
    pub fn new(inner: Variable) -> Self {
        Exception(inner)
    }

    pub fn inner(&self) -> &Variable {
        &self.0
    }
}

impl<T: Into<Primitive>> From<T> for Exception {
    fn from(ty: T) -> Self {
        let p = ty.into();
        Exception::new(Variable::new(Object::new(p)))
    }
}

pub type ExpressionResult = Result<Variable, Exception>;
