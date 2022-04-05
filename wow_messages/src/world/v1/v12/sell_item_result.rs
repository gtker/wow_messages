use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/remaining.wowm:4861`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/remaining.wowm#L4861):
/// ```text
/// enum SellItemResult : u8 {
///     CANT_FIND_ITEM = 1;
///     CANT_SELL_ITEM = 2;
///     CANT_FIND_VENDOR = 3;
///     YOU_DONT_OWN_THAT_ITEM = 4;
///     UNK = 5;
///     ONLY_EMPTY_BAG = 6;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum SellItemResult {
    CANT_FIND_ITEM,
    CANT_SELL_ITEM,
    CANT_FIND_VENDOR,
    YOU_DONT_OWN_THAT_ITEM,
    UNK,
    ONLY_EMPTY_BAG,
}

impl ReadableAndWritable for SellItemResult {
    type Error = SellItemResultError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u8_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u8().to_le_bytes())?;
        Ok(())
    }

}

impl SellItemResult {
    pub fn read_u16_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, SellItemResultError> {
        let a = crate::util::read_u16_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_le(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u16_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, SellItemResultError> {
        let a = crate::util::read_u16_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_be(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u32_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, SellItemResultError> {
        let a = crate::util::read_u32_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_le(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, SellItemResultError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, SellItemResultError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, SellItemResultError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub const fn as_u8(&self) -> u8 {
        match self {
            Self::CANT_FIND_ITEM => 0x1,
            Self::CANT_SELL_ITEM => 0x2,
            Self::CANT_FIND_VENDOR => 0x3,
            Self::YOU_DONT_OWN_THAT_ITEM => 0x4,
            Self::UNK => 0x5,
            Self::ONLY_EMPTY_BAG => 0x6,
        }
    }

    pub const fn new() -> Self {
        Self::CANT_FIND_ITEM
    }

}

impl ConstantSized for SellItemResult {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SellItemResult {
    fn maximum_possible_size() -> usize {
        1
    }
}

impl Default for SellItemResult {
    fn default() -> Self {
        Self::CANT_FIND_ITEM
    }
}

impl std::fmt::Display for SellItemResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CANT_FIND_ITEM => f.write_str("CANT_FIND_ITEM"),
            Self::CANT_SELL_ITEM => f.write_str("CANT_SELL_ITEM"),
            Self::CANT_FIND_VENDOR => f.write_str("CANT_FIND_VENDOR"),
            Self::YOU_DONT_OWN_THAT_ITEM => f.write_str("YOU_DONT_OWN_THAT_ITEM"),
            Self::UNK => f.write_str("UNK"),
            Self::ONLY_EMPTY_BAG => f.write_str("ONLY_EMPTY_BAG"),
        }
    }
}

impl TryFrom<u8> for SellItemResult {
    type Error = TryFromSellItemResultError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::CANT_FIND_ITEM),
            2 => Ok(Self::CANT_SELL_ITEM),
            3 => Ok(Self::CANT_FIND_VENDOR),
            4 => Ok(Self::YOU_DONT_OWN_THAT_ITEM),
            5 => Ok(Self::UNK),
            6 => Ok(Self::ONLY_EMPTY_BAG),
            _ => Err(TryFromSellItemResultError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromSellItemResultError {
    value: u8,
}

impl TryFromSellItemResultError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum SellItemResultError {
    Read(std::io::Error),
    TryFrom(TryFromSellItemResultError),
}

impl std::error::Error for SellItemResultError {}
impl std::fmt::Display for TryFromSellItemResultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'SellItemResult': '{}'", self.value))
    }
}

impl std::fmt::Display for SellItemResultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for SellItemResultError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromSellItemResultError> for SellItemResultError {
    fn from(value: TryFromSellItemResultError) -> Self {
        Self::TryFrom(value)
    }
}

