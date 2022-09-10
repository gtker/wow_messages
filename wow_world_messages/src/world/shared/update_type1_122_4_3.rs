use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gameobject/smsg_update_object.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gameobject/smsg_update_object.wowm#L3):
/// ```text
/// enum UpdateType : u8 {
///     VALUES = 0;
///     MOVEMENT = 1;
///     CREATE_OBJECT = 2;
///     CREATE_OBJECT2 = 3;
///     OUT_OF_RANGE_OBJECTS = 4;
///     NEAR_OBJECTS = 5;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum UpdateType {
    Values,
    Movement,
    CreateObject,
    CreateObject2,
    OutOfRangeObjects,
    NearObjects,
}

impl UpdateType {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Values => 0x0,
            Self::Movement => 0x1,
            Self::CreateObject => 0x2,
            Self::CreateObject2 => 0x3,
            Self::OutOfRangeObjects => 0x4,
            Self::NearObjects => 0x5,
        }
    }

}

impl Default for UpdateType {
    fn default() -> Self {
        Self::Values
    }
}

impl std::fmt::Display for UpdateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Values => f.write_str("Values"),
            Self::Movement => f.write_str("Movement"),
            Self::CreateObject => f.write_str("CreateObject"),
            Self::CreateObject2 => f.write_str("CreateObject2"),
            Self::OutOfRangeObjects => f.write_str("OutOfRangeObjects"),
            Self::NearObjects => f.write_str("NearObjects"),
        }
    }
}

impl TryFrom<u8> for UpdateType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Values),
            1 => Ok(Self::Movement),
            2 => Ok(Self::CreateObject),
            3 => Ok(Self::CreateObject2),
            4 => Ok(Self::OutOfRangeObjects),
            5 => Ok(Self::NearObjects),
            v => Err(crate::errors::EnumError::new("UpdateType", v as u32),)
        }
    }
}

