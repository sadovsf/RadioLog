use ratatui::widgets::ListItem;
use crate::database::{macros::{declare_table, define_table_data}, SchemaStep, DBObjectSerializable, DBSchemaObject};
use super::data_store::DataStoreTrait;


declare_table!(Race,
    SchemaStep::SQL(
        "CREATE TABLE Race (
            id          INTEGER PRIMARY KEY,
            create_time UINT   ,
            name        TEXT   ,
            my_location TEXT   ,
            my_call     TEXT
        )"
    )
);

define_table_data!(Race,
    (create_time: u32    ),
    (name       : String ),
    (my_location: String ),
    (my_call    : String )
);

impl<'a> Into<ListItem<'a>> for Race {
    fn into(self) -> ListItem<'a> {
        ListItem::new(self.name)
    }
}
