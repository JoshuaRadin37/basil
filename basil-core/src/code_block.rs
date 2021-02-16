use crate::object::Object;
use crate::statements::Statement;
use crate::variable::Variable;
use basil_frontend::span::WithSpan;

#[derive(Debug, Clone)]
pub struct CodeBlock {
    statements: Vec<WithSpan<Statement>>,
}

impl CodeBlock {
    pub fn new(statements: Vec<WithSpan<Statement>>) -> Self {
        CodeBlock { statements }
    }

    pub fn statements(&self) -> &Vec<WithSpan<Statement>> {
        &self.statements
    }
}
