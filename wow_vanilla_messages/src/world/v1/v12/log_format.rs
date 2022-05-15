use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum LogFormat {
    DEFAULT,
    DEBUG,
}

impl LogFormat {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::DEFAULT => 0x0,
            Self::DEBUG => 0x1,
        }
    }

}

impl ConstantSized for LogFormat {}

impl MaximumPossibleSized for LogFormat {
    fn maximum_possible_size() -> usize {
        1
    }
}

impl Default for LogFormat {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl std::fmt::Display for LogFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DEFAULT => f.write_str("DEFAULT"),
            Self::DEBUG => f.write_str("DEBUG"),
        }
    }
}

impl TryFrom<u8> for LogFormat {
    type Error = LogFormatError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::DEFAULT),
            1 => Ok(Self::DEBUG),
            _ => Err(LogFormatError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct LogFormatError {
    value: u8,
}

impl LogFormatError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

impl std::error::Error for LogFormatError {}
impl std::fmt::Display for LogFormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'LogFormat': '{}'", self.value))
    }
}

