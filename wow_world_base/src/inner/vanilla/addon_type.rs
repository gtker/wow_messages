/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/smsg_addon_info.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/smsg_addon_info.wowm#L1):
/// ```text
/// enum AddonType : u8 {
///     BANNED = 0;
///     ENABLED = 1;
///     BLIZZARD = 2;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum AddonType {
    Banned,
    /// Shows addon in list. Probably intended for player-created addons.
    Enabled,
    /// Hides addon from list.
    Blizzard,
}

impl AddonType {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Banned => 0x0,
            Self::Enabled => 0x1,
            Self::Blizzard => 0x2,
        }
    }

}

#[cfg(feature = "print-testcase")]
impl AddonType {
    pub fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Banned => "BANNED",
            Self::Enabled => "ENABLED",
            Self::Blizzard => "BLIZZARD",
        }
    }

}

impl Default for AddonType {
    fn default() -> Self {
        Self::Banned
    }
}

impl std::fmt::Display for AddonType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Banned => f.write_str("Banned"),
            Self::Enabled => f.write_str("Enabled"),
            Self::Blizzard => f.write_str("Blizzard"),
        }
    }
}

impl TryFrom<u8> for AddonType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Banned),
            1 => Ok(Self::Enabled),
            2 => Ok(Self::Blizzard),
            v => Err(crate::errors::EnumError::new("AddonType", v as u64),)
        }
    }
}

