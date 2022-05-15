use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum GmTicketType {
    STUCK,
    BEHAVIOR_HARASSMENT,
    GUILD,
    ITEM,
    ENVIRONMENTAL,
    NONQUEST_CREEP,
    QUEST_QUESTNPC,
    TECHNICAL,
    ACCOUNT_BILLING,
    CHARACTER,
}

impl GmTicketType {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::STUCK => 0x1,
            Self::BEHAVIOR_HARASSMENT => 0x2,
            Self::GUILD => 0x3,
            Self::ITEM => 0x4,
            Self::ENVIRONMENTAL => 0x5,
            Self::NONQUEST_CREEP => 0x6,
            Self::QUEST_QUESTNPC => 0x7,
            Self::TECHNICAL => 0x8,
            Self::ACCOUNT_BILLING => 0x9,
            Self::CHARACTER => 0xa,
        }
    }

}

impl Default for GmTicketType {
    fn default() -> Self {
        Self::STUCK
    }
}

impl std::fmt::Display for GmTicketType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::STUCK => f.write_str("STUCK"),
            Self::BEHAVIOR_HARASSMENT => f.write_str("BEHAVIOR_HARASSMENT"),
            Self::GUILD => f.write_str("GUILD"),
            Self::ITEM => f.write_str("ITEM"),
            Self::ENVIRONMENTAL => f.write_str("ENVIRONMENTAL"),
            Self::NONQUEST_CREEP => f.write_str("NONQUEST_CREEP"),
            Self::QUEST_QUESTNPC => f.write_str("QUEST_QUESTNPC"),
            Self::TECHNICAL => f.write_str("TECHNICAL"),
            Self::ACCOUNT_BILLING => f.write_str("ACCOUNT_BILLING"),
            Self::CHARACTER => f.write_str("CHARACTER"),
        }
    }
}

impl TryFrom<u8> for GmTicketType {
    type Error = GmTicketTypeError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::STUCK),
            2 => Ok(Self::BEHAVIOR_HARASSMENT),
            3 => Ok(Self::GUILD),
            4 => Ok(Self::ITEM),
            5 => Ok(Self::ENVIRONMENTAL),
            6 => Ok(Self::NONQUEST_CREEP),
            7 => Ok(Self::QUEST_QUESTNPC),
            8 => Ok(Self::TECHNICAL),
            9 => Ok(Self::ACCOUNT_BILLING),
            10 => Ok(Self::CHARACTER),
            _ => Err(GmTicketTypeError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct GmTicketTypeError {
    value: u8,
}

impl GmTicketTypeError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

impl std::error::Error for GmTicketTypeError {}
impl std::fmt::Display for GmTicketTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'GmTicketType': '{}'", self.value))
    }
}

