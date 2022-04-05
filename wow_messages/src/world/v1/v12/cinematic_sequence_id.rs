use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/add_messages.wowm:461`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/add_messages.wowm#L461):
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

impl ReadableAndWritable for CinematicSequenceId {
    type Error = CinematicSequenceIdError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u32_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u32().to_le_bytes())?;
        Ok(())
    }

}

impl CinematicSequenceId {
    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, CinematicSequenceIdError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u32() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, CinematicSequenceIdError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, CinematicSequenceIdError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub const fn as_u32(&self) -> u32 {
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

    pub const fn new() -> Self {
        Self::HUMAN
    }

}

impl ConstantSized for CinematicSequenceId {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CinematicSequenceId {
    fn maximum_possible_size() -> usize {
        4
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
    type Error = TryFromCinematicSequenceIdError;
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
            _ => Err(TryFromCinematicSequenceIdError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromCinematicSequenceIdError {
    value: u32,
}

impl TryFromCinematicSequenceIdError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum CinematicSequenceIdError {
    Read(std::io::Error),
    TryFrom(TryFromCinematicSequenceIdError),
}

impl std::error::Error for CinematicSequenceIdError {}
impl std::fmt::Display for TryFromCinematicSequenceIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'CinematicSequenceId': '{}'", self.value))
    }
}

impl std::fmt::Display for CinematicSequenceIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for CinematicSequenceIdError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromCinematicSequenceIdError> for CinematicSequenceIdError {
    fn from(value: TryFromCinematicSequenceIdError) -> Self {
        Self::TryFrom(value)
    }
}

