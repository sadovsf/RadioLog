use std::{cell::RefCell, fs::create_dir_all, rc::Rc};

mod config;
use config::ConfigData;

pub mod position;
use platform_dirs::AppDirs;
use position::Position;


mod logs;
pub use logs::LogEntry;

mod data_store;
use data_store::DataStore;

use crate::{database::Database, app_errors::AppError};


pub struct Data {
    pub logs: DataStore<LogEntry>,
    pub config: ConfigData,
}

impl Data {
    fn setup_database() -> Result<Database, AppError> {
        let app_paths = AppDirs::new(
            Some("org.sadovsf.radio_log"), false
        ).ok_or(
            std::io::Error::new(std::io::ErrorKind::NotFound, "Unable to find user directory!")
        )?;

        let mut db_path = app_paths.data_dir.clone();
        create_dir_all(&db_path)?;

        db_path.push("data.sqlite");
        let full_path = db_path.to_str().expect("Failed to convert path to string");

        Ok(Database::new(full_path)?)
    }

    pub fn new() -> Result<Self, AppError> {
        let database = Rc::new(RefCell::new(Data::setup_database()?));
        Ok(Self {
            logs: DataStore::new(Rc::clone(&database))?,

            config: ConfigData {
                own_position: Position::new(50.061520, 14.091540)
            }
        })
    }
}