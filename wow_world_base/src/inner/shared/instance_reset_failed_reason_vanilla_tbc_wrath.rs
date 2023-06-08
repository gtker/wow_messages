/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/smsg_instance_reset_failed.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/smsg_instance_reset_failed.wowm#L1):
/// ```text
/// enum InstanceResetFailedReason : u8 {
///     GENERAL = 0;
///     OFFLINE = 1;
///     ZONING = 2;
///     SILENTLY = 3;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum InstanceResetFailedReason {
    /// at least one player is in the instance
    General,
    /// at least one player is offline
    Offline,
    /// at least one player try to enter the instance (being teleported in)
    Zoning,
    Silently,
}

impl InstanceResetFailedReason {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::General => 0x0,
            Self::Offline => 0x1,
            Self::Zoning => 0x2,
            Self::Silently => 0x3,
        }
    }

}

#[cfg(feature = "print-testcase")]
impl InstanceResetFailedReason {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::General => "GENERAL",
            Self::Offline => "OFFLINE",
            Self::Zoning => "ZONING",
            Self::Silently => "SILENTLY",
        }
    }

}

impl Default for InstanceResetFailedReason {
    fn default() -> Self {
        Self::General
    }
}

impl std::fmt::Display for InstanceResetFailedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::General => f.write_str("General"),
            Self::Offline => f.write_str("Offline"),
            Self::Zoning => f.write_str("Zoning"),
            Self::Silently => f.write_str("Silently"),
        }
    }
}

impl TryFrom<u8> for InstanceResetFailedReason {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::General),
            1 => Ok(Self::Offline),
            2 => Ok(Self::Zoning),
            3 => Ok(Self::Silently),
            v => Err(crate::errors::EnumError::new("InstanceResetFailedReason", v as u64),)
        }
    }
}

