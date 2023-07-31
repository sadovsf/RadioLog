




macro_rules! define_table {
    ($struct_name:ident, $($x:expr),+ $(,)?) => {
        static SCHEMA :[SchemaStep; 1] = [
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

pub(crate) use define_table;