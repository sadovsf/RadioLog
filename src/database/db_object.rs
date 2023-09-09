use rusqlite::Connection;



pub enum SchemaStep {
    SQL(&'static str),
    FN(&'static dyn Fn(&Connection) -> Result<(), rusqlite::Error>),
}


pub trait DBSchemaObject {
    fn table_name() -> &'static str;
    fn schema() -> &'static [SchemaStep];
}

pub trait DBObjectSerializable where Self :Sized {
    fn from_row(row :&rusqlite::Row) -> Result<Self, rusqlite::Error>;

    fn insert_row(&mut self, conn :&Connection) -> Result<(), rusqlite::Error>;
    fn update_row(&self, conn :&Connection) -> Result<(), rusqlite::Error>;
    fn delete_row(&self, conn :&Connection) -> Result<(), rusqlite::Error>;
}