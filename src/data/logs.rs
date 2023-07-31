use std::time::{SystemTime, UNIX_EPOCH};

use tui::widgets::ListItem;

use crate::database::{macros::define_table, SchemaStep, DBObjectSerializable, DBSchemaObject};

use super::{Position, data_store::DataStoreTrait};

#[derive(Debug, Clone, PartialEq)]
pub struct LogEntry {
    pub rowid: Option<i64>,
    pub long: Option<f64>,
    pub lat: Option<f64>,
    pub time: Option<u32>,
    pub name: Option<String>,
}


define_table!(LogEntry,
    SchemaStep::SQL(
        "CREATE TABLE LogEntry (
            id INTEGER PRIMARY KEY,
            long REAL,
            lat REAL,
            time UINT,
            name TEXT
        )"
    )
);

impl DBObjectSerializable for LogEntry {
    fn from_row(row :&rusqlite::Row) -> Result<Self, rusqlite::Error> {
        Ok(Self {
            rowid: row.get(0)?,
            long: row.get(1)?,
            lat: row.get(2)?,
            time: row.get(3)?,
            name: row.get(4)?,
        })
    }

    fn insert_row(&mut self, conn :&mut rusqlite::Connection) -> Result<(), rusqlite::Error> {
        conn.execute("INSERT INTO LogEntry (long, lat, time, name) VALUES (?1, ?2, ?3, ?4)", (&self.long, &self.lat, &self.time, &self.name))?;
        self.rowid = Some(conn.last_insert_rowid());
        Ok(())
    }

    fn update_row(&self, conn :&mut rusqlite::Connection) -> Result<(), rusqlite::Error> {
        conn.execute("UPDATE LogEntry SET long=?1, lat=?2, time=?3, name=?4 WHERE id=?5", (&self.long, &self.lat, &self.time, &self.name, &self.rowid))?;
        Ok(())
    }

    fn delete_row(&self, conn :&mut rusqlite::Connection) -> Result<(), rusqlite::Error> {
        conn.execute("DELETE FROM LogEntry WHERE id=?1", [&self.rowid])?;
        Ok(())
    }
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
