use std::convert::{TryFrom, TryInto};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum RaidGroupError {
    REQUIRED,
    FULL,
}

impl RaidGroupError {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::REQUIRED => 0x1,
            Self::FULL => 0x2,
        }
    }

}

impl Default for RaidGroupError {
    fn default() -> Self {
        Self::REQUIRED
    }
}

impl std::fmt::Display for RaidGroupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::REQUIRED => f.write_str("REQUIRED"),
            Self::FULL => f.write_str("FULL"),
        }
    }
}

impl TryFrom<u32> for RaidGroupError {
    type Error = RaidGroupErrorError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::REQUIRED),
            2 => Ok(Self::FULL),
            _ => Err(RaidGroupErrorError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct RaidGroupErrorError {
    value: u32,
}

impl RaidGroupErrorError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

impl std::error::Error for RaidGroupErrorError {}
impl std::fmt::Display for RaidGroupErrorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'RaidGroupError': '{}'", self.value))
    }
}

