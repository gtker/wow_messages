use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub(crate) enum CorpseQueryResult {
    NOT_FOUND,
    FOUND,
}

impl CorpseQueryResult {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::NOT_FOUND => 0x0,
            Self::FOUND => 0x1,
        }
    }

}

impl Default for CorpseQueryResult {
    fn default() -> Self {
        Self::NOT_FOUND
    }
}

impl std::fmt::Display for CorpseQueryResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NOT_FOUND => f.write_str("NOT_FOUND"),
            Self::FOUND => f.write_str("FOUND"),
        }
    }
}

impl TryFrom<u8> for CorpseQueryResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NOT_FOUND),
            1 => Ok(Self::FOUND),
            v => Err(crate::errors::EnumError::new("CorpseQueryResult", v as u32),)
        }
    }
}

