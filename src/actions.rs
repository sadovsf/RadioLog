use std::slice::Iter;
use crate::ui::AlertDialogStyle;




#[derive(Clone, PartialEq)]
pub enum Actions {
    DeleteLog(i64),
    DeleteRace(i64),

    ShowError(String),
    ShowConfirm(String, AlertDialogStyle, Box<Actions>),

    CreateLogWanted,
    FocusLog(Option<i64>),
    EditLog(i64),

    ToggleMap
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