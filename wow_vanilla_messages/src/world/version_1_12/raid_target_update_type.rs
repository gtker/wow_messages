use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub(crate) enum RaidTargetUpdateType {
    PARTIAL,
    FULL,
}

impl RaidTargetUpdateType {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::PARTIAL => 0x0,
            Self::FULL => 0x1,
        }
    }

}

impl Default for RaidTargetUpdateType {
    fn default() -> Self {
        Self::PARTIAL
    }
}

impl std::fmt::Display for RaidTargetUpdateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PARTIAL => f.write_str("PARTIAL"),
            Self::FULL => f.write_str("FULL"),
        }
    }
}

impl TryFrom<u8> for RaidTargetUpdateType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::PARTIAL),
            1 => Ok(Self::FULL),
            v => Err(crate::errors::EnumError::new("RaidTargetUpdateType", v as u32),)
        }
    }
}

