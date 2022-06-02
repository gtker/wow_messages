use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/smsg_turn_in_petition_results.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/smsg_turn_in_petition_results.wowm#L3):
/// ```text
/// enum PetitionTurnInResult : u32 {
///     OK = 0;
///     ALREADY_SIGNED = 1;
///     ALREADY_IN_GUILD = 2;
///     CANT_SIGN_OWN = 3;
///     NEED_MORE = 4;
///     NOT_SERVER = 5;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum PetitionTurnInResult {
    OK,
    ALREADY_SIGNED,
    ALREADY_IN_GUILD,
    CANT_SIGN_OWN,
    NEED_MORE,
    NOT_SERVER,
}

impl PetitionTurnInResult {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::OK => 0x0,
            Self::ALREADY_SIGNED => 0x1,
            Self::ALREADY_IN_GUILD => 0x2,
            Self::CANT_SIGN_OWN => 0x3,
            Self::NEED_MORE => 0x4,
            Self::NOT_SERVER => 0x5,
        }
    }

}

impl Default for PetitionTurnInResult {
    fn default() -> Self {
        Self::OK
    }
}

impl std::fmt::Display for PetitionTurnInResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OK => f.write_str("OK"),
            Self::ALREADY_SIGNED => f.write_str("ALREADY_SIGNED"),
            Self::ALREADY_IN_GUILD => f.write_str("ALREADY_IN_GUILD"),
            Self::CANT_SIGN_OWN => f.write_str("CANT_SIGN_OWN"),
            Self::NEED_MORE => f.write_str("NEED_MORE"),
            Self::NOT_SERVER => f.write_str("NOT_SERVER"),
        }
    }
}

impl TryFrom<u32> for PetitionTurnInResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::OK),
            1 => Ok(Self::ALREADY_SIGNED),
            2 => Ok(Self::ALREADY_IN_GUILD),
            3 => Ok(Self::CANT_SIGN_OWN),
            4 => Ok(Self::NEED_MORE),
            5 => Ok(Self::NOT_SERVER),
            v => Err(crate::errors::EnumError::new("PetitionTurnInResult", v as u32),)
        }
    }
}

