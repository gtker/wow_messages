use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/cmsg_calendar_event_rsvp.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/cmsg_calendar_event_rsvp.wowm#L1):
/// ```text
/// enum CalendarStatus : u32 {
///     INVITED = 0;
///     ACCEPTED = 1;
///     DECLINED = 2;
///     CONFIRMED = 3;
///     OUT = 4;
///     STANDBY = 5;
///     SIGNED_UP = 6;
///     NOT_SIGNED_UP = 7;
///     TENTATIVE = 8;
///     REMOVED = 9;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum CalendarStatus {
    Invited,
    Accepted,
    Declined,
    Confirmed,
    Out,
    Standby,
    SignedUp,
    NotSignedUp,
    Tentative,
    Removed,
}

impl CalendarStatus {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::Invited => 0x0,
            Self::Accepted => 0x1,
            Self::Declined => 0x2,
            Self::Confirmed => 0x3,
            Self::Out => 0x4,
            Self::Standby => 0x5,
            Self::SignedUp => 0x6,
            Self::NotSignedUp => 0x7,
            Self::Tentative => 0x8,
            Self::Removed => 0x9,
        }
    }

}

impl Default for CalendarStatus {
    fn default() -> Self {
        Self::Invited
    }
}

impl std::fmt::Display for CalendarStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invited => f.write_str("Invited"),
            Self::Accepted => f.write_str("Accepted"),
            Self::Declined => f.write_str("Declined"),
            Self::Confirmed => f.write_str("Confirmed"),
            Self::Out => f.write_str("Out"),
            Self::Standby => f.write_str("Standby"),
            Self::SignedUp => f.write_str("SignedUp"),
            Self::NotSignedUp => f.write_str("NotSignedUp"),
            Self::Tentative => f.write_str("Tentative"),
            Self::Removed => f.write_str("Removed"),
        }
    }
}

impl TryFrom<u32> for CalendarStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Invited),
            1 => Ok(Self::Accepted),
            2 => Ok(Self::Declined),
            3 => Ok(Self::Confirmed),
            4 => Ok(Self::Out),
            5 => Ok(Self::Standby),
            6 => Ok(Self::SignedUp),
            7 => Ok(Self::NotSignedUp),
            8 => Ok(Self::Tentative),
            9 => Ok(Self::Removed),
            v => Err(crate::errors::EnumError::new("CalendarStatus", v as u32),)
        }
    }
}

