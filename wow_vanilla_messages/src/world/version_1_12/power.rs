use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/common.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/common.wowm#L3):
/// ```text
/// enum Power : u8 {
///     MANA = 0;
///     RAGE = 1;
///     FOCUS = 2;
///     ENERGY = 3;
///     HAPPINESS = 4;
///     MAX_POWERS = 5;
///     ALL = 127;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum Power {
    MANA,
    RAGE,
    FOCUS,
    ENERGY,
    HAPPINESS,
    MAX_POWERS,
    ALL,
}

impl Power {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::MANA => 0x0,
            Self::RAGE => 0x1,
            Self::FOCUS => 0x2,
            Self::ENERGY => 0x3,
            Self::HAPPINESS => 0x4,
            Self::MAX_POWERS => 0x5,
            Self::ALL => 0x7f,
        }
    }

}

impl Default for Power {
    fn default() -> Self {
        Self::MANA
    }
}

impl std::fmt::Display for Power {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MANA => f.write_str("MANA"),
            Self::RAGE => f.write_str("RAGE"),
            Self::FOCUS => f.write_str("FOCUS"),
            Self::ENERGY => f.write_str("ENERGY"),
            Self::HAPPINESS => f.write_str("HAPPINESS"),
            Self::MAX_POWERS => f.write_str("MAX_POWERS"),
            Self::ALL => f.write_str("ALL"),
        }
    }
}

impl TryFrom<u8> for Power {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::MANA),
            1 => Ok(Self::RAGE),
            2 => Ok(Self::FOCUS),
            3 => Ok(Self::ENERGY),
            4 => Ok(Self::HAPPINESS),
            5 => Ok(Self::MAX_POWERS),
            127 => Ok(Self::ALL),
            v => Err(crate::errors::EnumError::new("Power", v as u32),)
        }
    }
}

