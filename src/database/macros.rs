
macro_rules! count_tts {
    () => {0usize};
    ($_head:tt $($tail:tt)*) => {1usize + count_tts!($($tail)*)};
}



macro_rules! define_table {
    ($struct_name:ident, $($x:expr),+ $(,)?) => {
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

pub(crate) use define_table;
pub(crate) use count_tts;