use std::time::{SystemTime, UNIX_EPOCH};

use tui::widgets::ListItem;
use turbosql::{select, Turbosql, execute};

mod config;
use config::ConfigData;

pub mod position;
use position::Position;

#[derive(Turbosql, Clone, PartialEq)]
pub struct LogEntry {
    pub rowid: Option<i64>,
    pub long: Option<f64>,
    pub lat: Option<f64>,
    pub time: Option<u32>,
    pub name: Option<String>,
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

impl LogEntry {
    pub fn position(&self) -> Option<Position> {
        if self.lat.is_none() || self.long.is_none() {
            return None;
        }

        Some(Position::new(
            self.lat.unwrap(),
            self.long.unwrap(),
        ))
    }
}


pub struct Data;

impl Data {
    pub fn get_logs() -> Vec<LogEntry> {
        match select!(Vec<LogEntry>) {
            Ok(logs) => logs,
            Err(e) => {
                eprintln!("Error: {}", e);
                vec![]
            }
        }
    }

    pub fn get_log(id :i64) -> Option<LogEntry> {
        select!(LogEntry "WHERE rowid = " id).ok()
    }

    pub fn insert_log(log :&LogEntry) -> Result<i64, turbosql::Error> {
        log.insert()
    }

    pub fn delete_log(log_id :i64) -> Result<(), turbosql::Error> {
        let result = execute!("DELETE FROM logentry WHERE rowid = ?", log_id);
        if result.is_err() {
            return Err(result.err().unwrap());
        }
        if result.unwrap() == 0 {
            return Err(turbosql::Error::OtherError("No rows deleted"));
        }
        Ok(())
    }

    pub fn get_config() -> ConfigData {
        ConfigData {
            own_position: Position::new(50.061520, 14.091540)
        }
    }
}