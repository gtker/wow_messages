use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_monster_move.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_monster_move.wowm#L3):
/// ```text
/// enum MonsterMoveType : u8 {
///     NORMAL = 0;
///     STOP = 1;
///     FACING_SPOT = 2;
///     FACING_TARGET = 3;
///     FACING_ANGLE = 4;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum MonsterMoveType {
    NORMAL,
    STOP,
    FACING_SPOT,
    FACING_TARGET,
    FACING_ANGLE,
}

impl MonsterMoveType {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::NORMAL => 0x0,
            Self::STOP => 0x1,
            Self::FACING_SPOT => 0x2,
            Self::FACING_TARGET => 0x3,
            Self::FACING_ANGLE => 0x4,
        }
    }

}

impl Default for MonsterMoveType {
    fn default() -> Self {
        Self::NORMAL
    }
}

impl std::fmt::Display for MonsterMoveType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NORMAL => f.write_str("NORMAL"),
            Self::STOP => f.write_str("STOP"),
            Self::FACING_SPOT => f.write_str("FACING_SPOT"),
            Self::FACING_TARGET => f.write_str("FACING_TARGET"),
            Self::FACING_ANGLE => f.write_str("FACING_ANGLE"),
        }
    }
}

impl TryFrom<u8> for MonsterMoveType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NORMAL),
            1 => Ok(Self::STOP),
            2 => Ok(Self::FACING_SPOT),
            3 => Ok(Self::FACING_TARGET),
            4 => Ok(Self::FACING_ANGLE),
            v => Err(crate::errors::EnumError::new("MonsterMoveType", v as u32),)
        }
    }
}

