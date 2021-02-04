use crate::variable::Variable;
use std::collections::HashMap;

pub struct Expression {
    head: Variable,
    tail: Option<Box<ExpressionTail>>
}

pub enum ExpressionTail {
    GetMember(String),
    CallMethod {
        positional: Vec<Variable>,
        named: HashMap<String, Variable>
    }
}

