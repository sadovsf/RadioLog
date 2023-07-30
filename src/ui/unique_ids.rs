
pub const fn create_const_hash(module_path: &'static str, file: &'static str, type_name :&'static str) -> u64 {
    let mut hash = 0xcbf29ce484222325;
    let prime = 0x00000100000001B3;

    let mut bytes = module_path.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        hash ^= bytes[i] as u64;
        hash = hash.wrapping_mul(prime);
        i += 1;
    }

    bytes = file.as_bytes();
    i = 0;
    while i < bytes.len() {
        hash ^= bytes[i] as u64;
        hash = hash.wrapping_mul(prime);
        i += 1;
    }

    bytes = type_name.as_bytes();
    i = 0;
    while i < bytes.len() {
        hash ^= bytes[i] as u64;
        hash = hash.wrapping_mul(prime);
        i += 1;
    };

    hash
}


macro_rules! _define_typed_element_internal {
    ($type_name:ident) => {
        use crate::traits::{UIElementType, TypedUIElement};
        use crate::ui::create_const_hash;

        pub const TYPED_ELEMENT_TYPE :UIElementType = UIElementType::new(
            stringify!($type_name),
            create_const_hash(module_path!(), file!(), stringify!($type_name))
        );

        macro_rules! implement_typed_element {
            () => {
                fn get_type(&self) -> &'static UIElementType {
                    &TYPED_ELEMENT_TYPE
                }
            }
        }
    }
}


macro_rules! define_typed_element {
    ($type_name:ident) => {
        use crate::ui::_define_typed_element_internal;
        _define_typed_element_internal!($type_name);

        impl TypedUIElement for $type_name {
            fn get_type_type() -> &'static UIElementType {
                &TYPED_ELEMENT_TYPE
            }
        }
    };

    ($type_name:ident < $( $gen:tt ),+ >) => {
        use crate::ui::unique_ids::_define_typed_element_internal;
        _define_typed_element_internal!($type_name);

        impl TypedUIElement for $type_name<$($gen)*> {
            fn get_type_type() -> &'static UIElementType {
                &TYPED_ELEMENT_TYPE
            }
        }
    }
}
pub(crate) use define_typed_element;
pub(crate) use _define_typed_element_internal;