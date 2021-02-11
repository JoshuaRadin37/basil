use std::rc::Rc;
use std::sync::RwLock;
use std::marker::PhantomData;

/// Context structs
#[derive(Clone)]
pub struct Context<'c> {
    inner: Rc<RwLock<InnerContext<'c>>>,
    _lifetime: &'c PhantomData<()>
}

impl<'c> Context<'c> {

    pub fn new() -> Self {
        Self {
            inner: Rc::new(Default::default()),
            _lifetime: &Default::default()
        }
    }
}


#[derive(Debug, Default)]
struct InnerContext<'c> {
    parent: Option<Context<'c>>,
    data: ()
}

