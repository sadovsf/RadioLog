use ratatui::widgets::ListItem;
use crate::database::{macros::define_table, SchemaStep, DBObjectSerializable, DBSchemaObject};
use super::data_store::DataStoreTrait;


define_table!(Race,
    SchemaStep::SQL(
        "CREATE TABLE Race (
            id INTEGER PRIMARY KEY,
            create_time UINT,
            name TEXT,
            my_location TEXT,
            my_call TEXT
        )"
    )
);


#[derive(Debug, Clone, PartialEq)]
pub struct Race {
    pub id: Option<i64>,
    pub create_time: i64,
    pub name: String,
    pub my_location: String,
    pub my_call: String,
}

impl DBObjectSerializable for Race {
    fn from_row(row :&rusqlite::Row) -> Result<Self, rusqlite::Error> {
        Ok(Self {
            id: row.get(0)?,
            create_time: row.get(1)?,
            name: row.get(2)?,
            my_location: row.get(3)?,
            my_call: row.get(4)?,
        })
    }

    fn insert_row(&mut self, conn :&mut rusqlite::Connection) -> Result<(), rusqlite::Error> {
        conn.execute(
            "INSERT INTO Race VALUES (DEFAULT, ?1, ?2, ?3, ?4)",
            (&self.create_time, &self.name, &self.my_location, &self.my_call)
        )?;
        self.id = Some(conn.last_insert_rowid());
        Ok(())
    }

    fn update_row(&self, conn :&mut rusqlite::Connection) -> Result<(), rusqlite::Error> {
        conn.execute(
            "UPDATE Race SET create_time=?1, name=?2, my_location=?3, my_call=?4 WHERE id=?5",
            (&self.create_time, &self.name, &self.my_location, &self.my_call, &self.id)
        )?;
        Ok(())
    }

    fn delete_row(&self, conn :&mut rusqlite::Connection) -> Result<(), rusqlite::Error> {
        conn.execute("DELETE FROM Race WHERE id=?1", [&self.id])?;
        Ok(())
    }
}

impl DataStoreTrait for Race {
    fn set_id(&mut self, id: i64) {
        self.id = Some(id);
    }

    fn get_id(&self) -> i64 {
        self.id.expect("Race.id is None")
    }
}

impl<'a> Into<ListItem<'a>> for Race {
    fn into(self) -> ListItem<'a> {
        ListItem::new(self.name)
    }
}
