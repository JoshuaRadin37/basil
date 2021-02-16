use std::collections::HashMap;

use crate::object::Object;
use crate::variable::{IntoVariable, Variable};

#[derive(Debug, Clone)]
pub struct Expression {
    head: Variable,
    tail: Option<Box<ExpressionTail>>,
}

impl Expression {
    pub fn new(head: Variable, tail: Option<ExpressionTail>) -> Self {
        Expression {
            head,
            tail: tail.map(Box::new),
        }
    }

    pub fn head(&self) -> &Variable {
        &self.head
    }

    pub fn tail(&self) -> Option<&ExpressionTail> {
        match &self.tail {
            None => None,
            Some(t) => Some(&*t),
        }
    }

    pub fn head_mut(&mut self) -> &mut Variable {
        &mut self.head
    }

    pub fn tail_mut(&mut self) -> Option<&mut ExpressionTail> {
        match &mut self.tail {
            None => None,
            Some(t) => Some(&mut *t),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ExpressionTail {
    GetMember(String),
    CallMethod {
        positional: Vec<Variable>,
        named: HashMap<String, Variable>,
    },
}

/*
impl Executable for Expression {
    fn execute(&self, context: &mut Context) -> Variable {
        self.evaluate(context)
    }
}

 */
