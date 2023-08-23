
macro_rules! count_tts {
    () => {0usize};
    ($_head:tt $($tail:tt)*) => {1usize + count_tts!($($tail)*)};
}



macro_rules! declare_table {
    ($struct_name:ident, $($x:expr),+) => {
        use crate::database::macros::count_tts;
        const SCHEMA :[SchemaStep; count_tts!($($x)*)] = [
            $($x),+
        ];

        impl DBSchemaObject for $struct_name {
            fn table_name() -> &'static str {
                stringify!($struct_name)
            }

            fn schema() -> &'static [SchemaStep] {
                &SCHEMA
            }
        }
    }
}

macro_rules! define_table_data {
    ($struct_name:ident, $( ($field:ident: $field_type:path) ),+) => {

        // Define DB data structure
        #[derive(Debug, Clone, PartialEq)]
        pub struct $struct_name {
            pub id: i64,
            $(
                pub $field: $field_type
            ),+
        }

        impl DBObjectSerializable for $struct_name {

            fn insert_row(&mut self, conn :&mut rusqlite::Connection) -> Result<(), rusqlite::Error> {
                let sql = {
                    let mut sql = format!("INSERT INTO {} (", stringify!($struct_name));
                    $(
                        sql.push_str(stringify!($field));
                        sql.push_str(",");
                    )*
                    sql.pop();
                    sql.push_str(") VALUES (");
                    let mut val_index = 0;
                    $(
                        val_index += 1;
                        let _ = &self.$field;
                        sql.push_str(&format!("?{},", val_index));
                    )*
                    sql.pop();
                    sql.push_str(")");
                    sql
                };
                conn.execute(&sql,
                    ( $(&self.$field,)* )
                )?;
                self.id = conn.last_insert_rowid();
                Ok(())
            }

            fn update_row(&self, conn :&mut rusqlite::Connection) -> Result<(), rusqlite::Error> {
                let sql = {
                    let mut sql = format!("UPDATE {} SET ", stringify!($struct_name));
                    let mut val_index = 0;
                    $(
                        val_index += 1;
                        sql.push_str(&format!("{}=?{},", stringify!($field), val_index));
                    )*
                    sql.pop();
                    sql.push_str(&format!(" WHERE id=?{}", val_index + 1));
                    sql
                };
                conn.execute(&sql,
                    ( $(&self.$field),* , &self.id )
                )?;
                Ok(())
            }

            fn from_row(row :&rusqlite::Row) -> Result<Self, rusqlite::Error> {
                Ok(Self {
                    id: row.get("id")?,
                    $(
                        $field: row.get(stringify!($field))?
                    ),+
                })
            }
            fn delete_row(&self, conn :&mut rusqlite::Connection) -> Result<(), rusqlite::Error> {
                conn.execute(concat!("DELETE FROM ", stringify!($struct_name), " WHERE id=?1"), [&self.id])?;
                Ok(())
            }
        }

        impl DataStoreTrait for $struct_name {
            fn set_id(&mut self, id: i64) {
                self.id = id;
            }
        
            fn get_id(&self) -> i64 {
                self.id
            }
        }
    }
}

pub(crate) use declare_table;
pub(crate) use define_table_data;
pub(crate) use count_tts;
