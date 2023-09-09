use rusqlite::Connection;

use super::{DBSchemaObject, SchemaStep, macros::declare_table, db_object::DBObjectSerializable};




declare_table!(TableDescriptor,
    SchemaStep::SQL(
        "CREATE TABLE TableDescriptor (
            name TEXT PRIMARY KEY NOT NULL,
            schema_version UINT NOT NULL
        )"
    )
);


pub struct TableDescriptor {
    pub name: String,
    pub schema_version: u32,
}

impl TableDescriptor {
    pub fn from_table<T :DBSchemaObject>() -> Self {
        Self {
            name: T::table_name().to_string(),
            schema_version: T::schema().len() as u32,
        }
    }
}

impl DBObjectSerializable for TableDescriptor {
    fn from_row(row :&rusqlite::Row) -> Result<Self, rusqlite::Error> {
        Ok(Self {
            name: row.get(0).unwrap_or_default(),
            schema_version: row.get(1).unwrap_or_default(),
        })
    }

    fn update_row(&self, conn :&Connection) -> Result<(), rusqlite::Error> {
        conn.execute("UPDATE TableDescriptor SET schema_version=?1 WHERE name=?2", (&self.schema_version, &self.name))?;
        Ok(())
    }

    fn delete_row(&self, conn :&Connection) -> Result<(), rusqlite::Error> {
        conn.execute("DELETE FROM TableDescriptor WHERE name=?1", [&self.name])?;
        Ok(())
    }

    fn insert_row(&mut self, conn :&Connection) -> Result<(), rusqlite::Error> {
        conn.execute("INSERT INTO TableDescriptor (name, schema_version) VALUES (?1, ?2)", (&self.name, &self.schema_version))?;
        Ok(())
    }
}