use std::cell::RefCell;

mod config;
use config::ConfigData;

pub mod position;
use position::Position;


mod logs;
pub use logs::LogEntry;

mod races;
pub use races::Race;

mod data_store;
use data_store::DataStore;

use crate::{database::Database, app_errors::AppError};


pub struct Data<'a> {
    pub logs: DataStore<'a, LogEntry>,
    pub races: DataStore<'a, Race>,

    pub config: ConfigData,

    pub current_race_id: Option<i64>,
}

impl<'a> Data<'a> {
    pub fn new(db :&'a RefCell<Database>) -> Result<Self, AppError> {
        Ok(Self {
            races: DataStore::new(db)?,
            logs: DataStore::new(db)?,

            config: ConfigData {
                own_position: Position::new(50.061520, 14.091540)
            },

            current_race_id: None,
        })
    }

    pub fn race_logs(&self, race_id :Option<i64>) -> impl Iterator<Item = &LogEntry> {
        self.logs.iter().filter(move |v| race_id.is_none() || (*v).race_id == race_id)
    }

    pub fn my_position(&self) -> Position {
        match self.current_race_id {
            Some(id) => self.races.get(id).map(
                |r| Position::from_qth(&r.my_location)
            ).map_or(self.config.own_position, |v| v.unwrap()),
            None => self.config.own_position
        }
    }
}