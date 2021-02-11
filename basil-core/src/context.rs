use crate::dictionary::{Dictionary, IntoDictionary};
use crate::object::Object;
use crate::variable::Variable;
use crate::{Executor, FullExecutor};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, RwLock};

#[derive(Default, Debug)]
pub struct Context {
    parent: Option<Context>,
    variables: Dictionary,
    // _phantom: &'c PhantomData<()>
}

impl Context {
    pub fn base() -> Self {
        let mut ret = Context::default();
        // basic_print function

        ret
    }

    pub fn higher_scope(self) -> Context {
        Context {
            parent: Some(self),
            variables: Default::default(),
            // _phantom: &Default::default()
        }
    }

    pub fn push(self, mut other: Context) -> Context {
        other.parent = Some(self);
        other
    }

    pub fn pop(self) -> Option<Self> {
        let Context { parent, .. } = self;
        parent
    }

    /*
    pub fn get<E : FullExecutor>(&self, id: &str, executor: &mut E) -> Option<&Variable> {
        unsafe {
            let mut mutable_self = &mut *(self as *const Self as *mut Self);
            match self.variables.get(&id.to_string(), ) {
                None => match &self.parent {
                    None => None,
                    Some(parent) => parent.get(id),
                },
                Some(ret) => Some(ret),
            }
        }
    }

     */

    pub fn get_mut<E: FullExecutor>(
        &mut self,
        id: &str,
        executor: &mut E,
    ) -> Option<&mut Variable> {
        match self
            .variables
            .get_mut(&mut id.to_string().into(), self, executor)
        {
            None => match &mut self.parent {
                None => None,
                Some(parent) => parent.get_mut(id, executor),
            },
            Some(ret) => Some(ret),
        }
    }

    pub fn insert<E: FullExecutor>(&mut self, id: &str, value: Variable, executor: &mut E) {
        self.variables
            .insert(id.to_string().into(), value, self, executor);
    }

    pub fn from_mapping<'a, E: FullExecutor, D: IntoDictionary<(&'a Object, &'a Variable)>>(
        mapping: &D,
        executor: &'a mut E,
    ) -> Self {
        let mut parent = Context::base();
        Context {
            parent: None,
            variables: mapping.into_dictionary(&mut parent, executor),
            // _phantom: &Default::default()
        }
    }
}
