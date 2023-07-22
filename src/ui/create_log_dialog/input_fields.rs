use std::{fmt::Display};

use super::CreateLogDialogState;


#[derive(PartialEq, Debug, Clone, Copy)]
#[repr(u8)]
#[allow(unused)]
pub enum InputFields {
    Name = 0,
    Latitude,
    Longtitude,
    LAST
}


impl Display for InputFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Into<u8> for InputFields {
    fn into(self) -> u8 {
        self as u8
    }
}

impl From<u8> for InputFields {
    fn from(val :u8) -> Self {
        if val >= InputFields::LAST.into() {
            panic!("Invalid value for InputFields");
        }
        unsafe { *(&val as *const u8 as *const Self) }
    }
}

impl InputFields {
    pub fn next(self) -> InputFields {
        let old_val = self as u8;
        let new_val = (old_val + 1) % InputFields::LAST as u8;
        new_val.into()
    }

    pub fn prev(self) -> InputFields {
        let old_val = self as u8;
        if old_val == 0 {
            return (InputFields::LAST as u8 - 1).into();
        }
        ((old_val - 1) % InputFields::LAST as u8).into()
    }

    pub fn to_field_mut<'a>(self, state :&'a mut CreateLogDialogState) -> &'a mut String {
        match self {
            InputFields::Name => &mut state.name,
            InputFields::Latitude => &mut state.latitude,
            InputFields::Longtitude => &mut state.longtitude,
            _ => panic!("Invalid value for InputFields")
        }
    }
    pub fn to_field<'a>(self, state :&'a CreateLogDialogState) -> &'a String {
        match self {
            InputFields::Name => &state.name,
            InputFields::Latitude => &state.latitude,
            InputFields::Longtitude => &state.longtitude,
            _ => panic!("Invalid value for InputFields")
        }
    }
}