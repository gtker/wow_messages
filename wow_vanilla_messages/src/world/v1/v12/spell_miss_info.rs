use std::convert::{TryFrom, TryInto};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum SpellMissInfo {
    NONE,
    MISS,
    RESIST,
    DODGE,
    PARRY,
    BLOCK,
    EVADE,
    IMMUNE,
    IMMUNE2,
    DEFLECT,
    ABSORB,
    REFLECT,
}

impl SpellMissInfo {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::NONE => 0x0,
            Self::MISS => 0x1,
            Self::RESIST => 0x2,
            Self::DODGE => 0x3,
            Self::PARRY => 0x4,
            Self::BLOCK => 0x5,
            Self::EVADE => 0x6,
            Self::IMMUNE => 0x7,
            Self::IMMUNE2 => 0x8,
            Self::DEFLECT => 0x9,
            Self::ABSORB => 0xa,
            Self::REFLECT => 0xb,
        }
    }

}

impl Default for SpellMissInfo {
    fn default() -> Self {
        Self::NONE
    }
}

impl std::fmt::Display for SpellMissInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NONE => f.write_str("NONE"),
            Self::MISS => f.write_str("MISS"),
            Self::RESIST => f.write_str("RESIST"),
            Self::DODGE => f.write_str("DODGE"),
            Self::PARRY => f.write_str("PARRY"),
            Self::BLOCK => f.write_str("BLOCK"),
            Self::EVADE => f.write_str("EVADE"),
            Self::IMMUNE => f.write_str("IMMUNE"),
            Self::IMMUNE2 => f.write_str("IMMUNE2"),
            Self::DEFLECT => f.write_str("DEFLECT"),
            Self::ABSORB => f.write_str("ABSORB"),
            Self::REFLECT => f.write_str("REFLECT"),
        }
    }
}

impl TryFrom<u32> for SpellMissInfo {
    type Error = SpellMissInfoError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NONE),
            1 => Ok(Self::MISS),
            2 => Ok(Self::RESIST),
            3 => Ok(Self::DODGE),
            4 => Ok(Self::PARRY),
            5 => Ok(Self::BLOCK),
            6 => Ok(Self::EVADE),
            7 => Ok(Self::IMMUNE),
            8 => Ok(Self::IMMUNE2),
            9 => Ok(Self::DEFLECT),
            10 => Ok(Self::ABSORB),
            11 => Ok(Self::REFLECT),
            _ => Err(SpellMissInfoError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct SpellMissInfoError {
    pub value: u32,
}

impl SpellMissInfoError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

impl std::error::Error for SpellMissInfoError {}
impl std::fmt::Display for SpellMissInfoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'SpellMissInfo': '{}'", self.value))
    }
}

