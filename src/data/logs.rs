use std::time::{SystemTime, UNIX_EPOCH};
use ratatui::{widgets::{ListItem, Cell, Row}, style::{Style, Color}, prelude::Constraint};
use crate::{database::{macros::{declare_table, define_table_data}, SchemaStep, DBObjectSerializable, DBSchemaObject}, app_context::AppContext, app_errors::AppError};
use super::{Position, data_store::DataStoreTrait};
use rusqlite::Connection;

fn change_location_storage(conn :&Connection) -> Result<(), rusqlite::Error> {
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
            id: id,
            time: time,
            call: call,
            code: None,
            locator: Position::new(lat, long).to_qth(),
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

    conn.execute_batch("
        ALTER TABLE LogEntry DROP COLUMN long;
        ALTER TABLE LogEntry DROP COLUMN lat;
        ALTER TABLE LogEntry ADD COLUMN code TEXT;
        ALTER TABLE LogEntry ADD COLUMN locator TEXT;
    ")?;

    for log in unpacked_logs {
        log.update_row(conn).expect("Failed to update row");
    }

    Ok(())
}




declare_table!(LogEntry,
    SchemaStep::SQL(
        "CREATE TABLE LogEntry (
            id   INTEGER PRIMARY KEY,
            long REAL   ,
            lat  REAL   ,
            time UINT   ,
            name TEXT
        )"
    ),
    SchemaStep::SQL(
        "ALTER TABLE LogEntry RENAME COLUMN name TO call"
    ),
    SchemaStep::FN( &|conn :&Connection| change_location_storage(conn) ),
    SchemaStep::SQL(
        "ALTER TABLE LogEntry ADD COLUMN race_id INTEGER"
    )
);

define_table_data!(LogEntry,
    (time   : u32           ),
    (call   : String        ),
    (locator: String        ),
    (code   : Option<String>),
    (race_id: Option<i64>   )
);

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
        let my_position = self.my_position(app_ctx);
        let cell_time = chrono::NaiveDateTime::from_timestamp_opt(self.time.into(), 0);
        let cells = [
            // TIME
            cell_time.map_or(
                Cell::from("Invalid format"),
                |t| Cell::from(t.to_string())
            ),
            // CALL
            Cell::from(self.call.clone()),
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
                |v| {
                    let dist = match my_position {
                        Ok(my_position) => my_position.distance_to(&v),
                        Err(_) => return Cell::from("N/A")
                    };
                    Cell::from(format!("{:.2}", dist.km()))
                }
            ),
            // AZIMUTH
            self.position().map_or(
                Cell::from(""),
                |v| {
                    let azim = match my_position {
                        Ok(my_position) => my_position.azimuth_to(&v),
                        Err(_) => return Cell::from("N/A")
                    };
                    Cell::from(format!("{:.1}", azim))
                }
            )
        ];
        Row::new(cells).height(1)
    }

    pub fn position(&self) -> Option<Position> {
        Position::from_qth(&self.locator).ok()
    }

    pub fn my_position(&self, app_ctx :&AppContext) -> Result<Position, AppError> {
        match self.race_id {
            Some(race_id) => app_ctx.data.races.get(race_id).map_or(
                Ok(app_ctx.data.config.own_position),
                |v| Position::from_qth(&v.my_location).or(Err(AppError::InvalidQTHLocator))
            ),
            None => Ok(app_ctx.data.config.own_position)
        }

    }
}



impl Default for LogEntry {
    fn default() -> Self {
        LogEntry {
            time: SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backward").as_secs() as u32,
            call: "".to_string(),
            code: None,
            locator: "".to_string(),
            id: 0,
            race_id: None,
        }
    }
}

impl<'a> Into<ListItem<'a>> for LogEntry {
    fn into(self) -> ListItem<'a> {
        ListItem::new(self.call)
    }
}
