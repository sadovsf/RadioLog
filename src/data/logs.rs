use std::time::{SystemTime, UNIX_EPOCH};
use ratatui::{widgets::{ListItem, Cell, Row}, style::{Style, Color}, prelude::Constraint};
use crate::{database::{macros::define_table, SchemaStep, DBObjectSerializable, DBSchemaObject}, app_context::AppContext};
use super::{Position, data_store::DataStoreTrait};
use rusqlite::Connection;

fn update_1(conn :&mut Connection) -> Result<(), rusqlite::Error> {
    let stmt = conn.prepare("SELECT * FROM LogEntry");
    if stmt.is_err() {
        panic!("Failed to prepare statement: {}", stmt.err().unwrap());
    }
    let mut stmt = stmt.unwrap();
    let translated_logs = stmt.query_map((), |row| {
        let id :i64 = row.get(0)?;
        let long :f64 = row.get(1)?;
        let lat :f64 = row.get(2)?;
        let time :u32 = row.get(3)?;
        let call :String = row.get(4)?;

        Ok(LogEntry {
            rowid: Some(id),
            time: Some(time),
            call: Some(call),
            code: None,
            locator: Some(Position::new(lat, long).to_qth()),
            race_id: None,
        })
    })?;

    let mut unpacked_logs = vec!();
    for log in translated_logs {
        if log.is_err() {
            panic!("Failed to translate log: {}", log.err().unwrap());
        }

        let log = log.unwrap();
        unpacked_logs.push(log);
    }
    drop(stmt);

    let tx = conn.transaction().expect("Failed to start update transaction");
    tx.execute_batch("
        ALTER TABLE LogEntry DROP COLUMN long;
        ALTER TABLE LogEntry DROP COLUMN lat;
        ALTER TABLE LogEntry ADD COLUMN code TEXT;
        ALTER TABLE LogEntry ADD COLUMN locator TEXT;
    ")?;
    tx.commit()?;

    for log in unpacked_logs {
        log.update_row(conn).expect("Failed to update row");
    }

    Ok(())
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
    ),
    SchemaStep::SQL(
        "ALTER TABLE LogEntry RENAME COLUMN name TO call"
    ),
    SchemaStep::FN( &|conn :&mut Connection| update_1(conn) ),
    SchemaStep::SQL(
        "ALTER TABLE LogEntry ADD COLUMN race_id INTEGER"
    ),
);

#[derive(Debug, Clone, PartialEq)]
pub struct LogEntry {
    pub rowid: Option<i64>,
    pub time: Option<u32>,
    pub call: Option<String>,
    pub code: Option<String>,
    pub locator: Option<String>,
    pub race_id: Option<i64>,
}

impl DBObjectSerializable for LogEntry {
    fn from_row(row :&rusqlite::Row) -> Result<Self, rusqlite::Error> {
        Ok(Self {
            rowid: row.get(0)?,
            time: row.get(1)?,
            call: row.get(2)?,
            code: row.get(3)?,
            locator: row.get(4)?,
            race_id: row.get(5)?,
        })
    }

    fn insert_row(&mut self, conn :&mut rusqlite::Connection) -> Result<(), rusqlite::Error> {
        conn.execute(
            "INSERT INTO LogEntry (time, call, code, locator, race_id) VALUES (?1, ?2, ?3, ?4, ?5)",
            (&self.time, &self.call, &self.code, &self.locator, &self.race_id)
        )?;
        self.rowid = Some(conn.last_insert_rowid());
        Ok(())
    }

    fn update_row(&self, conn :&mut rusqlite::Connection) -> Result<(), rusqlite::Error> {
        conn.execute(
            "UPDATE LogEntry SET time=?1, call=?2, code=?3, locator=?4, race_id=?5 WHERE id=?6",
            (&self.time, &self.call, &self.code, &self.locator, &self.race_id, &self.rowid)
        )?;
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

static HEADER_CELLS: [&str; 7] = [" Time ", " Call ", " ID ", " Code ", " QTH ", " Dst(km) ", " Azim "];
impl LogEntry {
    pub fn table_header() -> Row<'static> {
        Row::new(HEADER_CELLS.iter()
            .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)))
        )
    }

    pub fn table_column_constraints() -> &'static [Constraint; 7] {
        &[
            Constraint::Min(22),
            Constraint::Min(30),
            Constraint::Min(5),
            Constraint::Min(9),
            Constraint::Min(7),
            Constraint::Min(10),
            Constraint::Min(8),
        ]
    }

    pub fn table_row(&self, app_ctx :& AppContext) -> Row {
        if self.call.is_none() || self.time.is_none() {
            return Row::new(vec!{Cell::from("Invalid data")});
        }

        let cell_time = chrono::NaiveDateTime::from_timestamp_opt(self.time.unwrap().into(), 0);
        let cells = [
            // TIME
            cell_time.map_or(
                Cell::from("Invalid format"),
                |t| Cell::from(t.to_string())
            ),
            // CALL
            self.call.as_ref().map_or(
                Cell::from(""),
                |v| Cell::from(v.clone())
            ),
            // ID
            Cell::from(format!("{}", self.get_id())),
            // CODE
            self.code.as_ref().map_or(
                Cell::from(""),
                |v| Cell::from(v.clone())
            ),
            // QTH
            self.position().map_or(
                Cell::from("N/A"),
                |p| Cell::from(p.to_qth())
            ),
            // DISTANCE
            self.position().map_or(
                Cell::from(""),
                |v| Cell::from(format!("{:.2}", v.distance_to(&app_ctx.data.config.own_position).km()))
            ),
            // AZIMUTH
            self.position().map_or(
                Cell::from(""),
                |v| Cell::from(format!("{:.1}", app_ctx.data.config.own_position.azimuth_to(&v)))
            )
        ];
        Row::new(cells).height(1)
    }

    pub fn position(&self) -> Option<Position> {
        Position::from_qth(&self.locator.as_ref()?).ok()
    }
}



impl Default for LogEntry {
    fn default() -> Self {
        LogEntry {
            time: Some(SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backward").as_secs() as u32),
            call: Some("".to_string()),
            code: Some("".to_string()),
            locator: Some("".to_string()),
            rowid: None,
            race_id: None,
        }
    }
}

impl<'a> Into<ListItem<'a>> for LogEntry {
    fn into(self) -> ListItem<'a> {
        ListItem::new(self.call.expect("LogEntry.name is None"))
    }
}
