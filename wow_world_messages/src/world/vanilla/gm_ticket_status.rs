use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gamemaster/gamemaster_common.wowm:31`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gamemaster/gamemaster_common.wowm#L31):
/// ```text
/// enum GmTicketStatus : u32 {
///     DBERROR = 0x00;
///     HASTEXT = 0x06;
///     DEFAULT = 0x0A;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub(crate) enum GmTicketStatus {
    Dberror,
    Hastext,
    Default,
}

impl GmTicketStatus {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::Dberror => 0x0,
            Self::Hastext => 0x6,
            Self::Default => 0xa,
        }
    }

}

impl Default for GmTicketStatus {
    fn default() -> Self {
        Self::Dberror
    }
}

impl std::fmt::Display for GmTicketStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Dberror => f.write_str("Dberror"),
            Self::Hastext => f.write_str("Hastext"),
            Self::Default => f.write_str("Default"),
        }
    }
}

impl TryFrom<u32> for GmTicketStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Dberror),
            6 => Ok(Self::Hastext),
            10 => Ok(Self::Default),
            v => Err(crate::errors::EnumError::new("GmTicketStatus", v as u32),)
        }
    }
}

