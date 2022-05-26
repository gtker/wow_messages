use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum ObjectType {
    OBJECT,
    ITEM,
    CONTAINER,
    UNIT,
    PLAYER,
    GAME_OBJECT,
    DYNAMIC_OBJECT,
    CORPSE,
}

impl ObjectType {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::OBJECT => 0x0,
            Self::ITEM => 0x1,
            Self::CONTAINER => 0x2,
            Self::UNIT => 0x3,
            Self::PLAYER => 0x4,
            Self::GAME_OBJECT => 0x5,
            Self::DYNAMIC_OBJECT => 0x6,
            Self::CORPSE => 0x7,
        }
    }

}

impl Default for ObjectType {
    fn default() -> Self {
        Self::OBJECT
    }
}

impl std::fmt::Display for ObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OBJECT => f.write_str("OBJECT"),
            Self::ITEM => f.write_str("ITEM"),
            Self::CONTAINER => f.write_str("CONTAINER"),
            Self::UNIT => f.write_str("UNIT"),
            Self::PLAYER => f.write_str("PLAYER"),
            Self::GAME_OBJECT => f.write_str("GAME_OBJECT"),
            Self::DYNAMIC_OBJECT => f.write_str("DYNAMIC_OBJECT"),
            Self::CORPSE => f.write_str("CORPSE"),
        }
    }
}

impl TryFrom<u8> for ObjectType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::OBJECT),
            1 => Ok(Self::ITEM),
            2 => Ok(Self::CONTAINER),
            3 => Ok(Self::UNIT),
            4 => Ok(Self::PLAYER),
            5 => Ok(Self::GAME_OBJECT),
            6 => Ok(Self::DYNAMIC_OBJECT),
            7 => Ok(Self::CORPSE),
            v => Err(crate::errors::EnumError::new("ObjectType", v as u32),)
        }
    }
}

