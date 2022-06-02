use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/enums/class.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/enums/class.wowm#L3):
/// ```text
/// enum Class : u8 {
///     WARRIOR = 1;
///     PALADIN = 2;
///     HUNTER = 3;
///     ROGUE = 4;
///     PRIEST = 5;
///     SHAMAN = 7;
///     MAGE = 8;
///     WARLOCK = 9;
///     DRUID = 11;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum Class {
    WARRIOR,
    PALADIN,
    HUNTER,
    ROGUE,
    PRIEST,
    SHAMAN,
    MAGE,
    WARLOCK,
    DRUID,
}

impl Class {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::WARRIOR => 0x1,
            Self::PALADIN => 0x2,
            Self::HUNTER => 0x3,
            Self::ROGUE => 0x4,
            Self::PRIEST => 0x5,
            Self::SHAMAN => 0x7,
            Self::MAGE => 0x8,
            Self::WARLOCK => 0x9,
            Self::DRUID => 0xb,
        }
    }

}

impl Default for Class {
    fn default() -> Self {
        Self::WARRIOR
    }
}

impl std::fmt::Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WARRIOR => f.write_str("WARRIOR"),
            Self::PALADIN => f.write_str("PALADIN"),
            Self::HUNTER => f.write_str("HUNTER"),
            Self::ROGUE => f.write_str("ROGUE"),
            Self::PRIEST => f.write_str("PRIEST"),
            Self::SHAMAN => f.write_str("SHAMAN"),
            Self::MAGE => f.write_str("MAGE"),
            Self::WARLOCK => f.write_str("WARLOCK"),
            Self::DRUID => f.write_str("DRUID"),
        }
    }
}

impl TryFrom<u8> for Class {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::WARRIOR),
            2 => Ok(Self::PALADIN),
            3 => Ok(Self::HUNTER),
            4 => Ok(Self::ROGUE),
            5 => Ok(Self::PRIEST),
            7 => Ok(Self::SHAMAN),
            8 => Ok(Self::MAGE),
            9 => Ok(Self::WARLOCK),
            11 => Ok(Self::DRUID),
            v => Err(crate::errors::EnumError::new("Class", v as u32),)
        }
    }
}

