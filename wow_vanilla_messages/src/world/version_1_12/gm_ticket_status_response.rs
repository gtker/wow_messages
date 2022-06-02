use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gamemaster/smsg_gm_ticket_status_update.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gamemaster/smsg_gm_ticket_status_update.wowm#L3):
/// ```text
/// enum GmTicketStatusResponse : u32 {
///     UPDATED = 1;
///     CLOSED = 2;
///     SURVEY = 3;
/// }

/// ```
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
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::UPDATED),
            2 => Ok(Self::CLOSED),
            3 => Ok(Self::SURVEY),
            v => Err(crate::errors::EnumError::new("GmTicketStatusResponse", v as u32),)
        }
    }
}

