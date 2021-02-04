use std::collections::HashMap;
use crate::variable::Variable;

#[derive(Default, Debug)]
pub struct Context {
    variables: HashMap<String, Variable>
}