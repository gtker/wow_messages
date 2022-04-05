use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/remaining.wowm:2830`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/remaining.wowm#L2830):
/// ```text
/// enum ItemClass : u8 {
///     CONSUMABLE = 0;
///     CONTAINER = 1;
///     WEAPON = 2;
///     RESERVED_1 = 3;
///     ARMOR = 4;
///     REAGENT = 5;
///     PROJECTILE = 6;
///     TRADE_GOODS = 7;
///     RESERVED_2 = 8;
///     RECIPE = 9;
///     RESERVED_3 = 10;
///     QUIVER = 11;
///     QUEST = 12;
///     KEY = 13;
///     RESERVED_4 = 14;
///     MISC = 15;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum ItemClass {
    CONSUMABLE,
    CONTAINER,
    WEAPON,
    RESERVED_1,
    ARMOR,
    REAGENT,
    PROJECTILE,
    TRADE_GOODS,
    RESERVED_2,
    RECIPE,
    RESERVED_3,
    QUIVER,
    QUEST,
    KEY,
    RESERVED_4,
    MISC,
}

impl ReadableAndWritable for ItemClass {
    type Error = ItemClassError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u8_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u8().to_le_bytes())?;
        Ok(())
    }

}

impl ItemClass {
    pub fn read_u16_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, ItemClassError> {
        let a = crate::util::read_u16_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_le(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u16_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, ItemClassError> {
        let a = crate::util::read_u16_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_be(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u32_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, ItemClassError> {
        let a = crate::util::read_u32_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_le(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, ItemClassError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, ItemClassError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, ItemClassError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub const fn as_u8(&self) -> u8 {
        match self {
            Self::CONSUMABLE => 0x0,
            Self::CONTAINER => 0x1,
            Self::WEAPON => 0x2,
            Self::RESERVED_1 => 0x3,
            Self::ARMOR => 0x4,
            Self::REAGENT => 0x5,
            Self::PROJECTILE => 0x6,
            Self::TRADE_GOODS => 0x7,
            Self::RESERVED_2 => 0x8,
            Self::RECIPE => 0x9,
            Self::RESERVED_3 => 0xa,
            Self::QUIVER => 0xb,
            Self::QUEST => 0xc,
            Self::KEY => 0xd,
            Self::RESERVED_4 => 0xe,
            Self::MISC => 0xf,
        }
    }

    pub const fn new() -> Self {
        Self::CONSUMABLE
    }

}

impl ConstantSized for ItemClass {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for ItemClass {
    fn maximum_possible_size() -> usize {
        1
    }
}

impl Default for ItemClass {
    fn default() -> Self {
        Self::CONSUMABLE
    }
}

impl std::fmt::Display for ItemClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CONSUMABLE => f.write_str("CONSUMABLE"),
            Self::CONTAINER => f.write_str("CONTAINER"),
            Self::WEAPON => f.write_str("WEAPON"),
            Self::RESERVED_1 => f.write_str("RESERVED_1"),
            Self::ARMOR => f.write_str("ARMOR"),
            Self::REAGENT => f.write_str("REAGENT"),
            Self::PROJECTILE => f.write_str("PROJECTILE"),
            Self::TRADE_GOODS => f.write_str("TRADE_GOODS"),
            Self::RESERVED_2 => f.write_str("RESERVED_2"),
            Self::RECIPE => f.write_str("RECIPE"),
            Self::RESERVED_3 => f.write_str("RESERVED_3"),
            Self::QUIVER => f.write_str("QUIVER"),
            Self::QUEST => f.write_str("QUEST"),
            Self::KEY => f.write_str("KEY"),
            Self::RESERVED_4 => f.write_str("RESERVED_4"),
            Self::MISC => f.write_str("MISC"),
        }
    }
}

impl TryFrom<u8> for ItemClass {
    type Error = TryFromItemClassError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::CONSUMABLE),
            1 => Ok(Self::CONTAINER),
            2 => Ok(Self::WEAPON),
            3 => Ok(Self::RESERVED_1),
            4 => Ok(Self::ARMOR),
            5 => Ok(Self::REAGENT),
            6 => Ok(Self::PROJECTILE),
            7 => Ok(Self::TRADE_GOODS),
            8 => Ok(Self::RESERVED_2),
            9 => Ok(Self::RECIPE),
            10 => Ok(Self::RESERVED_3),
            11 => Ok(Self::QUIVER),
            12 => Ok(Self::QUEST),
            13 => Ok(Self::KEY),
            14 => Ok(Self::RESERVED_4),
            15 => Ok(Self::MISC),
            _ => Err(TryFromItemClassError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromItemClassError {
    value: u8,
}

impl TryFromItemClassError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum ItemClassError {
    Read(std::io::Error),
    TryFrom(TryFromItemClassError),
}

impl std::error::Error for ItemClassError {}
impl std::fmt::Display for TryFromItemClassError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'ItemClass': '{}'", self.value))
    }
}

impl std::fmt::Display for ItemClassError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for ItemClassError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromItemClassError> for ItemClassError {
    fn from(value: TryFromItemClassError) -> Self {
        Self::TryFrom(value)
    }
}

