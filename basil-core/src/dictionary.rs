use crate::context::Context;
use crate::function::Function;
use crate::object::Object;
use crate::variable::Variable;
use crate::{Executor, FullExecutor};
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct Dictionary {
    hashmap: HashMap<u64, Variable>,
}

impl Dictionary {
    pub fn new() -> Self {
        Dictionary {
            hashmap: Default::default(),
        }
    }

    pub fn insert<E: FullExecutor>(
        &mut self,
        mut key: Object,
        value: Variable,
        context: &mut Context,
        executor: &mut E,
    ) {
        let hash = key
            .get_hash(context, executor)
            .expect("Need a hash value for a key in an index");
        self.hashmap.insert(hash, value);
    }

    pub fn get<E: FullExecutor>(&self, key: &mut Object, executor: &mut E) -> Option<&Variable> {
        let hash = key
            .get_hash(context, executor)
            .expect("Need a hash value for a key in an index");
        self.hashmap.get(&hash)
    }

    pub fn get_mut<E: FullExecutor>(
        &mut self,
        key: &mut Object,
        executor: &mut E,
    ) -> Option<&mut Variable> {
        let hash = key
            .get_hash(context, executor)
            .expect("Need a hash value for a key in an index");
        self.hashmap.get_mut(&hash)
    }

    pub fn remove<E: FullExecutor>(
        &mut self,
        key: &mut Object,
        executor: &mut E,
    ) -> Option<Variable> {
        let hash = key
            .get_hash(context, executor)
            .expect("Need a hash value for a key in an index");
        self.hashmap.remove(&hash)
    }
}
