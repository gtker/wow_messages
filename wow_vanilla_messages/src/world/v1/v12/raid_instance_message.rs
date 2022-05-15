use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum RaidInstanceMessage {
    WARNING_HOURS,
    WARNING_MIN,
    WARNING_MIN_SOON,
    WELCOME,
}

impl RaidInstanceMessage {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::WARNING_HOURS => 0x1,
            Self::WARNING_MIN => 0x2,
            Self::WARNING_MIN_SOON => 0x3,
            Self::WELCOME => 0x4,
        }
    }

}

impl ConstantSized for RaidInstanceMessage {}

impl MaximumPossibleSized for RaidInstanceMessage {
    fn maximum_possible_size() -> usize {
        4
    }
}

impl Default for RaidInstanceMessage {
    fn default() -> Self {
        Self::WARNING_HOURS
    }
}

impl std::fmt::Display for RaidInstanceMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WARNING_HOURS => f.write_str("WARNING_HOURS"),
            Self::WARNING_MIN => f.write_str("WARNING_MIN"),
            Self::WARNING_MIN_SOON => f.write_str("WARNING_MIN_SOON"),
            Self::WELCOME => f.write_str("WELCOME"),
        }
    }
}

impl TryFrom<u32> for RaidInstanceMessage {
    type Error = RaidInstanceMessageError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::WARNING_HOURS),
            2 => Ok(Self::WARNING_MIN),
            3 => Ok(Self::WARNING_MIN_SOON),
            4 => Ok(Self::WELCOME),
            _ => Err(RaidInstanceMessageError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct RaidInstanceMessageError {
    value: u32,
}

impl RaidInstanceMessageError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

impl std::error::Error for RaidInstanceMessageError {}
impl std::fmt::Display for RaidInstanceMessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'RaidInstanceMessage': '{}'", self.value))
    }
}

