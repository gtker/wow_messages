use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
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
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::DEFAULT),
            1 => Ok(Self::DEBUG),
            v => Err(crate::errors::EnumError::new("LogFormat", v as u32),)
        }
    }
}

