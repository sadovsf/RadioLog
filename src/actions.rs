use std::slice::Iter;

use crossterm::event::Event;

use crate::{data::LogEntry, traits::{UIEvents, EventResult, UIElement}, ui_handler::UIHandler};




#[derive(Clone, PartialEq)]
pub enum Actions {
    DeleteLog(i64),
    CreateLog(LogEntry),
    ShowError(String),

    CreateLogWanted,
    FocusLog(Option<i64>),
    EditLog(i64)
}


#[derive(Default, Clone)]
pub struct ActionProcessor {
    pending :Vec<Actions>
}

impl ActionProcessor {
    pub fn add(&mut self, action :Actions) {
        self.pending.push(action);
    }

    pub fn iter(&self) -> Iter<Actions> {
        self.pending.iter()
    }

    pub fn clear(&mut self) {
        self.pending.clear();
    }
}