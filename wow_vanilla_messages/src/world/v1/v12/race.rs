use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum Race {
    HUMAN,
    ORC,
    DWARF,
    NIGHTELF,
    UNDEAD,
    TAUREN,
    GNOME,
    TROLL,
    GOBLIN,
}

impl Race {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::HUMAN => 0x1,
            Self::ORC => 0x2,
            Self::DWARF => 0x3,
            Self::NIGHTELF => 0x4,
            Self::UNDEAD => 0x5,
            Self::TAUREN => 0x6,
            Self::GNOME => 0x7,
            Self::TROLL => 0x8,
            Self::GOBLIN => 0x9,
        }
    }

}

impl Default for Race {
    fn default() -> Self {
        Self::HUMAN
    }
}

impl std::fmt::Display for Race {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HUMAN => f.write_str("HUMAN"),
            Self::ORC => f.write_str("ORC"),
            Self::DWARF => f.write_str("DWARF"),
            Self::NIGHTELF => f.write_str("NIGHTELF"),
            Self::UNDEAD => f.write_str("UNDEAD"),
            Self::TAUREN => f.write_str("TAUREN"),
            Self::GNOME => f.write_str("GNOME"),
            Self::TROLL => f.write_str("TROLL"),
            Self::GOBLIN => f.write_str("GOBLIN"),
        }
    }
}

impl TryFrom<u8> for Race {
    type Error = RaceError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::HUMAN),
            2 => Ok(Self::ORC),
            3 => Ok(Self::DWARF),
            4 => Ok(Self::NIGHTELF),
            5 => Ok(Self::UNDEAD),
            6 => Ok(Self::TAUREN),
            7 => Ok(Self::GNOME),
            8 => Ok(Self::TROLL),
            9 => Ok(Self::GOBLIN),
            _ => Err(RaceError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct RaceError {
    value: u8,
}

impl RaceError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

impl std::error::Error for RaceError {}
impl std::fmt::Display for RaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'Race': '{}'", self.value))
    }
}

