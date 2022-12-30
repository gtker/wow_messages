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
    Updated,
    Closed,
    Survey,
}

impl GmTicketStatusResponse {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::Updated => 0x1,
            Self::Closed => 0x2,
            Self::Survey => 0x3,
        }
    }

}

impl Default for GmTicketStatusResponse {
    fn default() -> Self {
        Self::Updated
    }
}

impl std::fmt::Display for GmTicketStatusResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Updated => f.write_str("Updated"),
            Self::Closed => f.write_str("Closed"),
            Self::Survey => f.write_str("Survey"),
        }
    }
}

impl TryFrom<u32> for GmTicketStatusResponse {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Updated),
            2 => Ok(Self::Closed),
            3 => Ok(Self::Survey),
            v => Err(crate::errors::EnumError::new("GmTicketStatusResponse", v as u64),)
        }
    }
}

