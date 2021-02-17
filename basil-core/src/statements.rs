use crate::code_block::CodeBlock;
use crate::expression::Expression;
use crate::variable::Variable;

#[derive(Debug, Clone)]
pub enum Statement {

    Assignment(Expression, Expression),
    If {
        condition: Expression,
        block: CodeBlock,
        elifs: Vec<(Expression, CodeBlock)>,
        r#else: Option<CodeBlock>,
    },
    While {
        condition: Expression,
        block: CodeBlock,
    },
    Expression(Expression),
    Return(Expression),
    Raise(Expression),
}
