use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum GmTicketStatusResponse {
    UPDATED,
    CLOSED,
    SURVEY,
}

impl GmTicketStatusResponse {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::UPDATED => 0x1,
            Self::CLOSED => 0x2,
            Self::SURVEY => 0x3,
        }
    }

}

impl Default for GmTicketStatusResponse {
    fn default() -> Self {
        Self::UPDATED
    }
}

impl std::fmt::Display for GmTicketStatusResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UPDATED => f.write_str("UPDATED"),
            Self::CLOSED => f.write_str("CLOSED"),
            Self::SURVEY => f.write_str("SURVEY"),
        }
    }
}

impl TryFrom<u32> for GmTicketStatusResponse {
    type Error = GmTicketStatusResponseError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::UPDATED),
            2 => Ok(Self::CLOSED),
            3 => Ok(Self::SURVEY),
            _ => Err(GmTicketStatusResponseError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct GmTicketStatusResponseError {
    pub value: u32,
}

impl GmTicketStatusResponseError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

impl std::error::Error for GmTicketStatusResponseError {}
impl std::fmt::Display for GmTicketStatusResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'GmTicketStatusResponse': '{}'", self.value))
    }
}

