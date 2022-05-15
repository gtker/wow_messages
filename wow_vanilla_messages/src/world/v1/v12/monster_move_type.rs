use std::convert::{TryFrom, TryInto};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

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
    type Error = MonsterMoveTypeError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NORMAL),
            1 => Ok(Self::STOP),
            2 => Ok(Self::FACING_SPOT),
            3 => Ok(Self::FACING_TARGET),
            4 => Ok(Self::FACING_ANGLE),
            _ => Err(MonsterMoveTypeError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct MonsterMoveTypeError {
    pub value: u8,
}

impl MonsterMoveTypeError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

impl std::error::Error for MonsterMoveTypeError {}
impl std::fmt::Display for MonsterMoveTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'MonsterMoveType': '{}'", self.value))
    }
}

