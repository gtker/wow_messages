use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/remaining.wowm:980`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/remaining.wowm#L980):
/// ```text
/// enum SpellMissInfo : u32 {
///     NONE = 0;
///     MISS = 1;
///     RESIST = 2;
///     DODGE = 3;
///     PARRY = 4;
///     BLOCK = 5;
///     EVADE = 6;
///     IMMUNE = 7;
///     IMMUNE2 = 8;
///     DEFLECT = 9;
///     ABSORB = 10;
///     REFLECT = 11;
/// }
/// ```
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

impl ReadableAndWritable for SpellMissInfo {
    type Error = SpellMissInfoError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u32_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u32().to_le_bytes())?;
        Ok(())
    }

}

impl SpellMissInfo {
    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, SpellMissInfoError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u32() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, SpellMissInfoError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, SpellMissInfoError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub const fn as_u32(&self) -> u32 {
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

    pub const fn new() -> Self {
        Self::NONE
    }

}

impl ConstantSized for SpellMissInfo {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SpellMissInfo {
    fn maximum_possible_size() -> usize {
        4
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
    type Error = TryFromSpellMissInfoError;
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
            _ => Err(TryFromSpellMissInfoError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromSpellMissInfoError {
    value: u32,
}

impl TryFromSpellMissInfoError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum SpellMissInfoError {
    Read(std::io::Error),
    TryFrom(TryFromSpellMissInfoError),
}

impl std::error::Error for SpellMissInfoError {}
impl std::fmt::Display for TryFromSpellMissInfoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'SpellMissInfo': '{}'", self.value))
    }
}

impl std::fmt::Display for SpellMissInfoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for SpellMissInfoError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromSpellMissInfoError> for SpellMissInfoError {
    fn from(value: TryFromSpellMissInfoError) -> Self {
        Self::TryFrom(value)
    }
}

