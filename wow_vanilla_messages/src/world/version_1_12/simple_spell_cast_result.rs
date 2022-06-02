use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_cast_result.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_cast_result.wowm#L3):
/// ```text
/// enum SimpleSpellCastResult : u8 {
///     SUCCESS = 0;
///     FAILURE = 2;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub(crate) enum SimpleSpellCastResult {
    SUCCESS,
    FAILURE,
}

impl SimpleSpellCastResult {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::SUCCESS => 0x0,
            Self::FAILURE => 0x2,
        }
    }

}

impl Default for SimpleSpellCastResult {
    fn default() -> Self {
        Self::SUCCESS
    }
}

impl std::fmt::Display for SimpleSpellCastResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SUCCESS => f.write_str("SUCCESS"),
            Self::FAILURE => f.write_str("FAILURE"),
        }
    }
}

impl TryFrom<u8> for SimpleSpellCastResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::SUCCESS),
            2 => Ok(Self::FAILURE),
            v => Err(crate::errors::EnumError::new("SimpleSpellCastResult", v as u32),)
        }
    }
}

