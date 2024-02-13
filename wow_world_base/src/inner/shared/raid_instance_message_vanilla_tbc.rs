/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/smsg_raid_instance_message.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/smsg_raid_instance_message.wowm#L1):
/// ```text
/// enum RaidInstanceMessage : u32 {
///     WARNING_HOURS = 1;
///     WARNING_MIN = 2;
///     WARNING_MIN_SOON = 3;
///     WELCOME = 4;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum RaidInstanceMessage {
    /// WARNING! %s is scheduled to reset in %d hour(s).
    WarningHours,
    /// WARNING! %s is scheduled to reset in %d minute(s)!
    WarningMin,
    /// WARNING! %s is scheduled to reset in %d minute(s). Please exit the zone or you will be returned to your bind location!
    WarningMinSoon,
    /// Welcome to %s. This raid instance is scheduled to reset in %s.
    Welcome,
}

impl RaidInstanceMessage {
    pub const fn as_int(&self) -> u32 {
        match self {
            Self::WarningHours => 0x1,
            Self::WarningMin => 0x2,
            Self::WarningMinSoon => 0x3,
            Self::Welcome => 0x4,
        }
    }

    pub const fn variants() -> [Self; 4] {
        [
            Self::WarningHours,
            Self::WarningMin,
            Self::WarningMinSoon,
            Self::Welcome,
        ]
    }

    pub const fn from_int(value: u32) -> Result<Self, crate::errors::EnumError> {
        match value {
            1 => Ok(Self::WarningHours),
            2 => Ok(Self::WarningMin),
            3 => Ok(Self::WarningMinSoon),
            4 => Ok(Self::Welcome),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl RaidInstanceMessage {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::WarningHours => "WARNING_HOURS",
            Self::WarningMin => "WARNING_MIN",
            Self::WarningMinSoon => "WARNING_MIN_SOON",
            Self::Welcome => "WELCOME",
        }
    }

}

const NAME: &str = "RaidInstanceMessage";

impl Default for RaidInstanceMessage {
    fn default() -> Self {
        Self::WarningHours
    }
}

impl std::fmt::Display for RaidInstanceMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WarningHours => f.write_str("WarningHours"),
            Self::WarningMin => f.write_str("WarningMin"),
            Self::WarningMinSoon => f.write_str("WarningMinSoon"),
            Self::Welcome => f.write_str("Welcome"),
        }
    }
}

impl TryFrom<u32> for RaidInstanceMessage {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u8> for RaidInstanceMessage {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value.into())
    }
}

impl TryFrom<u16> for RaidInstanceMessage {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::from_int(value.into())
    }
}

impl TryFrom<u64> for RaidInstanceMessage {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for RaidInstanceMessage {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for RaidInstanceMessage {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for RaidInstanceMessage {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let v = u32::from_le_bytes(value.to_le_bytes());
        Self::from_int(v)
    }
}

impl TryFrom<i64> for RaidInstanceMessage {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for RaidInstanceMessage {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

