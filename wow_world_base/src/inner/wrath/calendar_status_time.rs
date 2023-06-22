/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/calendar/smsg_calendar_event_invite.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/calendar/smsg_calendar_event_invite.wowm#L1):
/// ```text
/// enum CalendarStatusTime : u8 {
///     NOT_PRESENT = 0;
///     PRESENT = 1;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum CalendarStatusTime {
    NotPresent,
    Present,
}

impl CalendarStatusTime {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::NotPresent => 0x0,
            Self::Present => 0x1,
        }
    }

}

#[cfg(feature = "print-testcase")]
impl CalendarStatusTime {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::NotPresent => "NOT_PRESENT",
            Self::Present => "PRESENT",
        }
    }

}

impl Default for CalendarStatusTime {
    fn default() -> Self {
        Self::NotPresent
    }
}

impl std::fmt::Display for CalendarStatusTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotPresent => f.write_str("NotPresent"),
            Self::Present => f.write_str("Present"),
        }
    }
}

impl TryFrom<u8> for CalendarStatusTime {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NotPresent),
            1 => Ok(Self::Present),
            v => Err(crate::errors::EnumError::new("CalendarStatusTime", v.into()),)
        }
    }
}

