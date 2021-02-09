use crate::code_block::CodeBlock;
use crate::context::Context;
use crate::expression::Expression;
use crate::function::Function;
use crate::object::Object;
use crate::statements::Statement;
use crate::variable::Variable;
use std::sync::RwLock;

pub mod code_block;
pub mod context;
pub mod dictionary;
pub mod exception;
pub mod expression;
pub mod function;
pub mod object;
pub mod primitive;
pub mod statements;
pub mod variable;

pub trait Executable {
    fn execute<E>(&self, context: &mut Context, executor: E) -> Variable;
}

pub trait Executor<T> {
    fn run(&mut self, context: &mut Context, runnable: &T) -> Object;
}

pub trait FullExecutor:
    Executor<Expression> + Executor<Statement> + Executor<CodeBlock> + Executor<Function>
{
}
