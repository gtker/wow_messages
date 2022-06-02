use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gamemaster/gamemaster_common.wowm:29`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gamemaster/gamemaster_common.wowm#L29):
/// ```text
/// enum GmTicketStatus : u32 {
///     DBERROR = 0x00;
///     HASTEXT = 0x06;
///     DEFAULT = 0x0A;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub(crate) enum GmTicketStatus {
    DBERROR,
    HASTEXT,
    DEFAULT,
}

impl GmTicketStatus {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::DBERROR => 0x0,
            Self::HASTEXT => 0x6,
            Self::DEFAULT => 0xa,
        }
    }

}

impl Default for GmTicketStatus {
    fn default() -> Self {
        Self::DBERROR
    }
}

impl std::fmt::Display for GmTicketStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DBERROR => f.write_str("DBERROR"),
            Self::HASTEXT => f.write_str("HASTEXT"),
            Self::DEFAULT => f.write_str("DEFAULT"),
        }
    }
}

impl TryFrom<u32> for GmTicketStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::DBERROR),
            6 => Ok(Self::HASTEXT),
            10 => Ok(Self::DEFAULT),
            v => Err(crate::errors::EnumError::new("GmTicketStatus", v as u32),)
        }
    }
}

