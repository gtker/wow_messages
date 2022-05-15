use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum PartyOperation {
    INVITE,
    LEAVE,
}

impl PartyOperation {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::INVITE => 0x0,
            Self::LEAVE => 0x2,
        }
    }

}

impl Default for PartyOperation {
    fn default() -> Self {
        Self::INVITE
    }
}

impl std::fmt::Display for PartyOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::INVITE => f.write_str("INVITE"),
            Self::LEAVE => f.write_str("LEAVE"),
        }
    }
}

impl TryFrom<u8> for PartyOperation {
    type Error = PartyOperationError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::INVITE),
            2 => Ok(Self::LEAVE),
            _ => Err(PartyOperationError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct PartyOperationError {
    value: u8,
}

impl PartyOperationError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

impl std::error::Error for PartyOperationError {}
impl std::fmt::Display for PartyOperationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'PartyOperation': '{}'", self.value))
    }
}

