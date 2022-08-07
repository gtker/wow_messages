use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_realm/server.wowm:43`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_realm/server.wowm#L43):
/// ```text
/// enum RealmCategory : u8 {
///     DEFAULT = 0x0;
///     ONE = 0x1;
///     TWO = 0x2;
///     THREE = 0x3;
///     FIVE = 0x5;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum RealmCategory {
    Default,
    One,
    Two,
    Three,
    Five,
}

impl RealmCategory {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Default => 0x0,
            Self::One => 0x1,
            Self::Two => 0x2,
            Self::Three => 0x3,
            Self::Five => 0x5,
        }
    }

}

impl Default for RealmCategory {
    fn default() -> Self {
        Self::Default
    }
}

impl std::fmt::Display for RealmCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Default => f.write_str("Default"),
            Self::One => f.write_str("One"),
            Self::Two => f.write_str("Two"),
            Self::Three => f.write_str("Three"),
            Self::Five => f.write_str("Five"),
        }
    }
}

impl TryFrom<u8> for RealmCategory {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Default),
            1 => Ok(Self::One),
            2 => Ok(Self::Two),
            3 => Ok(Self::Three),
            5 => Ok(Self::Five),
            v => Err(crate::errors::EnumError::new("RealmCategory", v as u32),)
        }
    }
}

