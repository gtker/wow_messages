use std::convert::{TryFrom, TryInto};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum LogoutResult {
    SUCCESS,
    FAILURE_IN_COMBAT,
    FAILURE_FROZEN_BY_GM,
    FAILURE_JUMPING_OR_FALLING,
}

impl LogoutResult {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::SUCCESS => 0x0,
            Self::FAILURE_IN_COMBAT => 0x1,
            Self::FAILURE_FROZEN_BY_GM => 0x2,
            Self::FAILURE_JUMPING_OR_FALLING => 0x3,
        }
    }

}

impl Default for LogoutResult {
    fn default() -> Self {
        Self::SUCCESS
    }
}

impl std::fmt::Display for LogoutResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SUCCESS => f.write_str("SUCCESS"),
            Self::FAILURE_IN_COMBAT => f.write_str("FAILURE_IN_COMBAT"),
            Self::FAILURE_FROZEN_BY_GM => f.write_str("FAILURE_FROZEN_BY_GM"),
            Self::FAILURE_JUMPING_OR_FALLING => f.write_str("FAILURE_JUMPING_OR_FALLING"),
        }
    }
}

impl TryFrom<u32> for LogoutResult {
    type Error = LogoutResultError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::SUCCESS),
            1 => Ok(Self::FAILURE_IN_COMBAT),
            2 => Ok(Self::FAILURE_FROZEN_BY_GM),
            3 => Ok(Self::FAILURE_JUMPING_OR_FALLING),
            _ => Err(LogoutResultError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct LogoutResultError {
    value: u32,
}

impl LogoutResultError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

impl std::error::Error for LogoutResultError {}
impl std::fmt::Display for LogoutResultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'LogoutResult': '{}'", self.value))
    }
}

