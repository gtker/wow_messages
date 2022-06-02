use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_raid_group_only.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_raid_group_only.wowm#L3):
/// ```text
/// enum RaidGroupError : u32 {
///     REQUIRED = 1;
///     FULL = 2;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum RaidGroupError {
    REQUIRED,
    FULL,
}

impl RaidGroupError {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::REQUIRED => 0x1,
            Self::FULL => 0x2,
        }
    }

}

impl Default for RaidGroupError {
    fn default() -> Self {
        Self::REQUIRED
    }
}

impl std::fmt::Display for RaidGroupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::REQUIRED => f.write_str("REQUIRED"),
            Self::FULL => f.write_str("FULL"),
        }
    }
}

impl TryFrom<u32> for RaidGroupError {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::REQUIRED),
            2 => Ok(Self::FULL),
            v => Err(crate::errors::EnumError::new("RaidGroupError", v as u32),)
        }
    }
}

