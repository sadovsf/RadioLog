mod log_list;
pub use log_list::LogList;


mod world_map;
pub use world_map::{WorldMap, WorldMapState};

mod create_log_dialog;
pub use create_log_dialog::CreateLogDialog;

mod alert_dialog;
pub use alert_dialog::AlertDialog;
pub use alert_dialog::AlertDialogButton;
pub use alert_dialog::AlertDialogStyle;

mod details_window;
pub use self::details_window::DetailsWindow;

use self::log_list::LogListState;


#[derive(Default)]
pub struct UIState {
    pub log_list_state :LogListState,
    pub world_map_state :WorldMapState,
}