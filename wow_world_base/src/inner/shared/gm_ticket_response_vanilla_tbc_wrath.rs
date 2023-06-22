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
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
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
    pub const fn as_int(&self) -> u32 {
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

#[cfg(feature = "print-testcase")]
impl GmTicketResponse {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::NotExist => "NOT_EXIST",
            Self::AlreadyExist => "ALREADY_EXIST",
            Self::CreateSuccess => "CREATE_SUCCESS",
            Self::CreateError => "CREATE_ERROR",
            Self::UpdateSuccess => "UPDATE_SUCCESS",
            Self::UpdateError => "UPDATE_ERROR",
            Self::TicketDeleted => "TICKET_DELETED",
        }
    }

}

const NAME: &str = "GmTicketResponse";

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
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NotExist),
            1 => Ok(Self::AlreadyExist),
            2 => Ok(Self::CreateSuccess),
            3 => Ok(Self::CreateError),
            4 => Ok(Self::UpdateSuccess),
            5 => Ok(Self::UpdateError),
            9 => Ok(Self::TicketDeleted),
            v => Err(crate::errors::EnumError::new(NAME, v.into()),)
        }
    }
}

impl TryFrom<u8> for GmTicketResponse {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u16> for GmTicketResponse {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for GmTicketResponse {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for GmTicketResponse {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for GmTicketResponse {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for GmTicketResponse {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for GmTicketResponse {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for GmTicketResponse {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

