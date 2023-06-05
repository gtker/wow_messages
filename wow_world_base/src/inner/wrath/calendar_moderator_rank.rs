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

}

#[cfg(feature = "print-testcase")]
impl CalendarModeratorRank {
    pub fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Player => "PLAYER",
            Self::Moderator => "MODERATOR",
            Self::Owner => "OWNER",
        }
    }

}

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
        match value {
            0 => Ok(Self::Player),
            1 => Ok(Self::Moderator),
            2 => Ok(Self::Owner),
            v => Err(crate::errors::EnumError::new("CalendarModeratorRank", v as u64),)
        }
    }
}

