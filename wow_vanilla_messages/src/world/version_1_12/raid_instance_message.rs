use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/smsg_raid_instance_message.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/smsg_raid_instance_message.wowm#L3):
/// ```text
/// enum RaidInstanceMessage : u32 {
///     WARNING_HOURS = 1;
///     WARNING_MIN = 2;
///     WARNING_MIN_SOON = 3;
///     WELCOME = 4;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum RaidInstanceMessage {
    /// # Comment
    /// 
    /// WARNING! %s is scheduled to reset in %d hour(s).
    WARNING_HOURS,
    /// # Comment
    /// 
    /// WARNING! %s is scheduled to reset in %d minute(s)!
    WARNING_MIN,
    /// # Comment
    /// 
    /// WARNING! %s is scheduled to reset in %d minute(s). Please exit the zone or you will be returned to your bind location!
    WARNING_MIN_SOON,
    /// # Comment
    /// 
    /// Welcome to %s. This raid instance is scheduled to reset in %s.
    WELCOME,
}

impl RaidInstanceMessage {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::WARNING_HOURS => 0x1,
            Self::WARNING_MIN => 0x2,
            Self::WARNING_MIN_SOON => 0x3,
            Self::WELCOME => 0x4,
        }
    }

}

impl Default for RaidInstanceMessage {
    fn default() -> Self {
        Self::WARNING_HOURS
    }
}

impl std::fmt::Display for RaidInstanceMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WARNING_HOURS => f.write_str("WARNING_HOURS"),
            Self::WARNING_MIN => f.write_str("WARNING_MIN"),
            Self::WARNING_MIN_SOON => f.write_str("WARNING_MIN_SOON"),
            Self::WELCOME => f.write_str("WELCOME"),
        }
    }
}

impl TryFrom<u32> for RaidInstanceMessage {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::WARNING_HOURS),
            2 => Ok(Self::WARNING_MIN),
            3 => Ok(Self::WARNING_MIN_SOON),
            4 => Ok(Self::WELCOME),
            v => Err(crate::errors::EnumError::new("RaidInstanceMessage", v as u32),)
        }
    }
}

