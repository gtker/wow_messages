use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

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
    type Error = UpdateTypeError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::VALUES),
            1 => Ok(Self::MOVEMENT),
            2 => Ok(Self::CREATE_OBJECT),
            3 => Ok(Self::CREATE_OBJECT2),
            4 => Ok(Self::OUT_OF_RANGE_OBJECTS),
            5 => Ok(Self::NEAR_OBJECTS),
            _ => Err(UpdateTypeError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct UpdateTypeError {
    value: u8,
}

impl UpdateTypeError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

impl std::error::Error for UpdateTypeError {}
impl std::fmt::Display for UpdateTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'UpdateType': '{}'", self.value))
    }
}

