use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

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
    type Error = ClassError;
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
            _ => Err(ClassError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct ClassError {
    pub value: u8,
}

impl ClassError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

impl std::error::Error for ClassError {}
impl std::fmt::Display for ClassError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'Class': '{}'", self.value))
    }
}

