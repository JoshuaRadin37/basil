use crate::function::Function;
use crate::object::{DeepClone, Object};
use crate::primitive::Primitive;
use crate::variable::{IntoVariable, Variable};
use std::borrow::Borrow;
use std::cell::{Ref, RefCell};
use std::collections::hash_map::IterMut;
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::iter::FromIterator;

#[derive(Debug, Clone, Default)]
pub struct Dictionary {
    values: HashMap<u64, Vec<Variable>>,
    keys: HashMap<u64, Vec<RefCell<Object>>>,
}

impl Dictionary {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_entries<S: AsRef<str>, I: IntoIterator<Item = S>>(entries: I) -> Self {
        let mut ret = Dictionary::new();
        for entry in entries {
            let name = entry.as_ref();
            let object = Object::from(name);
            ret.insert(
                object,
                Primitive::None.into_variable(),
                Object::basic_hash,
                Object::basic_eq,
            );
        }
        ret
    }

    pub fn insert<Hash: FnMut(&mut Object) -> u64, Eq: FnMut(&mut Object, &mut Object) -> bool>(
        &mut self,
        mut key: Object,
        value: Variable,
        mut hash: Hash,
        mut eq: Eq,
    ) {
        let hash_value = hash(&mut key);
        let mut keys = self.keys.entry(hash_value).or_default();
        let mut values = self.values.entry(hash_value).or_default();
        for (index, key2) in keys.iter_mut().enumerate() {
            let key2 = &mut *key2.borrow_mut();
            if eq(&mut key, key2) {
                values[index] = value;
                return;
            }
        }
        keys.push(RefCell::new(key));
        values.push(value);
    }

    pub fn get<Hash: FnMut(&mut Object) -> u64, Eq: FnMut(&mut Object, &mut Object) -> bool>(
        &self,
        key: &mut Object,
        mut hash: Hash,
        mut eq: Eq,
    ) -> Option<&Variable> {
        let hash_value = hash(key);
        let keys = self.keys.get(&hash_value)?;
        let values = self.values.get(&hash_value)?;
        for (index, key2) in keys.iter().enumerate() {
            let key2 = &mut *key2.borrow_mut();
            if eq(key, key2) {
                return Some(&values[index]);
            }
        }
        None
    }

    pub fn get_mut<Hash: FnMut(&mut Object) -> u64, Eq: FnMut(&mut Object, &mut Object) -> bool>(
        &mut self,
        key: &mut Object,
        mut hash: Hash,
        mut eq: Eq,
    ) -> Option<&mut Variable> {
        let hash_value = hash(key);
        let keys = self.keys.get(&hash_value)?;
        let values = self.values.get_mut(&hash_value)?;
        for (index, key2) in keys.iter().enumerate() {
            let key2 = &mut *key2.borrow_mut();
            if eq(key, key2) {
                return Some(&mut values[index]);
            }
        }
        None
    }

    pub fn remove<Hash: FnMut(&mut Object) -> u64, Eq: FnMut(&mut Object, &mut Object) -> bool>(
        &mut self,
        key: &mut Object,
        mut hash: Hash,
        mut eq: Eq,
    ) -> Option<Variable> {
        let hash_value = hash(key);
        let keys = self.keys.get_mut(&hash_value)?;

        let mut found_index = None;
        for (index, key2) in keys.iter().enumerate() {
            let key2 = &mut *key2.borrow_mut();
            if eq(key, key2) {
                found_index = Some(index);
                break;
            }
        }
        if let Some(index) = found_index {
            let values = self.values.get_mut(&hash_value)?;
            keys.remove(index);
            Some(values.remove(index))
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> DictionaryIterator {
        self.into_iter()
    }

    /*
    pub fn iter_mut(&mut self) -> MutDictionaryIterator {
        self.into_iter()
    }

     */
}

pub struct DictionaryIterator<'a> {
    buffer: Vec<(Ref<'a, Object>, &'a Variable)>,
}

impl<'a> DictionaryIterator<'a> {
    fn new(buffer: Vec<(Ref<'a, Object>, &'a Variable)>) -> Self {
        DictionaryIterator { buffer }
    }
}

impl<'a> Iterator for DictionaryIterator<'a> {
    type Item = (Ref<'a, Object>, &'a Variable);

    fn next(&mut self) -> Option<Self::Item> {
        self.buffer.pop()
    }
}

impl<'a> IntoIterator for &'a Dictionary {
    type Item = (Ref<'a, Object>, &'a Variable);
    type IntoIter = DictionaryIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let mut ret = Vec::new();
        for (key, value) in &self.values {
            let real_key = self.keys.get(key).unwrap();
            for (key, value) in real_key.iter().zip(value) {
                let key = key.borrow();
                ret.push((key, value))
            }
        }
        DictionaryIterator::new(ret)
    }
}

/*
pub struct MutDictionaryIterator<'a> {
    dict: &'a mut Dictionary,
    hashes: Vec<u64>,
    last_hash: Option<u64>,
    count: usize,
}

impl<'a> MutDictionaryIterator<'a> {
    fn new(dict: &'a mut Dictionary) -> Self {
        let vec = dict
            .keys
            .keys()
            .map(|m| {
                let keys = dict.keys.get(m).unwrap().len();
                vec![*m; keys]
            })
            .flatten()
            .collect();
        MutDictionaryIterator {
            dict,
            hashes: vec,
            last_hash: None,
            count: 0,
        }
    }
}

impl<'a> Iterator for MutDictionaryIterator<'a> {
    type Item = (&'a mut Object, &'a mut Variable);

    fn next(&mut self) -> Option<Self::Item> {
        let next_hash = self.hashes.pop()?;
        unsafe {
            let keys = &mut *(self.dict.keys.get_mut(&next_hash)? as *mut Vec<RefCell<Object>>);
            let values = &mut *(self.dict.values.get_mut(&next_hash)? as *mut Vec<Variable>);
            match self.last_hash {
                None => {
                    self.last_hash = Some(next_hash);
                }
                Some(last_hash) => {
                    if next_hash != last_hash {
                        self.count = 0;
                    }
                }
            }
            let key = &mut *keys.get(self.count)?.borrow_mut();
            let value = values.get_mut(self.count)?;
            self.count += 1;
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

 */

impl DeepClone for Dictionary {
    fn deep_clone(&self) -> Self {
        let mut ret = Dictionary::new();
        for hash in self.keys.keys() {
            let keys = &self.keys[hash];
            let values = &self.values[hash];
            let mut zipped = keys.iter().zip(values);

            let ret_keys = ret.keys.entry(*hash).or_default();
            let ret_values = ret.values.entry(*hash).or_default();
            for (key, value) in zipped {
                let key = key.borrow().deep_clone();
                let value = value.deep_clone();
                ret_keys.push(RefCell::new(key));
                ret_values.push(value);
            }
        }
        ret
    }
}
impl IntoVariable for Dictionary {
    fn into_variable(self) -> Variable {
        Primitive::Dictionary(self).into_variable()
    }
}
