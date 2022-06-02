use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/cinematic/smsg_trigger_cinematic.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/cinematic/smsg_trigger_cinematic.wowm#L3):
/// ```text
/// enum CinematicSequenceId : u32 {
///     HUMAN = 81;
///     ORC = 21;
///     DWARF = 41;
///     NIGHT_ELF = 61;
///     UNDEAD = 2;
///     TAUREN = 141;
///     GNOME = 101;
///     TROLL = 121;
///     GOBLIN = 0;
/// }

/// ```
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
    type Error = crate::errors::EnumError;
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
            v => Err(crate::errors::EnumError::new("CinematicSequenceId", v as u32),)
        }
    }
}

