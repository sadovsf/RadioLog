use turbosql::select;

mod config;
use config::ConfigData;

pub mod position;
use position::Position;


mod logs;
pub use logs::LogEntry;

mod data_store;
use data_store::DataStore;






pub struct Data {
    pub logs: DataStore<LogEntry>,
    pub config: ConfigData,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            logs: DataStore::from(select!(Vec<LogEntry>).expect("Failed to load logs")),
            config: ConfigData {
                own_position: Position::new(50.061520, 14.091540)
            }
        }
    }
}