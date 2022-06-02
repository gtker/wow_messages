use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/smsg_battlefield_status.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/smsg_battlefield_status.wowm#L3):
/// ```text
/// enum StatusId : u8 {
///     NONE = 0;
///     WAIT_QUEUE = 1;
///     WAIT_JOIN = 2;
///     IN_PROGRESS = 3;
///     WAIT_LEAVE = 4;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub(crate) enum StatusId {
    /// # Comment
    /// 
    /// first status, should mean bg is not instance
    NONE,
    /// # Comment
    /// 
    /// means bg is empty and waiting for queue
    WAIT_QUEUE,
    /// # Comment
    /// 
    /// this means, that BG has already started and it is waiting for more players
    WAIT_JOIN,
    /// # Comment
    /// 
    /// means bg is running
    IN_PROGRESS,
    /// # Comment
    /// 
    /// means some faction has won BG and it is ending
    WAIT_LEAVE,
}

impl StatusId {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::NONE => 0x0,
            Self::WAIT_QUEUE => 0x1,
            Self::WAIT_JOIN => 0x2,
            Self::IN_PROGRESS => 0x3,
            Self::WAIT_LEAVE => 0x4,
        }
    }

}

impl Default for StatusId {
    fn default() -> Self {
        Self::NONE
    }
}

impl std::fmt::Display for StatusId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NONE => f.write_str("NONE"),
            Self::WAIT_QUEUE => f.write_str("WAIT_QUEUE"),
            Self::WAIT_JOIN => f.write_str("WAIT_JOIN"),
            Self::IN_PROGRESS => f.write_str("IN_PROGRESS"),
            Self::WAIT_LEAVE => f.write_str("WAIT_LEAVE"),
        }
    }
}

impl TryFrom<u8> for StatusId {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NONE),
            1 => Ok(Self::WAIT_QUEUE),
            2 => Ok(Self::WAIT_JOIN),
            3 => Ok(Self::IN_PROGRESS),
            4 => Ok(Self::WAIT_LEAVE),
            v => Err(crate::errors::EnumError::new("StatusId", v as u32),)
        }
    }
}

