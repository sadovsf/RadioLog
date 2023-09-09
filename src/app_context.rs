use std::cell::RefCell;

use crate::{data::Data, actions::ActionProcessor, app_errors::AppError, database::Database};

pub struct AppContext<'a> {
    pub db :&'a RefCell<Database>,
    pub data :Data<'a>,
    pub actions :ActionProcessor
}

impl<'a> AppContext<'a> {
    pub fn new(db :&'a RefCell<Database>) -> Result<Self, AppError> {
        Ok(Self {
            db,
            data: Data::new(db)?,
            actions: ActionProcessor::default(),
        })
    }
}