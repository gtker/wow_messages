use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum GmTicketResponse {
    NOT_EXIST,
    ALREADY_EXIST,
    CREATE_SUCCESS,
    CREATE_ERROR,
    UPDATE_SUCCESS,
    UPDATE_ERROR,
    TICKET_DELETED,
}

impl GmTicketResponse {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::NOT_EXIST => 0x0,
            Self::ALREADY_EXIST => 0x1,
            Self::CREATE_SUCCESS => 0x2,
            Self::CREATE_ERROR => 0x3,
            Self::UPDATE_SUCCESS => 0x4,
            Self::UPDATE_ERROR => 0x5,
            Self::TICKET_DELETED => 0x9,
        }
    }

}

impl Default for GmTicketResponse {
    fn default() -> Self {
        Self::NOT_EXIST
    }
}

impl std::fmt::Display for GmTicketResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NOT_EXIST => f.write_str("NOT_EXIST"),
            Self::ALREADY_EXIST => f.write_str("ALREADY_EXIST"),
            Self::CREATE_SUCCESS => f.write_str("CREATE_SUCCESS"),
            Self::CREATE_ERROR => f.write_str("CREATE_ERROR"),
            Self::UPDATE_SUCCESS => f.write_str("UPDATE_SUCCESS"),
            Self::UPDATE_ERROR => f.write_str("UPDATE_ERROR"),
            Self::TICKET_DELETED => f.write_str("TICKET_DELETED"),
        }
    }
}

impl TryFrom<u32> for GmTicketResponse {
    type Error = GmTicketResponseError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NOT_EXIST),
            1 => Ok(Self::ALREADY_EXIST),
            2 => Ok(Self::CREATE_SUCCESS),
            3 => Ok(Self::CREATE_ERROR),
            4 => Ok(Self::UPDATE_SUCCESS),
            5 => Ok(Self::UPDATE_ERROR),
            9 => Ok(Self::TICKET_DELETED),
            _ => Err(GmTicketResponseError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct GmTicketResponseError {
    value: u32,
}

impl GmTicketResponseError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

impl std::error::Error for GmTicketResponseError {}
impl std::fmt::Display for GmTicketResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'GmTicketResponse': '{}'", self.value))
    }
}

