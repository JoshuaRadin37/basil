use crate::code_block::CodeBlock;
use crate::object::Object;
use crate::span::WithSpan;
use crate::variable::Variable;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicUsize, Ordering};

static FUNCTION_COUNT: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone)]
pub struct Function {
    id: usize,
    captures: HashMap<String, Variable>,
    positional_arguments: Vec<String>,
    keyword_arguments: Vec<(String, Object)>,
    code_block: CodeBlock,
}

impl Function {
    pub fn new(
        captures: HashMap<String, Variable>,
        positional_arguments: Vec<String>,
        keyword_arguments: Vec<(String, Object)>,
        code_block: CodeBlock,
    ) -> Self {
        let id = FUNCTION_COUNT.fetch_add(1, Ordering::Acquire);
        Function {
            id,
            captures,
            positional_arguments,
            keyword_arguments,
            code_block,
        }
    }

    pub fn empty_span(self) -> WithSpan<Self> {
        WithSpan::empty(self)
    }
}

impl Function {
    pub fn id(&self) -> usize {
        self.id
    }

    pub fn captures(&self) -> &HashMap<String, Variable> {
        &self.captures
    }
    pub fn positional_arguments(&self) -> &Vec<String> {
        &self.positional_arguments
    }
    pub fn keyword_arguments(&self) -> &Vec<(String, Object)> {
        &self.keyword_arguments
    }

    pub fn code_block(&self) -> &CodeBlock {
        &self.code_block
    }
}
