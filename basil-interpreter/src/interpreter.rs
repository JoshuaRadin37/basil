use basil_core::expression::Expression;
use basil_core::object::Object;
use basil_core::Executor;
use std::task::Context;

pub struct Interpreter<'a> {
    executor: InterpreterExecutor,
    context: Context<'a>,
}

struct InterpreterExecutor;

impl Executor<Expression> for InterpreterExecutor {
    fn run(&mut self, context: &mut Context<'_>, runnable: &Expression) -> Object {}
}
