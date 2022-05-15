use std::convert::{TryFrom, TryInto};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum CinematicSequenceId {
    HUMAN,
    ORC,
    DWARF,
    NIGHT_ELF,
    UNDEAD,
    TAUREN,
    GNOME,
    TROLL,
    GOBLIN,
}

impl CinematicSequenceId {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::HUMAN => 0x51,
            Self::ORC => 0x15,
            Self::DWARF => 0x29,
            Self::NIGHT_ELF => 0x3d,
            Self::UNDEAD => 0x2,
            Self::TAUREN => 0x8d,
            Self::GNOME => 0x65,
            Self::TROLL => 0x79,
            Self::GOBLIN => 0x0,
        }
    }

}

impl Default for CinematicSequenceId {
    fn default() -> Self {
        Self::HUMAN
    }
}

impl std::fmt::Display for CinematicSequenceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HUMAN => f.write_str("HUMAN"),
            Self::ORC => f.write_str("ORC"),
            Self::DWARF => f.write_str("DWARF"),
            Self::NIGHT_ELF => f.write_str("NIGHT_ELF"),
            Self::UNDEAD => f.write_str("UNDEAD"),
            Self::TAUREN => f.write_str("TAUREN"),
            Self::GNOME => f.write_str("GNOME"),
            Self::TROLL => f.write_str("TROLL"),
            Self::GOBLIN => f.write_str("GOBLIN"),
        }
    }
}

impl TryFrom<u32> for CinematicSequenceId {
    type Error = CinematicSequenceIdError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            81 => Ok(Self::HUMAN),
            21 => Ok(Self::ORC),
            41 => Ok(Self::DWARF),
            61 => Ok(Self::NIGHT_ELF),
            2 => Ok(Self::UNDEAD),
            141 => Ok(Self::TAUREN),
            101 => Ok(Self::GNOME),
            121 => Ok(Self::TROLL),
            0 => Ok(Self::GOBLIN),
            _ => Err(CinematicSequenceIdError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct CinematicSequenceIdError {
    value: u32,
}

impl CinematicSequenceIdError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

impl std::error::Error for CinematicSequenceIdError {}
impl std::fmt::Display for CinematicSequenceIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'CinematicSequenceId': '{}'", self.value))
    }
}

