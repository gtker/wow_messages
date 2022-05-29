use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum PowerType {
    MANA,
    RAGE,
    FOCUS,
    ENERGY,
    HAPPINESS,
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

