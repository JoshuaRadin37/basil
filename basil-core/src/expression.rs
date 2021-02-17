use std::collections::HashMap;

use crate::object::Object;
use crate::variable::{IntoVariable, Variable};


#[derive(Debug, Clone)]
pub enum Atom {
    Identifier(String),
    Variable(Variable)
}

#[derive(Debug, Clone)]
pub struct Expression {
    head: Atom,
    tail: Option<Box<ExpressionTail>>,
}

impl Expression {
    pub fn new(head: Atom, tail: Option<ExpressionTail>) -> Self {
        Expression {
            head,
            tail: tail.map(Box::new),
        }
    }

    pub fn head(&self) -> &Atom {
        &self.head
    }

    pub fn tail(&self) -> Option<&ExpressionTail> {
        match &self.tail {
            None => None,
            Some(t) => Some(&*t),
        }
    }

    pub fn head_mut(&mut self) -> &mut Atom {
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
        positional: Vec<Expression>,
        named: HashMap<String, Expression>,
    },
}

/*
impl Executable for Expression {
    fn execute(&self, context: &mut Context) -> Variable {
        self.evaluate(context)
    }
}

 */
