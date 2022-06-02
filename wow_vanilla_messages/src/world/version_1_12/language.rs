use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/social_common.wowm:59`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/social_common.wowm#L59):
/// ```text
/// enum Language : u32 {
///     UNIVERSAL = 0;
///     ORCISH = 1;
///     DARNASSIAN = 2;
///     TAURAHE = 3;
///     DWARVISH = 6;
///     COMMON = 7;
///     DEMONIC = 8;
///     TITAN = 9;
///     THALASSIAN = 10;
///     DRACONIC = 11;
///     KALIMAG = 12;
///     GNOMISH = 13;
///     TROLL = 14;
///     GUTTERSPEAK = 33;
///     ADDON = 0xFFFFFFFF;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum Language {
    UNIVERSAL,
    ORCISH,
    DARNASSIAN,
    TAURAHE,
    DWARVISH,
    COMMON,
    DEMONIC,
    TITAN,
    THALASSIAN,
    DRACONIC,
    KALIMAG,
    GNOMISH,
    TROLL,
    GUTTERSPEAK,
    ADDON,
}

impl Language {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::UNIVERSAL => 0x0,
            Self::ORCISH => 0x1,
            Self::DARNASSIAN => 0x2,
            Self::TAURAHE => 0x3,
            Self::DWARVISH => 0x6,
            Self::COMMON => 0x7,
            Self::DEMONIC => 0x8,
            Self::TITAN => 0x9,
            Self::THALASSIAN => 0xa,
            Self::DRACONIC => 0xb,
            Self::KALIMAG => 0xc,
            Self::GNOMISH => 0xd,
            Self::TROLL => 0xe,
            Self::GUTTERSPEAK => 0x21,
            Self::ADDON => 0xffffffff,
        }
    }

}

impl Default for Language {
    fn default() -> Self {
        Self::UNIVERSAL
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UNIVERSAL => f.write_str("UNIVERSAL"),
            Self::ORCISH => f.write_str("ORCISH"),
            Self::DARNASSIAN => f.write_str("DARNASSIAN"),
            Self::TAURAHE => f.write_str("TAURAHE"),
            Self::DWARVISH => f.write_str("DWARVISH"),
            Self::COMMON => f.write_str("COMMON"),
            Self::DEMONIC => f.write_str("DEMONIC"),
            Self::TITAN => f.write_str("TITAN"),
            Self::THALASSIAN => f.write_str("THALASSIAN"),
            Self::DRACONIC => f.write_str("DRACONIC"),
            Self::KALIMAG => f.write_str("KALIMAG"),
            Self::GNOMISH => f.write_str("GNOMISH"),
            Self::TROLL => f.write_str("TROLL"),
            Self::GUTTERSPEAK => f.write_str("GUTTERSPEAK"),
            Self::ADDON => f.write_str("ADDON"),
        }
    }
}

impl TryFrom<u32> for Language {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::UNIVERSAL),
            1 => Ok(Self::ORCISH),
            2 => Ok(Self::DARNASSIAN),
            3 => Ok(Self::TAURAHE),
            6 => Ok(Self::DWARVISH),
            7 => Ok(Self::COMMON),
            8 => Ok(Self::DEMONIC),
            9 => Ok(Self::TITAN),
            10 => Ok(Self::THALASSIAN),
            11 => Ok(Self::DRACONIC),
            12 => Ok(Self::KALIMAG),
            13 => Ok(Self::GNOMISH),
            14 => Ok(Self::TROLL),
            33 => Ok(Self::GUTTERSPEAK),
            4294967295 => Ok(Self::ADDON),
            v => Err(crate::errors::EnumError::new("Language", v as u32),)
        }
    }
}

