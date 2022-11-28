use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gamemaster/gamemaster_common.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gamemaster/gamemaster_common.wowm#L1):
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
    NotExist,
    AlreadyExist,
    CreateSuccess,
    CreateError,
    UpdateSuccess,
    UpdateError,
    TicketDeleted,
}

impl GmTicketResponse {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::NotExist => 0x0,
            Self::AlreadyExist => 0x1,
            Self::CreateSuccess => 0x2,
            Self::CreateError => 0x3,
            Self::UpdateSuccess => 0x4,
            Self::UpdateError => 0x5,
            Self::TicketDeleted => 0x9,
        }
    }

}

impl Default for GmTicketResponse {
    fn default() -> Self {
        Self::NotExist
    }
}

impl std::fmt::Display for GmTicketResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotExist => f.write_str("NotExist"),
            Self::AlreadyExist => f.write_str("AlreadyExist"),
            Self::CreateSuccess => f.write_str("CreateSuccess"),
            Self::CreateError => f.write_str("CreateError"),
            Self::UpdateSuccess => f.write_str("UpdateSuccess"),
            Self::UpdateError => f.write_str("UpdateError"),
            Self::TicketDeleted => f.write_str("TicketDeleted"),
        }
    }
}

impl TryFrom<u32> for GmTicketResponse {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NotExist),
            1 => Ok(Self::AlreadyExist),
            2 => Ok(Self::CreateSuccess),
            3 => Ok(Self::CreateError),
            4 => Ok(Self::UpdateSuccess),
            5 => Ok(Self::UpdateError),
            9 => Ok(Self::TicketDeleted),
            v => Err(crate::errors::EnumError::new("GmTicketResponse", v as u32),)
        }
    }
}

