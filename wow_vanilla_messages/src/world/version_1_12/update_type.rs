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
pub(crate) enum UpdateType {
    VALUES,
    MOVEMENT,
    CREATE_OBJECT,
    CREATE_OBJECT2,
    OUT_OF_RANGE_OBJECTS,
    NEAR_OBJECTS,
}

impl UpdateType {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::VALUES => 0x0,
            Self::MOVEMENT => 0x1,
            Self::CREATE_OBJECT => 0x2,
            Self::CREATE_OBJECT2 => 0x3,
            Self::OUT_OF_RANGE_OBJECTS => 0x4,
            Self::NEAR_OBJECTS => 0x5,
        }
    }

}

impl Default for UpdateType {
    fn default() -> Self {
        Self::VALUES
    }
}

impl std::fmt::Display for UpdateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::VALUES => f.write_str("VALUES"),
            Self::MOVEMENT => f.write_str("MOVEMENT"),
            Self::CREATE_OBJECT => f.write_str("CREATE_OBJECT"),
            Self::CREATE_OBJECT2 => f.write_str("CREATE_OBJECT2"),
            Self::OUT_OF_RANGE_OBJECTS => f.write_str("OUT_OF_RANGE_OBJECTS"),
            Self::NEAR_OBJECTS => f.write_str("NEAR_OBJECTS"),
        }
    }
}

impl TryFrom<u8> for UpdateType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::VALUES),
            1 => Ok(Self::MOVEMENT),
            2 => Ok(Self::CREATE_OBJECT),
            3 => Ok(Self::CREATE_OBJECT2),
            4 => Ok(Self::OUT_OF_RANGE_OBJECTS),
            5 => Ok(Self::NEAR_OBJECTS),
            v => Err(crate::errors::EnumError::new("UpdateType", v as u32),)
        }
    }
}

