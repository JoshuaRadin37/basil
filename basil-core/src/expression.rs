use std::collections::HashMap;


use crate::context::Context;
use crate::object::Object;
use crate::variable::{IntoVariable, Variable};

pub struct Expression {
    head: Variable,
    tail: Option<Box<ExpressionTail>>
}

impl Expression {
    pub fn new(head: Variable, tail: Option<ExpressionTail>) -> Self {
        Expression { head, tail: tail.map(Box::new) }
    }

}

pub enum ExpressionTail {
    GetMember(String),
    CallMethod {
        positional: Vec<Variable>,
        named: HashMap<String, Variable>
    }
}

/*
impl Executable for Expression {
    fn execute(&self, context: &mut Context) -> Variable {
        self.evaluate(context)
    }
}

 */
