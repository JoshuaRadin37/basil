use crate::variable::{Variable, IntoVariable};
use std::collections::HashMap;
use crate::context::Context;
use crate::object::Object;

pub struct Expression {
    head: Variable,
    tail: Option<Box<ExpressionTail>>
}

impl Expression {
    pub fn new(head: Variable, tail: Option<ExpressionTail>) -> Self {
        Expression { head, tail: tail.map(Box::new) }
    }

    pub fn evaluate(&self, context: &mut Context) -> Variable {
        if let Some(tail) = self.tail {

        } else {

        }
    }
}

pub enum ExpressionTail {
    GetMember(String),
    CallMethod {
        positional: Vec<Variable>,
        named: HashMap<String, Variable>
    }
}
