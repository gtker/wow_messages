use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_group_joined_battleground.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_group_joined_battleground.wowm#L3):
/// ```text
/// enum BgTypeId : u32 {
///     NOT_ELIGIBLE = 0;
///     QUEUED_FOR_AV = 1;
///     QUEUED_FOR_WSG = 2;
///     QUEUED_FOR_AB = 3;
///     REMOVE_FROM_QUEUE = 0xFFFFFFFE;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum BgTypeId {
    /// # Comment
    /// 
    /// Your group has joined a battleground queue, but you are not eligible
    NOT_ELIGIBLE,
    /// # Comment
    /// 
    /// Your group has joined the queue for AV
    QUEUED_FOR_AV,
    /// # Comment
    /// 
    /// Your group has joined the queue for WS
    QUEUED_FOR_WSG,
    /// # Comment
    /// 
    /// Your group has joined the queue for AB
    QUEUED_FOR_AB,
    /// # Comment
    /// 
    /// send bg command result to show nice message
    REMOVE_FROM_QUEUE,
}

impl BgTypeId {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::NOT_ELIGIBLE => 0x0,
            Self::QUEUED_FOR_AV => 0x1,
            Self::QUEUED_FOR_WSG => 0x2,
            Self::QUEUED_FOR_AB => 0x3,
            Self::REMOVE_FROM_QUEUE => 0xfffffffe,
        }
    }

}

impl Default for BgTypeId {
    fn default() -> Self {
        Self::NOT_ELIGIBLE
    }
}

impl std::fmt::Display for BgTypeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NOT_ELIGIBLE => f.write_str("NOT_ELIGIBLE"),
            Self::QUEUED_FOR_AV => f.write_str("QUEUED_FOR_AV"),
            Self::QUEUED_FOR_WSG => f.write_str("QUEUED_FOR_WSG"),
            Self::QUEUED_FOR_AB => f.write_str("QUEUED_FOR_AB"),
            Self::REMOVE_FROM_QUEUE => f.write_str("REMOVE_FROM_QUEUE"),
        }
    }
}

impl TryFrom<u32> for BgTypeId {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NOT_ELIGIBLE),
            1 => Ok(Self::QUEUED_FOR_AV),
            2 => Ok(Self::QUEUED_FOR_WSG),
            3 => Ok(Self::QUEUED_FOR_AB),
            4294967294 => Ok(Self::REMOVE_FROM_QUEUE),
            v => Err(crate::errors::EnumError::new("BgTypeId", v as u32),)
        }
    }
}

