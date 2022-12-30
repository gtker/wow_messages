use std::convert::{TryFrom, TryInto};

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
pub enum RaidInstanceMessage {
    /// WARNING! %s is scheduled to reset in %d hour(s).
    ///
    WarningHours,
    /// WARNING! %s is scheduled to reset in %d minute(s)!
    ///
    WarningMin,
    /// WARNING! %s is scheduled to reset in %d minute(s). Please exit the zone or you will be returned to your bind location!
    ///
    WarningMinSoon,
    /// Welcome to %s. This raid instance is scheduled to reset in %s.
    ///
    Welcome,
}

impl RaidInstanceMessage {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::WarningHours => 0x1,
            Self::WarningMin => 0x2,
            Self::WarningMinSoon => 0x3,
            Self::Welcome => 0x4,
        }
    }

}

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
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::WarningHours),
            2 => Ok(Self::WarningMin),
            3 => Ok(Self::WarningMinSoon),
            4 => Ok(Self::Welcome),
            v => Err(crate::errors::EnumError::new("RaidInstanceMessage", v),)
        }
    }
}

