use crate::object::Object;
use crate::statements::Statement;
use crate::variable::Variable;

#[derive(Debug, Clone)]
pub struct CodeBlock {
    statements: Vec<Statement>,
}

impl CodeBlock {
    pub fn new(statements: Vec<Statement>) -> Self {
        CodeBlock { statements }
    }

    pub fn statements(&self) -> &Vec<Statement> {
        &self.statements
    }
}
