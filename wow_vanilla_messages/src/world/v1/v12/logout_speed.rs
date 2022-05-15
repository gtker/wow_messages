use std::convert::{TryFrom, TryInto};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum LogoutSpeed {
    DELAYED,
    INSTANT,
}

impl LogoutSpeed {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::DELAYED => 0x0,
            Self::INSTANT => 0x1,
        }
    }

}

impl Default for LogoutSpeed {
    fn default() -> Self {
        Self::DELAYED
    }
}

impl std::fmt::Display for LogoutSpeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DELAYED => f.write_str("DELAYED"),
            Self::INSTANT => f.write_str("INSTANT"),
        }
    }
}

impl TryFrom<u8> for LogoutSpeed {
    type Error = LogoutSpeedError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::DELAYED),
            1 => Ok(Self::INSTANT),
            _ => Err(LogoutSpeedError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct LogoutSpeedError {
    pub value: u8,
}

impl LogoutSpeedError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

impl std::error::Error for LogoutSpeedError {}
impl std::fmt::Display for LogoutSpeedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'LogoutSpeed': '{}'", self.value))
    }
}

