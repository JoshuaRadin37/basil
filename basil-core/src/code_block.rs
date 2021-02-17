use crate::object::Object;
use crate::span::WithSpan;
use crate::statements::Statement;
use crate::variable::Variable;

#[derive(Debug, Clone)]
pub struct CodeBlock {
    statements: Vec<WithSpan<Statement>>,
}

impl CodeBlock {
    pub fn new(statements: Vec<WithSpan<Statement>>) -> Self {
        CodeBlock { statements }
    }

    pub fn no_span<I: IntoIterator<Item = Statement>>(iter: I) -> Self {
        CodeBlock {
            statements: iter.into_iter().map(|s| WithSpan::empty(s)).collect(),
        }
    }

    pub fn statements(&self) -> &Vec<WithSpan<Statement>> {
        &self.statements
    }
}
