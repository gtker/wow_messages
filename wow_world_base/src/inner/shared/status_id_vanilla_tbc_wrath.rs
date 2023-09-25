/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/smsg_battlefield_status.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/smsg_battlefield_status.wowm#L1):
/// ```text
/// enum StatusId : u8 {
///     NONE = 0;
///     WAIT_QUEUE = 1;
///     WAIT_JOIN = 2;
///     IN_PROGRESS = 3;
///     WAIT_LEAVE = 4;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum StatusId {
    /// first status, should mean bg is not instance
    None,
    /// means bg is empty and waiting for queue
    WaitQueue,
    /// this means, that BG has already started and it is waiting for more players
    WaitJoin,
    /// means bg is running
    InProgress,
    /// means some faction has won BG and it is ending
    WaitLeave,
}

impl StatusId {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::None => 0x0,
            Self::WaitQueue => 0x1,
            Self::WaitJoin => 0x2,
            Self::InProgress => 0x3,
            Self::WaitLeave => 0x4,
        }
    }

    pub const fn variants() -> [Self; 5] {
        [
            Self::None,
            Self::WaitQueue,
            Self::WaitJoin,
            Self::InProgress,
            Self::WaitLeave,
        ]
    }

    pub const fn from_int(value: u8) -> Result<Self, crate::errors::EnumError> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::WaitQueue),
            2 => Ok(Self::WaitJoin),
            3 => Ok(Self::InProgress),
            4 => Ok(Self::WaitLeave),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl StatusId {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::None => "NONE",
            Self::WaitQueue => "WAIT_QUEUE",
            Self::WaitJoin => "WAIT_JOIN",
            Self::InProgress => "IN_PROGRESS",
            Self::WaitLeave => "WAIT_LEAVE",
        }
    }

}

const NAME: &str = "StatusId";

impl Default for StatusId {
    fn default() -> Self {
        Self::None
    }
}

impl std::fmt::Display for StatusId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => f.write_str("None"),
            Self::WaitQueue => f.write_str("WaitQueue"),
            Self::WaitJoin => f.write_str("WaitJoin"),
            Self::InProgress => f.write_str("InProgress"),
            Self::WaitLeave => f.write_str("WaitLeave"),
        }
    }
}

impl TryFrom<u8> for StatusId {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u16> for StatusId {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for StatusId {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for StatusId {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for StatusId {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for StatusId {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for StatusId {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for StatusId {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for StatusId {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

