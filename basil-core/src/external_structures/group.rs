use crate::external_structures::application::Application;
use std::sync::{RwLock, Arc};

pub struct Group {
    app: Arc<RwLock<Application>>
}

impl Group {

    pub fn get_application(&self) -> &Arc<RwLock<Application>> {
        &self.app
    }
}