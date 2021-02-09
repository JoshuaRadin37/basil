use crate::variable::Variable;
use crate::code_block::CodeBlock;
use crate::expression::Expression;

pub enum Statement {
    Assignment(String, Variable),
    If {
        condition: Expression,
        block: CodeBlock,
        elifs: Vec<(Expression, CodeBlock)>,
        r#else: Option<CodeBlock>
    },
    While {
        condition: Expression,
        block: CodeBlock
    },
    Expression(Expression)
}