use std::fs::create_dir_all;

use platform_dirs::AppDirs;
use rusqlite::Connection;

mod db_object;
pub use db_object::{DBSchemaObject, DBObjectSerializable, SchemaStep};

mod descriptor_table;
use descriptor_table::TableDescriptor;

use crate::app_errors::AppError;

pub mod macros;


pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn from_app_database() -> Result<Database, AppError> {
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

    pub fn new(path :&str) -> Result<Self, rusqlite::Error> {
        let mut inst = Self {
            connection: Connection::open(path).expect("Failed to open database"),
        };
        inst.register_type::<TableDescriptor>()?;
        Ok(inst)
    }

    pub fn register_type<T :DBSchemaObject + DBObjectSerializable>(&mut self) -> Result<(), rusqlite::Error> {
        let stmt = self.connection.prepare("SELECT * FROM TableDescriptor WHERE name=?1");
        if stmt.is_ok() {
            let descriptor = stmt.unwrap().query_row([T::table_name()], |row| -> Result<TableDescriptor, rusqlite::Error> {
                let name :String = row.get(0)?;
                assert_eq!(name, T::table_name());

                TableDescriptor::from_row(row)
            });

            self.update_schema::<T>(descriptor.ok())?;
        } else {
            drop(stmt);
            self.update_schema::<T>(None)?;
        }
        Ok(())
    }

    fn update_schema<T :DBSchemaObject + DBObjectSerializable>(&mut self, descriptor :Option<TableDescriptor>) -> Result<(), rusqlite::Error> {
        let schema = T::schema();

        let last_known_version = match descriptor.as_ref() {
            Some(descriptor) => descriptor.schema_version,
            None => 0,
        };

        if last_known_version as usize > schema.len() {
            panic!("Database schema is newer than the application");
        }
        if last_known_version as usize == schema.len() {
            return Ok(());
        }

        for step in &schema[last_known_version as usize..] {
            match step {
                SchemaStep::SQL(sql) => {
                    self.connection.execute(sql, [])?;
                },
                SchemaStep::FN(fn_ptr) => fn_ptr(&mut self.connection)?,
            }
        }

        if descriptor.is_some() {
            let mut descriptor = descriptor.unwrap();
            descriptor.schema_version = schema.len() as u32;
            self.update(&descriptor)?;
        } else {
            self.insert(
                &mut TableDescriptor::from_table::<T>()
            )?;
        }

        Ok(())
    }

    pub fn select_all<T :DBObjectSerializable + DBSchemaObject>(&mut self) -> Result<Vec<T>, rusqlite::Error> {
        let mut stmt = self.connection.prepare(format!("select * from {}", T::table_name()).as_str() )?;
        let iter = stmt.query_map((), |row| T::from_row(row) )?;

        let mut result = Vec::new();
        for el in iter {
            result.push(el?);
        }
        Ok(result)
    }

    pub fn select_where<T :DBObjectSerializable + DBSchemaObject>(&mut self, where_clause :&str, params :&[&dyn rusqlite::ToSql]) -> Result<Vec<T>, rusqlite::Error> {
        let mut stmt = self.connection.prepare(format!("select * from {} where {}", T::table_name(), where_clause).as_str() )?;
        let iter = stmt.query_map(params, |row| T::from_row(row) )?;

        let mut result = Vec::new();
        for el in iter {
            result.push(el?);
        }
        Ok(result)
    }

    pub fn insert(&mut self, obj :&mut impl DBObjectSerializable) -> Result<(), rusqlite::Error> {
        obj.insert_row(&mut self.connection)
    }

    pub fn update(&mut self, obj :&impl DBObjectSerializable) -> Result<(), rusqlite::Error> {
        obj.update_row(&mut self.connection)
    }

    pub fn delete(&mut self, obj :&impl DBObjectSerializable) -> Result<(), rusqlite::Error> {
        obj.delete_row(&mut self.connection)
    }
}
