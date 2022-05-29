use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum GroupType {
    NORMAL,
    RAID,
}

impl GroupType {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::NORMAL => 0x0,
            Self::RAID => 0x1,
        }
    }

}

impl Default for GroupType {
    fn default() -> Self {
        Self::NORMAL
    }
}

impl std::fmt::Display for GroupType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NORMAL => f.write_str("NORMAL"),
            Self::RAID => f.write_str("RAID"),
        }
    }
}

impl TryFrom<u8> for GroupType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NORMAL),
            1 => Ok(Self::RAID),
            v => Err(crate::errors::EnumError::new("GroupType", v as u32),)
        }
    }
}

