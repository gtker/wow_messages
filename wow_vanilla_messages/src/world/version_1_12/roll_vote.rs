use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum RollVote {
    PASS,
    NEED,
    GREED,
}

impl RollVote {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::PASS => 0x0,
            Self::NEED => 0x1,
            Self::GREED => 0x2,
        }
    }

}

impl Default for RollVote {
    fn default() -> Self {
        Self::PASS
    }
}

impl std::fmt::Display for RollVote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PASS => f.write_str("PASS"),
            Self::NEED => f.write_str("NEED"),
            Self::GREED => f.write_str("GREED"),
        }
    }
}

impl TryFrom<u8> for RollVote {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::PASS),
            1 => Ok(Self::NEED),
            2 => Ok(Self::GREED),
            v => Err(crate::errors::EnumError::new("RollVote", v as u32),)
        }
    }
}

