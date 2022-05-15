use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum PlayerChatTag {
    NONE,
    AFK,
    DND,
    GM,
}

impl PlayerChatTag {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::NONE => 0x0,
            Self::AFK => 0x1,
            Self::DND => 0x2,
            Self::GM => 0x3,
        }
    }

}

impl Default for PlayerChatTag {
    fn default() -> Self {
        Self::NONE
    }
}

impl std::fmt::Display for PlayerChatTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NONE => f.write_str("NONE"),
            Self::AFK => f.write_str("AFK"),
            Self::DND => f.write_str("DND"),
            Self::GM => f.write_str("GM"),
        }
    }
}

impl TryFrom<u8> for PlayerChatTag {
    type Error = PlayerChatTagError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NONE),
            1 => Ok(Self::AFK),
            2 => Ok(Self::DND),
            3 => Ok(Self::GM),
            _ => Err(PlayerChatTagError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct PlayerChatTagError {
    value: u8,
}

impl PlayerChatTagError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

impl std::error::Error for PlayerChatTagError {}
impl std::fmt::Display for PlayerChatTagError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'PlayerChatTag': '{}'", self.value))
    }
}

