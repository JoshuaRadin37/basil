use std::collections::HashMap;
use crate::variable::Variable;
use std::sync::Arc;
use std::marker::PhantomData;
use crate::object::Object;

#[derive(Default, Debug)]
pub struct Context<'c> {
    parent: Option<Box<Context<'c>>>,
    variables: HashMap<String, Variable>,
    _phantom: &'c PhantomData<()>
}

impl<'c> Context<'c> {

    pub fn base() -> Self {
        let mut ret = Context::default();
        // basic_print function
        let basic_print =
            |context: &mut Context| -> bool {
                let string = context.get("0").unwrap();


                true
            };


        ret
    }

    pub fn higher_scope<'d : 'c>(self) -> Arc<Context<'d>> {
        Arc::new(Context {
            parent: Some(Box::new(self)),
            variables: Default::default(),
            _phantom: &Default::default()
        })
    }

    pub fn push<'d, 'e>(self, other: Context<'e>) -> Context<'d>
        where
            'd : 'c,
            'e : 'c,
            'd : 'e {

        let mut next = Context {
            parent: Some(Box::new(self)),
            variables: other.variables.clone(),
            _phantom: &Default::default()
        };

        next
    }


    pub fn pop(self) -> Option<Self> {
        let Context { parent, ..} = self;
        parent.map(|p| *p)
    }

    pub fn get(&self, id: &str) -> Option<&Variable> {
        match self.variables.get(&id.to_string()) {
            None => {
                match &self.parent {
                    None => { None }
                    Some(parent) => {
                        parent.get(id)
                    }
                }
            }
            Some(ret) => {
                Some(ret)
            }
        }
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut Variable> {
        match self.variables.get_mut(&id.to_string()) {
            None => {
                match &mut self.parent {
                    None => { None }
                    Some(parent) => {
                        parent.get_mut(id)
                    }
                }
            }
            Some(ret) => {
                Some(ret)
            }
        }
    }

    pub fn insert(&mut self, id: &str, value: Variable) {
        self.variables.insert(id.to_string(), value);
    }

    pub fn from_mapping(mapping: &HashMap<Object, Variable>) -> Self {
        Context {
            parent: None,
            variables: mapping.iter()
                .map(|(key, val)| (format!("{}", key.as_hashmap_string().expect("Can't turn this into a context variable")), val.clone()))
                .collect(),
            _phantom: &Default::default()
        }
    }

}

