use std::cell::RefCell;

mod config;
use config::ConfigData;

pub mod position;
use position::Position;


mod logs;
pub use logs::LogEntry;

mod data_store;
use data_store::DataStore;

use crate::{database::Database, app_errors::AppError};


pub struct Data<'a> {
    pub logs: DataStore<'a, LogEntry>,
    pub config: ConfigData,
}

impl<'a> Data<'a> {
    pub fn new(db :&'a RefCell<Database>) -> Result<Self, AppError> {
        Ok(Self {
            logs: DataStore::new(db)?,

            config: ConfigData {
                own_position: Position::new(50.061520, 14.091540)
            }
        })
    }
}