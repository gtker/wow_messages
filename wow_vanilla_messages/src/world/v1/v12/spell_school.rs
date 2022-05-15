use std::convert::{TryFrom, TryInto};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum SpellSchool {
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
    type Error = SpellSchoolError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NORMAL),
            1 => Ok(Self::HOLY),
            2 => Ok(Self::FIRE),
            3 => Ok(Self::NATURE),
            4 => Ok(Self::FROST),
            5 => Ok(Self::SHADOW),
            6 => Ok(Self::ARCANE),
            _ => Err(SpellSchoolError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct SpellSchoolError {
    value: u8,
}

impl SpellSchoolError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

impl std::error::Error for SpellSchoolError {}
impl std::fmt::Display for SpellSchoolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'SpellSchool': '{}'", self.value))
    }
}

