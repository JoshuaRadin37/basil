use crate::statements::Statement;
use crate::context::Context;
use crate::object::Object;
use crate::variable::Variable;

pub struct CodeBlock {
    statements: Vec<Statement>
}

impl CodeBlock {
    pub fn new(statements: Vec<Statement>) -> Self {
        CodeBlock { statements }
    }

}


pub trait Executable {

    fn execute(&self, context: &mut Context) -> Variable;
}

impl Executable for CodeBlock {
    fn execute(&self, context: &mut Context) -> Variable {

    }
}