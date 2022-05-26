use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum TrainingFailureReason {
    UNAVAILABLE,
    NOT_ENOUGH_MONEY,
    NOT_ENOUGH_SKILL,
}

impl TrainingFailureReason {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::UNAVAILABLE => 0x0,
            Self::NOT_ENOUGH_MONEY => 0x1,
            Self::NOT_ENOUGH_SKILL => 0x2,
        }
    }

}

impl Default for TrainingFailureReason {
    fn default() -> Self {
        Self::UNAVAILABLE
    }
}

impl std::fmt::Display for TrainingFailureReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UNAVAILABLE => f.write_str("UNAVAILABLE"),
            Self::NOT_ENOUGH_MONEY => f.write_str("NOT_ENOUGH_MONEY"),
            Self::NOT_ENOUGH_SKILL => f.write_str("NOT_ENOUGH_SKILL"),
        }
    }
}

impl TryFrom<u32> for TrainingFailureReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::UNAVAILABLE),
            1 => Ok(Self::NOT_ENOUGH_MONEY),
            2 => Ok(Self::NOT_ENOUGH_SKILL),
            v => Err(crate::errors::EnumError::new("TrainingFailureReason", v as u32),)
        }
    }
}

