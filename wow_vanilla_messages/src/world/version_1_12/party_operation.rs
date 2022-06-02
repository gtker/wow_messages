use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_party_command_result.wowm:15`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_party_command_result.wowm#L15):
/// ```text
/// enum PartyOperation : u8 {
///     INVITE = 0;
///     LEAVE = 2;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum PartyOperation {
    INVITE,
    LEAVE,
}

impl PartyOperation {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::INVITE => 0x0,
            Self::LEAVE => 0x2,
        }
    }

}

impl Default for PartyOperation {
    fn default() -> Self {
        Self::INVITE
    }
}

impl std::fmt::Display for PartyOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::INVITE => f.write_str("INVITE"),
            Self::LEAVE => f.write_str("LEAVE"),
        }
    }
}

impl TryFrom<u8> for PartyOperation {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::INVITE),
            2 => Ok(Self::LEAVE),
            v => Err(crate::errors::EnumError::new("PartyOperation", v as u32),)
        }
    }
}

