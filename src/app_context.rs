use crate::{data::Data, actions::ActionProcessor, app_errors::AppError};

pub struct AppContext {
    pub data :Data,
    pub actions :ActionProcessor
    ,
}

impl AppContext {
    pub fn new() -> Result<Self, AppError> {
        Ok(Self {
            data: Data::new()?,
            actions: ActionProcessor::default(),
        })
    }
}