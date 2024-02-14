/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/calendar/cmsg_calendar_event_moderator_status.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/calendar/cmsg_calendar_event_moderator_status.wowm#L1):
/// ```text
/// enum CalendarModeratorRank : u8 {
///     PLAYER = 0;
///     MODERATOR = 1;
///     OWNER = 2;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum CalendarModeratorRank {
    Player,
    Moderator,
    Owner,
}

impl CalendarModeratorRank {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Player => 0x0,
            Self::Moderator => 0x1,
            Self::Owner => 0x2,
        }
    }

    pub const fn variants() -> [Self; 3] {
        [
            Self::Player,
            Self::Moderator,
            Self::Owner,
        ]
    }

    pub const fn from_int(value: u8) -> Result<Self, crate::errors::EnumError> {
        match value {
            0 => Ok(Self::Player),
            1 => Ok(Self::Moderator),
            2 => Ok(Self::Owner),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl CalendarModeratorRank {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Player => "PLAYER",
            Self::Moderator => "MODERATOR",
            Self::Owner => "OWNER",
        }
    }

}

const NAME: &str = "CalendarModeratorRank";

impl Default for CalendarModeratorRank {
    fn default() -> Self {
        Self::Player
    }
}

impl std::fmt::Display for CalendarModeratorRank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Player => f.write_str("Player"),
            Self::Moderator => f.write_str("Moderator"),
            Self::Owner => f.write_str("Owner"),
        }
    }
}

impl TryFrom<u8> for CalendarModeratorRank {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u16> for CalendarModeratorRank {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for CalendarModeratorRank {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for CalendarModeratorRank {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for CalendarModeratorRank {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let v = u8::from_le_bytes(value.to_le_bytes());
        Self::from_int(v)
    }
}

impl TryFrom<i16> for CalendarModeratorRank {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for CalendarModeratorRank {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for CalendarModeratorRank {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for CalendarModeratorRank {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

