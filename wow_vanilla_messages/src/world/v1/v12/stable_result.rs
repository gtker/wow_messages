use std::convert::{TryFrom, TryInto};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum StableResult {
    ERR_MONEY,
    ERR_STABLE,
    SUCCESS_STABLE,
    SUCCESS_UNSTABLE,
    SUCCESS_BUY_SLOT,
}

impl StableResult {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::ERR_MONEY => 0x1,
            Self::ERR_STABLE => 0x6,
            Self::SUCCESS_STABLE => 0x8,
            Self::SUCCESS_UNSTABLE => 0x9,
            Self::SUCCESS_BUY_SLOT => 0xa,
        }
    }

}

impl Default for StableResult {
    fn default() -> Self {
        Self::ERR_MONEY
    }
}

impl std::fmt::Display for StableResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ERR_MONEY => f.write_str("ERR_MONEY"),
            Self::ERR_STABLE => f.write_str("ERR_STABLE"),
            Self::SUCCESS_STABLE => f.write_str("SUCCESS_STABLE"),
            Self::SUCCESS_UNSTABLE => f.write_str("SUCCESS_UNSTABLE"),
            Self::SUCCESS_BUY_SLOT => f.write_str("SUCCESS_BUY_SLOT"),
        }
    }
}

impl TryFrom<u8> for StableResult {
    type Error = StableResultError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::ERR_MONEY),
            6 => Ok(Self::ERR_STABLE),
            8 => Ok(Self::SUCCESS_STABLE),
            9 => Ok(Self::SUCCESS_UNSTABLE),
            10 => Ok(Self::SUCCESS_BUY_SLOT),
            _ => Err(StableResultError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct StableResultError {
    pub value: u8,
}

impl StableResultError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

impl std::error::Error for StableResultError {}
impl std::fmt::Display for StableResultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'StableResult': '{}'", self.value))
    }
}

