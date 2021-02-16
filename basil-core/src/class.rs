use crate::object::{DeepClone, Object};
use crate::type_id::Explicit;
use std::cell::RefCell;
use std::sync::atomic::{AtomicU64, Ordering};

static CLASS_ID: AtomicU64 = AtomicU64::new(0);

#[derive(Debug)]
pub struct Class {
    parents: Vec<Explicit>,
    id: Explicit,
    created: RefCell<bool>,
    definitions: Vec<(String, Object)>,
}

impl Class {
    pub fn new(parents: Vec<Explicit>, definitions: Vec<(String, Object)>) -> Self {
        let id = CLASS_ID.fetch_add(1, Ordering::Acquire);
        Class {
            parents,
            id: Explicit::new(id),
            created: RefCell::new(false),
            definitions,
        }
    }

    pub fn set_created(&self) {
        if !*self.created.borrow() {
            *self.created.borrow_mut() = true;
        }
    }

    pub fn created(&self) -> bool {
        *self.created.borrow()
    }

    pub fn parents(&self) -> &Vec<Explicit> {
        &self.parents
    }
    pub fn id(&self) -> Explicit {
        self.id
    }
    pub fn definitions(&self) -> &Vec<(String, Object)> {
        &self.definitions
    }
}

impl Clone for Class {
    fn clone(&self) -> Self {
        let id = CLASS_ID.fetch_add(1, Ordering::Acquire);
        Self {
            parents: self.parents.clone(),
            id: Explicit::new(id),
            created: RefCell::new(false),
            definitions: self.definitions.clone(),
        }
    }
}

impl DeepClone for Class {
    fn deep_clone(&self) -> Self {
        self.clone()
    }
}
