/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/calendar/cmsg_calendar_event_rsvp.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/calendar/cmsg_calendar_event_rsvp.wowm#L1):
/// ```text
/// enum CalendarStatus : u8 {
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
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
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
    pub const fn as_int(&self) -> u8 {
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

    pub const fn variants() -> [Self; 10] {
        [
            Self::Invited,
            Self::Accepted,
            Self::Declined,
            Self::Confirmed,
            Self::Out,
            Self::Standby,
            Self::SignedUp,
            Self::NotSignedUp,
            Self::Tentative,
            Self::Removed,
        ]
    }

    pub const fn from_int(value: u8) -> Result<Self, crate::errors::EnumError> {
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
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl CalendarStatus {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Invited => "INVITED",
            Self::Accepted => "ACCEPTED",
            Self::Declined => "DECLINED",
            Self::Confirmed => "CONFIRMED",
            Self::Out => "OUT",
            Self::Standby => "STANDBY",
            Self::SignedUp => "SIGNED_UP",
            Self::NotSignedUp => "NOT_SIGNED_UP",
            Self::Tentative => "TENTATIVE",
            Self::Removed => "REMOVED",
        }
    }

}

const NAME: &str = "CalendarStatus";

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

impl TryFrom<u8> for CalendarStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u16> for CalendarStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for CalendarStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for CalendarStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for CalendarStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let v = u8::from_le_bytes(value.to_le_bytes());
        Self::from_int(v)
    }
}

impl TryFrom<i16> for CalendarStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for CalendarStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for CalendarStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for CalendarStatus {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

