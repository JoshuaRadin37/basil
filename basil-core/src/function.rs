use std::collections::HashMap;
use crate::object::Object;
use crate::variable::Variable;
use std::sync::atomic::AtomicUsize;
use std::hash::{Hash, Hasher};
use crate::code_block::{CodeBlock};
use crate::context::Context;

static FUNCTION_COUNT: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone)]
pub struct Function {
    id: usize,
    captures: HashMap<String, Variable>,
    inputs: Vec<String>,
}

impl Function {
    pub fn id(&self) -> usize {
        self.id
    }
}

pub enum InnerFunction {
    Basil(CodeBlock),
    Rust(Box<dyn Fn(&mut Context) -> bool>)
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Function {

}

impl Hash for Function {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}
/*
impl Executable for Function {
    fn execute(&self, context: &mut Context<'c>) -> Variable {
        unimplemented!()
    }
}

 */


