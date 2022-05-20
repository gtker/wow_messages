use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum NewItemSource {
    LOOTED,
    FROM_NPC,
}

impl NewItemSource {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::LOOTED => 0x0,
            Self::FROM_NPC => 0x1,
        }
    }

}

impl Default for NewItemSource {
    fn default() -> Self {
        Self::LOOTED
    }
}

impl std::fmt::Display for NewItemSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LOOTED => f.write_str("LOOTED"),
            Self::FROM_NPC => f.write_str("FROM_NPC"),
        }
    }
}

impl TryFrom<u32> for NewItemSource {
    type Error = NewItemSourceError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::LOOTED),
            1 => Ok(Self::FROM_NPC),
            _ => Err(NewItemSourceError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct NewItemSourceError {
    pub value: u32,
}

impl NewItemSourceError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

impl std::error::Error for NewItemSourceError {}
impl std::fmt::Display for NewItemSourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'NewItemSource': '{}'", self.value))
    }
}

