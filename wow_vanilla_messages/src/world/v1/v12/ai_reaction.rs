use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
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
    type Error = AiReactionError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::ALERT),
            1 => Ok(Self::FRIENDLY),
            2 => Ok(Self::HOSTILE),
            3 => Ok(Self::AFRAID),
            4 => Ok(Self::DESTROY),
            _ => Err(AiReactionError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct AiReactionError {
    value: u32,
}

impl AiReactionError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

impl std::error::Error for AiReactionError {}
impl std::fmt::Display for AiReactionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'AiReaction': '{}'", self.value))
    }
}

