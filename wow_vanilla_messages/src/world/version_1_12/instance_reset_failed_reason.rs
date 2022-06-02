use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/smsg_instance_reset_failed.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/smsg_instance_reset_failed.wowm#L3):
/// ```text
/// enum InstanceResetFailedReason : u8 {
///     GENERAL = 0;
///     OFFLINE = 1;
///     ZONING = 2;
///     SILENTLY = 3;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum InstanceResetFailedReason {
    /// # Comment
    ///
    /// at least one player is in the instance
    ///
    GENERAL,
    /// # Comment
    ///
    /// at least one player is offline
    ///
    OFFLINE,
    /// # Comment
    ///
    /// at least one player try to enter the instance (being teleported in)
    ///
    ZONING,
    SILENTLY,
}

impl InstanceResetFailedReason {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::GENERAL => 0x0,
            Self::OFFLINE => 0x1,
            Self::ZONING => 0x2,
            Self::SILENTLY => 0x3,
        }
    }

}

impl Default for InstanceResetFailedReason {
    fn default() -> Self {
        Self::GENERAL
    }
}

impl std::fmt::Display for InstanceResetFailedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GENERAL => f.write_str("GENERAL"),
            Self::OFFLINE => f.write_str("OFFLINE"),
            Self::ZONING => f.write_str("ZONING"),
            Self::SILENTLY => f.write_str("SILENTLY"),
        }
    }
}

impl TryFrom<u8> for InstanceResetFailedReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::GENERAL),
            1 => Ok(Self::OFFLINE),
            2 => Ok(Self::ZONING),
            3 => Ok(Self::SILENTLY),
            v => Err(crate::errors::EnumError::new("InstanceResetFailedReason", v as u32),)
        }
    }
}

