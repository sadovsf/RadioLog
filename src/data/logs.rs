use std::time::{SystemTime, UNIX_EPOCH};

use tui::widgets::ListItem;
use turbosql::Turbosql;

use super::{Position, data_store::DataStoreTrait};

#[derive(Turbosql, Clone, PartialEq)]
pub struct LogEntry {
    pub rowid: Option<i64>,
    pub long: Option<f64>,
    pub lat: Option<f64>,
    pub time: Option<u32>,
    pub name: Option<String>,
}

impl DataStoreTrait for LogEntry {
    fn set_id(&mut self, id: i64) {
        self.rowid = Some(id);
    }

    fn get_id(&self) -> i64 {
        self.rowid.expect("LogEntry.rowid is None")
    }
}

impl LogEntry {
    pub fn position(&self) -> Option<Position> {
        Some(Position::new(
            self.lat?,
            self.long?,
        ))
    }
}


impl Default for LogEntry {
    fn default() -> Self {
        LogEntry {
            rowid: None,
            long: None,
            lat: None,
            time: Some(SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backward").as_secs() as u32),
            name: Some("New log".to_string()),
        }
    }
}

impl<'a> Into<ListItem<'a>> for LogEntry {
    fn into(self) -> ListItem<'a> {
        ListItem::new(self.name.expect("LogEntry.name is None"))
    }
}
