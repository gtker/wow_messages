use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/quest_common.wowm:36`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/quest_common.wowm#L36):
/// ```text
/// enum QuestGiverStatus : u8 {
///     NONE = 0;
///     UNAVAILABLE = 1;
///     CHAT = 2;
///     INCOMPLETE = 3;
///     REWARD_REP = 4;
///     AVAILABLE = 5;
///     REWARD_OLD = 6;
///     REWARD2 = 7;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum QuestGiverStatus {
    NONE,
    UNAVAILABLE,
    CHAT,
    INCOMPLETE,
    REWARD_REP,
    AVAILABLE,
    /// # Comment
    /// 
    /// red dot on minimap
    REWARD_OLD,
    /// # Comment
    /// 
    /// yellow dot on minimap
    REWARD2,
}

impl QuestGiverStatus {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::NONE => 0x0,
            Self::UNAVAILABLE => 0x1,
            Self::CHAT => 0x2,
            Self::INCOMPLETE => 0x3,
            Self::REWARD_REP => 0x4,
            Self::AVAILABLE => 0x5,
            Self::REWARD_OLD => 0x6,
            Self::REWARD2 => 0x7,
        }
    }

}

impl Default for QuestGiverStatus {
    fn default() -> Self {
        Self::NONE
    }
}

impl std::fmt::Display for QuestGiverStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NONE => f.write_str("NONE"),
            Self::UNAVAILABLE => f.write_str("UNAVAILABLE"),
            Self::CHAT => f.write_str("CHAT"),
            Self::INCOMPLETE => f.write_str("INCOMPLETE"),
            Self::REWARD_REP => f.write_str("REWARD_REP"),
            Self::AVAILABLE => f.write_str("AVAILABLE"),
            Self::REWARD_OLD => f.write_str("REWARD_OLD"),
            Self::REWARD2 => f.write_str("REWARD2"),
        }
    }
}

impl TryFrom<u8> for QuestGiverStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NONE),
            1 => Ok(Self::UNAVAILABLE),
            2 => Ok(Self::CHAT),
            3 => Ok(Self::INCOMPLETE),
            4 => Ok(Self::REWARD_REP),
            5 => Ok(Self::AVAILABLE),
            6 => Ok(Self::REWARD_OLD),
            7 => Ok(Self::REWARD2),
            v => Err(crate::errors::EnumError::new("QuestGiverStatus", v as u32),)
        }
    }
}

