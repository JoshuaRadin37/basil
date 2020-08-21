use crate::common_types::{Dimension, BasilResult, BasilError};
use crate::external_structures::group::Group;
use std::sync::{Arc, RwLock, PoisonError, RwLockWriteGuard};
use crate::external_structures::application::Application;
use crate::rendering::two_dimensional::Graphics2D;

pub enum ActionOnWindowClose {
    QuitApplication,
    CloseOnlyWindow,
    CloseGroup,
    Minimize,
}


pub struct Window {
    current_size: Dimension,
    show: bool,
    on_close: ActionOnWindowClose
}

impl Window {

    pub async fn perform_close_option(&self, app: Arc<RwLock<Application>>) -> BasilResult<()> {

        match self.on_close {
            ActionOnWindowClose::QuitApplication => {
                let guard = app.write();
                match guard {
                    Ok(mut app) => {
                        return Ok(app.quit(0));
                    },
                    Err(e) => {
                        return Err(BasilError::CloseOperationFailed);
                    },
                }

            },
            ActionOnWindowClose::CloseOnlyWindow => {},
            ActionOnWindowClose::CloseGroup => {},
            ActionOnWindowClose::Minimize => {},
        }


        Ok(())
    }

    pub fn get_renderer(&self) -> &mut dyn Graphics2D {
        unimplemented!()
    }
}