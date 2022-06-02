use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/smsg_guild_event.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/smsg_guild_event.wowm#L3):
/// ```text
/// enum GuildEvent : u8 {
///     PROMOTION = 0;
///     DEMOTION = 1;
///     MOTD = 2;
///     JOINED = 3;
///     LEFT = 4;
///     REMOVED = 5;
///     LEADER_IS = 6;
///     LEADER_CHANGED = 7;
///     DISBANDED = 8;
///     TABARD_CHANGED = 9;
///     UNKNOWN10 = 10;
///     ROSTER_UPDATE = 11;
///     SIGNED_ON = 12;
///     SIGNED_OFF = 13;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum GuildEvent {
    PROMOTION,
    DEMOTION,
    MOTD,
    JOINED,
    LEFT,
    REMOVED,
    LEADER_IS,
    LEADER_CHANGED,
    DISBANDED,
    TABARD_CHANGED,
    UNKNOWN10,
    ROSTER_UPDATE,
    SIGNED_ON,
    SIGNED_OFF,
}

impl GuildEvent {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::PROMOTION => 0x0,
            Self::DEMOTION => 0x1,
            Self::MOTD => 0x2,
            Self::JOINED => 0x3,
            Self::LEFT => 0x4,
            Self::REMOVED => 0x5,
            Self::LEADER_IS => 0x6,
            Self::LEADER_CHANGED => 0x7,
            Self::DISBANDED => 0x8,
            Self::TABARD_CHANGED => 0x9,
            Self::UNKNOWN10 => 0xa,
            Self::ROSTER_UPDATE => 0xb,
            Self::SIGNED_ON => 0xc,
            Self::SIGNED_OFF => 0xd,
        }
    }

}

impl Default for GuildEvent {
    fn default() -> Self {
        Self::PROMOTION
    }
}

impl std::fmt::Display for GuildEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PROMOTION => f.write_str("PROMOTION"),
            Self::DEMOTION => f.write_str("DEMOTION"),
            Self::MOTD => f.write_str("MOTD"),
            Self::JOINED => f.write_str("JOINED"),
            Self::LEFT => f.write_str("LEFT"),
            Self::REMOVED => f.write_str("REMOVED"),
            Self::LEADER_IS => f.write_str("LEADER_IS"),
            Self::LEADER_CHANGED => f.write_str("LEADER_CHANGED"),
            Self::DISBANDED => f.write_str("DISBANDED"),
            Self::TABARD_CHANGED => f.write_str("TABARD_CHANGED"),
            Self::UNKNOWN10 => f.write_str("UNKNOWN10"),
            Self::ROSTER_UPDATE => f.write_str("ROSTER_UPDATE"),
            Self::SIGNED_ON => f.write_str("SIGNED_ON"),
            Self::SIGNED_OFF => f.write_str("SIGNED_OFF"),
        }
    }
}

impl TryFrom<u8> for GuildEvent {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::PROMOTION),
            1 => Ok(Self::DEMOTION),
            2 => Ok(Self::MOTD),
            3 => Ok(Self::JOINED),
            4 => Ok(Self::LEFT),
            5 => Ok(Self::REMOVED),
            6 => Ok(Self::LEADER_IS),
            7 => Ok(Self::LEADER_CHANGED),
            8 => Ok(Self::DISBANDED),
            9 => Ok(Self::TABARD_CHANGED),
            10 => Ok(Self::UNKNOWN10),
            11 => Ok(Self::ROSTER_UPDATE),
            12 => Ok(Self::SIGNED_ON),
            13 => Ok(Self::SIGNED_OFF),
            v => Err(crate::errors::EnumError::new("GuildEvent", v as u32),)
        }
    }
}

