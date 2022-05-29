use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum AiReaction {
    ALERT,
    FRIENDLY,
    HOSTILE,
    AFRAID,
    DESTROY,
}

impl AiReaction {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::ALERT => 0x0,
            Self::FRIENDLY => 0x1,
            Self::HOSTILE => 0x2,
            Self::AFRAID => 0x3,
            Self::DESTROY => 0x4,
        }
    }

}

impl Default for AiReaction {
    fn default() -> Self {
        Self::ALERT
    }
}

impl std::fmt::Display for AiReaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ALERT => f.write_str("ALERT"),
            Self::FRIENDLY => f.write_str("FRIENDLY"),
            Self::HOSTILE => f.write_str("HOSTILE"),
            Self::AFRAID => f.write_str("AFRAID"),
            Self::DESTROY => f.write_str("DESTROY"),
        }
    }
}

impl TryFrom<u32> for AiReaction {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::ALERT),
            1 => Ok(Self::FRIENDLY),
            2 => Ok(Self::HOSTILE),
            3 => Ok(Self::AFRAID),
            4 => Ok(Self::DESTROY),
            v => Err(crate::errors::EnumError::new("AiReaction", v as u32),)
        }
    }
}

