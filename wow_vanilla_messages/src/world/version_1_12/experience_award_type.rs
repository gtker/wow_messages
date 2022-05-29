use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub(crate) enum ExperienceAwardType {
    KILL,
    NON_KILL,
}

impl ExperienceAwardType {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::KILL => 0x0,
            Self::NON_KILL => 0x1,
        }
    }

}

impl Default for ExperienceAwardType {
    fn default() -> Self {
        Self::KILL
    }
}

impl std::fmt::Display for ExperienceAwardType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::KILL => f.write_str("KILL"),
            Self::NON_KILL => f.write_str("NON_KILL"),
        }
    }
}

impl TryFrom<u8> for ExperienceAwardType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::KILL),
            1 => Ok(Self::NON_KILL),
            v => Err(crate::errors::EnumError::new("ExperienceAwardType", v as u32),)
        }
    }
}

