use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spellenergizelog.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spellenergizelog.wowm#L3):
/// ```text
/// enum PowerType : u32 {
///     MANA = 0;
///     RAGE = 1;
///     FOCUS = 2;
///     ENERGY = 3;
///     HAPPINESS = 4;
///     HEALTH = 0xFFFFFFFE;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum PowerType {
    /// # Comment
    ///
    /// UNIT_FIELD_POWER1
    MANA,
    /// # Comment
    ///
    /// UNIT_FIELD_POWER2
    RAGE,
    /// # Comment
    ///
    /// UNIT_FIELD_POWER3
    FOCUS,
    /// # Comment
    ///
    /// UNIT_FIELD_POWER4
    ENERGY,
    /// # Comment
    ///
    /// UNIT_FIELD_POWER5
    HAPPINESS,
    /// # Comment
    ///
    /// (-2 as signed value)
    HEALTH,
}

impl PowerType {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::MANA => 0x0,
            Self::RAGE => 0x1,
            Self::FOCUS => 0x2,
            Self::ENERGY => 0x3,
            Self::HAPPINESS => 0x4,
            Self::HEALTH => 0xfffffffe,
        }
    }

}

impl Default for PowerType {
    fn default() -> Self {
        Self::MANA
    }
}

impl std::fmt::Display for PowerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MANA => f.write_str("MANA"),
            Self::RAGE => f.write_str("RAGE"),
            Self::FOCUS => f.write_str("FOCUS"),
            Self::ENERGY => f.write_str("ENERGY"),
            Self::HAPPINESS => f.write_str("HAPPINESS"),
            Self::HEALTH => f.write_str("HEALTH"),
        }
    }
}

impl TryFrom<u32> for PowerType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::MANA),
            1 => Ok(Self::RAGE),
            2 => Ok(Self::FOCUS),
            3 => Ok(Self::ENERGY),
            4 => Ok(Self::HAPPINESS),
            4294967294 => Ok(Self::HEALTH),
            v => Err(crate::errors::EnumError::new("PowerType", v as u32),)
        }
    }
}

