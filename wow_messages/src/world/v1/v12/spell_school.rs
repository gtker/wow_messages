use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/new5.wowm:231`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/new5.wowm#L231):
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
    NORMAL,
    HOLY,
    FIRE,
    NATURE,
    FROST,
    SHADOW,
    ARCANE,
}

impl ReadableAndWritable for SpellSchool {
    type Error = SpellSchoolError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u8_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u8().to_le_bytes())?;
        Ok(())
    }

}

impl SpellSchool {
    pub fn read_u16_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, SpellSchoolError> {
        let a = crate::util::read_u16_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_le(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u16_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, SpellSchoolError> {
        let a = crate::util::read_u16_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_be(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u32_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, SpellSchoolError> {
        let a = crate::util::read_u32_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_le(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, SpellSchoolError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, SpellSchoolError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, SpellSchoolError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub const fn as_u8(&self) -> u8 {
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

    pub const fn new() -> Self {
        Self::NORMAL
    }

}

impl ConstantSized for SpellSchool {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SpellSchool {
    fn maximum_possible_size() -> usize {
        1
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
    type Error = TryFromSpellSchoolError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NORMAL),
            1 => Ok(Self::HOLY),
            2 => Ok(Self::FIRE),
            3 => Ok(Self::NATURE),
            4 => Ok(Self::FROST),
            5 => Ok(Self::SHADOW),
            6 => Ok(Self::ARCANE),
            _ => Err(TryFromSpellSchoolError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromSpellSchoolError {
    value: u8,
}

impl TryFromSpellSchoolError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum SpellSchoolError {
    Read(std::io::Error),
    TryFrom(TryFromSpellSchoolError),
}

impl std::error::Error for SpellSchoolError {}
impl std::fmt::Display for TryFromSpellSchoolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'SpellSchool': '{}'", self.value))
    }
}

impl std::fmt::Display for SpellSchoolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for SpellSchoolError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromSpellSchoolError> for SpellSchoolError {
    fn from(value: TryFromSpellSchoolError) -> Self {
        Self::TryFrom(value)
    }
}

