use std::convert::{TryFrom, TryInto};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum FarSightOperation {
    REMOVE,
    ADD,
}

impl FarSightOperation {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::REMOVE => 0x0,
            Self::ADD => 0x1,
        }
    }

}

impl Default for FarSightOperation {
    fn default() -> Self {
        Self::REMOVE
    }
}

impl std::fmt::Display for FarSightOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::REMOVE => f.write_str("REMOVE"),
            Self::ADD => f.write_str("ADD"),
        }
    }
}

impl TryFrom<u8> for FarSightOperation {
    type Error = FarSightOperationError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::REMOVE),
            1 => Ok(Self::ADD),
            _ => Err(FarSightOperationError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct FarSightOperationError {
    value: u8,
}

impl FarSightOperationError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

impl std::error::Error for FarSightOperationError {}
impl std::fmt::Display for FarSightOperationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'FarSightOperation': '{}'", self.value))
    }
}

