use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/remaining.wowm:3149`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/remaining.wowm#L3149):
/// ```text
/// enum LootMethod : u8 {
///     CORPSE = 1;
///     PICKPOCKETING = 2;
///     FISHING = 3;
///     DISENCHANTING = 4;
///     SKINNING = 6;
///     FISHINGHOLE = 20;
///     FISHING_FAIL = 21;
///     INSIGNIA = 22;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum LootMethod {
    CORPSE,
    PICKPOCKETING,
    FISHING,
    DISENCHANTING,
    SKINNING,
    FISHINGHOLE,
    FISHING_FAIL,
    INSIGNIA,
}

impl ReadableAndWritable for LootMethod {
    type Error = LootMethodError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u8_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u8().to_le_bytes())?;
        Ok(())
    }

}

impl LootMethod {
    pub fn read_u16_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, LootMethodError> {
        let a = crate::util::read_u16_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_le(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u16_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, LootMethodError> {
        let a = crate::util::read_u16_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_be(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u32_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, LootMethodError> {
        let a = crate::util::read_u32_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_le(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, LootMethodError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, LootMethodError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, LootMethodError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub const fn as_u8(&self) -> u8 {
        match self {
            Self::CORPSE => 0x1,
            Self::PICKPOCKETING => 0x2,
            Self::FISHING => 0x3,
            Self::DISENCHANTING => 0x4,
            Self::SKINNING => 0x6,
            Self::FISHINGHOLE => 0x14,
            Self::FISHING_FAIL => 0x15,
            Self::INSIGNIA => 0x16,
        }
    }

    pub const fn new() -> Self {
        Self::CORPSE
    }

}

impl ConstantSized for LootMethod {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for LootMethod {
    fn maximum_possible_size() -> usize {
        1
    }
}

impl Default for LootMethod {
    fn default() -> Self {
        Self::CORPSE
    }
}

impl std::fmt::Display for LootMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CORPSE => f.write_str("CORPSE"),
            Self::PICKPOCKETING => f.write_str("PICKPOCKETING"),
            Self::FISHING => f.write_str("FISHING"),
            Self::DISENCHANTING => f.write_str("DISENCHANTING"),
            Self::SKINNING => f.write_str("SKINNING"),
            Self::FISHINGHOLE => f.write_str("FISHINGHOLE"),
            Self::FISHING_FAIL => f.write_str("FISHING_FAIL"),
            Self::INSIGNIA => f.write_str("INSIGNIA"),
        }
    }
}

impl TryFrom<u8> for LootMethod {
    type Error = TryFromLootMethodError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::CORPSE),
            2 => Ok(Self::PICKPOCKETING),
            3 => Ok(Self::FISHING),
            4 => Ok(Self::DISENCHANTING),
            6 => Ok(Self::SKINNING),
            20 => Ok(Self::FISHINGHOLE),
            21 => Ok(Self::FISHING_FAIL),
            22 => Ok(Self::INSIGNIA),
            _ => Err(TryFromLootMethodError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromLootMethodError {
    value: u8,
}

impl TryFromLootMethodError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum LootMethodError {
    Read(std::io::Error),
    TryFrom(TryFromLootMethodError),
}

impl std::error::Error for LootMethodError {}
impl std::fmt::Display for TryFromLootMethodError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'LootMethod': '{}'", self.value))
    }
}

impl std::fmt::Display for LootMethodError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for LootMethodError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromLootMethodError> for LootMethodError {
    fn from(value: TryFromLootMethodError) -> Self {
        Self::TryFrom(value)
    }
}

