use crate::{data::Data, actions::ActionProcessor};

pub struct AppContext {
    pub data :Data,
    pub actions :ActionProcessor
    ,
}

impl AppContext {
    pub fn new() -> Result<Self, rusqlite::Error> {
        Ok(Self {
            data: Data::new()?,
            actions: ActionProcessor::default(),
        })
    }
}