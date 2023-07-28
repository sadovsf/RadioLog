use crate::{data::Data, actions::ActionProcessor};

#[derive(Default)]
pub struct AppContext {
    pub data :Data,
    pub actions :ActionProcessor
    ,
}