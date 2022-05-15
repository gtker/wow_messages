use std::convert::{TryFrom, TryInto};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

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
    type Error = GmTicketStatusError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::DBERROR),
            6 => Ok(Self::HASTEXT),
            10 => Ok(Self::DEFAULT),
            _ => Err(GmTicketStatusError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct GmTicketStatusError {
    pub value: u32,
}

impl GmTicketStatusError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

impl std::error::Error for GmTicketStatusError {}
impl std::fmt::Display for GmTicketStatusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'GmTicketStatus': '{}'", self.value))
    }
}

