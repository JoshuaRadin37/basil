use crate::context::Context;
use crate::function::Function;
use crate::object::Object;
use crate::variable::Variable;
use crate::{Executor, FullExecutor};
use std::collections::hash_map::IterMut;
use std::collections::{BinaryHeap, HashMap, VecDeque};

#[derive(Debug, Clone, Default)]
pub struct Dictionary {
    hashmap: HashMap<u64, Variable>,
    keys: HashMap<u64, Object>,
}

impl Dictionary {
    pub fn new() -> Self {
        Default::default()
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
        self.keys.insert(hash, key);
    }

    pub fn get<E: FullExecutor>(
        &self,
        key: &mut Object,
        context: Context,
        executor: &mut E,
    ) -> Option<&Variable> {
        let hash = key
            .get_hash(context, executor)
            .expect("Need a hash value for a key in an index");
        self.hashmap.get(&hash)
    }

    pub fn get_mut<E: FullExecutor>(
        &mut self,
        key: &mut Object,
        context: Context,
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
        context: &mut Context<'_>,
        executor: &mut E,
    ) -> Option<Variable> {
        let hash = key
            .get_hash(context, executor)
            .expect("Need a hash value for a key in an index");
        self.keys.remove(&hash);
        self.hashmap.remove(&hash)
    }

    pub fn len(&self) -> usize {
        self.hashmap.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> DictionaryIterator {
        self.into_iter()
    }

    pub fn iter_mut(&mut self) -> MutDictionaryIterator {
        self.into_iter()
    }
}

pub struct DictionaryIterator<'a> {
    buffer: Vec<(&'a Object, &'a Variable)>,
}

impl<'a> DictionaryIterator<'a> {
    fn new(buffer: Vec<(&'a Object, &'a Variable)>) -> Self {
        DictionaryIterator { buffer }
    }
}

impl<'a> Iterator for DictionaryIterator<'a> {
    type Item = (&'a Object, &'a Variable);

    fn next(&mut self) -> Option<Self::Item> {
        self.buffer.pop()
    }
}

impl<'a> IntoIterator for &'a Dictionary {
    type Item = (&'a Object, &'a Variable);
    type IntoIter = DictionaryIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let mut ret = Vec::new();
        for (key, value) in &self.hashmap {
            let real_key = self.keys.get(key).unwrap();
            ret.push((real_key, value))
        }
        DictionaryIterator::new(ret)
    }
}

pub trait IntoDictionary<KeyValue>: Iterator<Item = KeyValue> {
    fn into_dictionary<E: FullExecutor>(
        self,
        context: &mut Context,
        executor: &mut E,
    ) -> Dictionary;
}

impl<'a, I: Iterator<Item = (&'a Object, &'a Variable)>> IntoDictionary<(&'a Object, &'a Variable)>
    for I
{
    fn into_dictionary<E: FullExecutor>(
        mut self,
        context: &mut Context<'_>,
        executor: &mut E,
    ) -> Dictionary {
        let mut ret = Dictionary::new();
        for (key, value) in self {
            ret.insert(key.clone(), value.clone(), context, executor)
        }
        ret
    }
}

pub struct MutDictionaryIterator<'a> {
    dict: &'a mut Dictionary,
    hashes: Vec<u64>,
}

impl<'a> MutDictionaryIterator<'a> {
    fn new(dict: &'a mut Dictionary) -> Self {
        let vec = dict.keys.keys().map(|m| *m).collect();
        MutDictionaryIterator { dict, hashes: vec }
    }
}

impl<'a> Iterator for MutDictionaryIterator<'a> {
    type Item = (&'a mut Object, &'a mut Variable);

    fn next(&mut self) -> Option<Self::Item> {
        let next_hash = self.hashes.pop()?;
        unsafe {
            let key = &mut *(self.dict.keys.get_mut(&next_hash)? as *mut Object);
            let value = &mut *(self.dict.hashmap.get_mut(&next_hash)? as *mut Variable);
            Some((key, value))
        }
    }
}

impl<'a> IntoIterator for &'a mut Dictionary {
    type Item = (&'a mut Object, &'a mut Variable);
    type IntoIter = MutDictionaryIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        MutDictionaryIterator::new(self)
    }
}
