extern crate queues;
use queues::*;

use crate::data::LogEntry;




#[derive(Clone, PartialEq)]
pub enum Actions {
    DeleteLog(i64),
    CreateLog(LogEntry),
    ShowError(String),
}



pub struct ActionProcessor {
    pending :Queue<Actions>
}

impl Default for ActionProcessor {
    fn default() -> Self {
        Self {
            pending: Queue::new()
        }
    }
}

type Result<'a, T> = std::result::Result<T, &'a str>;

impl ActionProcessor {
    pub fn add(&mut self, action :Actions) {
        let res = self.pending.add(action);
        if res.is_err() {
            panic!("Error adding action to queue: {}", res.err().unwrap());
        }
    }

    pub fn peek(&self) -> Result<Actions> {
        self.pending.peek()
    }

    pub fn consume(&mut self) -> Result<Actions> {
        self.pending.remove()
    }
}