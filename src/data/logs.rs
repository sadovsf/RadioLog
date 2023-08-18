use std::{time::{SystemTime, UNIX_EPOCH}};

use ratatui::{widgets::{ListItem, Cell, Row}, style::{Style, Color}};

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

static HEADER_CELLS: [&str; 3] = [" Name ", " Time ", " QTH "];
impl LogEntry {
    pub fn table_header() -> Row<'static> {
        Row::new(HEADER_CELLS.iter()
            .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)))
        )
    }

    pub fn position(&self) -> Option<Position> {
        Some(Position::new(
            self.lat?,
            self.long?,
        ))
    }

    pub fn set_position(&mut self, pos :Position) {
        self.lat = Some(pos.latitude);
        self.long = Some(pos.longitude);
    }
}


impl From<&LogEntry> for Row<'_> {
    fn from(value: &LogEntry) -> Self {
        if value.name.is_none() || value.time.is_none() {
            return Row::new(vec!{Cell::from("Invalid data")});
        }

        let cell_time = chrono::NaiveDateTime::from_timestamp_opt(value.time.unwrap().into(), 0);
        let cells = [
            Cell::from(value.name.as_ref().unwrap().clone()),
            cell_time.map_or(Cell::from("Invalid format"), |t| Cell::from(t.to_string())),
            value.position().map_or(Cell::from("Missing"), |p| Cell::from(p.to_qth())),
        ];
        Row::new(cells).height(1)
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
