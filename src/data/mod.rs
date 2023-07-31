use std::{cell::RefCell, rc::Rc, fs::create_dir_all};

mod config;
use config::ConfigData;

pub mod position;
use platform_dirs::AppDirs;
use position::Position;


mod logs;
pub use logs::LogEntry;

mod data_store;
use data_store::DataStore;

use crate::database::Database;






pub struct Data {
    pub logs: DataStore<LogEntry>,
    pub config: ConfigData,
}

impl Data {
    pub fn new() -> Result<Self, rusqlite::Error> {
        // TODO properly handle errors
        let app_paths = AppDirs::new(Some("org.sadovsf.radio_log"), false).expect("Failed to get app dirs");

        let mut db_path = app_paths.data_dir.clone();
        create_dir_all(&db_path).expect("Failed to create data dir");

        db_path.push("data.sqlite");
        let full_path = db_path.to_str().expect("Failed to convert path to string");

        let db = Rc::new(RefCell::new(
            Database::new(full_path).expect("Failed to open database")
        ));
        db.as_ref().borrow_mut().register_type::<LogEntry>()?;


        Ok(Self {
            logs: DataStore::from(Rc::clone(&db))?,
            config: ConfigData {
                own_position: Position::new(50.061520, 14.091540)
            }
        })
    }
}