use std::collections::HashMap;
use crate::object::Object;
use crate::variable::Variable;

#[derive(Debug, Clone)]
pub struct Function {
    captures: HashMap<String, Variable>,
    inputs: Vec<String>
}