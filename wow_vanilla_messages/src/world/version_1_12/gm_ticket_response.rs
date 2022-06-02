use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gamemaster/gamemaster_common.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gamemaster/gamemaster_common.wowm#L3):
/// ```text
/// enum GmTicketResponse : u32 {
///     NOT_EXIST = 0;
///     ALREADY_EXIST = 1;
///     CREATE_SUCCESS = 2;
///     CREATE_ERROR = 3;
///     UPDATE_SUCCESS = 4;
///     UPDATE_ERROR = 5;
///     TICKET_DELETED = 9;
/// }

/// ```
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
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NOT_EXIST),
            1 => Ok(Self::ALREADY_EXIST),
            2 => Ok(Self::CREATE_SUCCESS),
            3 => Ok(Self::CREATE_ERROR),
            4 => Ok(Self::UPDATE_SUCCESS),
            5 => Ok(Self::UPDATE_ERROR),
            9 => Ok(Self::TICKET_DELETED),
            v => Err(crate::errors::EnumError::new("GmTicketResponse", v as u32),)
        }
    }
}

