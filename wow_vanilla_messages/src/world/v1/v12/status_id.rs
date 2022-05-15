use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub(crate) enum StatusId {
    NONE,
    WAIT_QUEUE,
    WAIT_JOIN,
    IN_PROGRESS,
    WAIT_LEAVE,
}

impl StatusId {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::NONE => 0x0,
            Self::WAIT_QUEUE => 0x1,
            Self::WAIT_JOIN => 0x2,
            Self::IN_PROGRESS => 0x3,
            Self::WAIT_LEAVE => 0x4,
        }
    }

}

impl ConstantSized for StatusId {}

impl MaximumPossibleSized for StatusId {
    fn maximum_possible_size() -> usize {
        1
    }
}

impl Default for StatusId {
    fn default() -> Self {
        Self::NONE
    }
}

impl std::fmt::Display for StatusId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NONE => f.write_str("NONE"),
            Self::WAIT_QUEUE => f.write_str("WAIT_QUEUE"),
            Self::WAIT_JOIN => f.write_str("WAIT_JOIN"),
            Self::IN_PROGRESS => f.write_str("IN_PROGRESS"),
            Self::WAIT_LEAVE => f.write_str("WAIT_LEAVE"),
        }
    }
}

impl TryFrom<u8> for StatusId {
    type Error = StatusIdError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NONE),
            1 => Ok(Self::WAIT_QUEUE),
            2 => Ok(Self::WAIT_JOIN),
            3 => Ok(Self::IN_PROGRESS),
            4 => Ok(Self::WAIT_LEAVE),
            _ => Err(StatusIdError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct StatusIdError {
    value: u8,
}

impl StatusIdError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

impl std::error::Error for StatusIdError {}
impl std::fmt::Display for StatusIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'StatusId': '{}'", self.value))
    }
}

