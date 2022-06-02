use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/spell_common.wowm:33`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/spell_common.wowm#L33):
/// ```text
/// enum SpellSchool : u8 {
///     NORMAL = 0;
///     HOLY = 1;
///     FIRE = 2;
///     NATURE = 3;
///     FROST = 4;
///     SHADOW = 5;
///     ARCANE = 6;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum SpellSchool {
    /// # Comment
    ///
    /// Physical, Armor
    NORMAL,
    HOLY,
    FIRE,
    NATURE,
    FROST,
    SHADOW,
    ARCANE,
}

impl SpellSchool {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::NORMAL => 0x0,
            Self::HOLY => 0x1,
            Self::FIRE => 0x2,
            Self::NATURE => 0x3,
            Self::FROST => 0x4,
            Self::SHADOW => 0x5,
            Self::ARCANE => 0x6,
        }
    }

}

impl Default for SpellSchool {
    fn default() -> Self {
        Self::NORMAL
    }
}

impl std::fmt::Display for SpellSchool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NORMAL => f.write_str("NORMAL"),
            Self::HOLY => f.write_str("HOLY"),
            Self::FIRE => f.write_str("FIRE"),
            Self::NATURE => f.write_str("NATURE"),
            Self::FROST => f.write_str("FROST"),
            Self::SHADOW => f.write_str("SHADOW"),
            Self::ARCANE => f.write_str("ARCANE"),
        }
    }
}

impl TryFrom<u8> for SpellSchool {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NORMAL),
            1 => Ok(Self::HOLY),
            2 => Ok(Self::FIRE),
            3 => Ok(Self::NATURE),
            4 => Ok(Self::FROST),
            5 => Ok(Self::SHADOW),
            6 => Ok(Self::ARCANE),
            v => Err(crate::errors::EnumError::new("SpellSchool", v as u32),)
        }
    }
}

