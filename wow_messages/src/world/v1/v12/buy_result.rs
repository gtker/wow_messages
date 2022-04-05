use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/remaining.wowm:3327`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/remaining.wowm#L3327):
/// ```text
/// enum BuyResult : u8 {
///     CANT_FIND_ITEM = 0;
///     ITEM_ALREADY_SOLD = 1;
///     NOT_ENOUGHT_MONEY = 2;
///     SELLER_DONT_LIKE_YOU = 4;
///     DISTANCE_TOO_FAR = 5;
///     ITEM_SOLD_OUT = 7;
///     CANT_CARRY_MORE = 8;
///     RANK_REQUIRE = 11;
///     REPUTATION_REQUIRE = 12;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum BuyResult {
    CANT_FIND_ITEM,
    ITEM_ALREADY_SOLD,
    NOT_ENOUGHT_MONEY,
    SELLER_DONT_LIKE_YOU,
    DISTANCE_TOO_FAR,
    ITEM_SOLD_OUT,
    CANT_CARRY_MORE,
    RANK_REQUIRE,
    REPUTATION_REQUIRE,
}

impl ReadableAndWritable for BuyResult {
    type Error = BuyResultError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u8_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u8().to_le_bytes())?;
        Ok(())
    }

}

impl BuyResult {
    pub fn read_u16_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, BuyResultError> {
        let a = crate::util::read_u16_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_le(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u16_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, BuyResultError> {
        let a = crate::util::read_u16_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_be(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u32_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, BuyResultError> {
        let a = crate::util::read_u32_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_le(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, BuyResultError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, BuyResultError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, BuyResultError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub const fn as_u8(&self) -> u8 {
        match self {
            Self::CANT_FIND_ITEM => 0x0,
            Self::ITEM_ALREADY_SOLD => 0x1,
            Self::NOT_ENOUGHT_MONEY => 0x2,
            Self::SELLER_DONT_LIKE_YOU => 0x4,
            Self::DISTANCE_TOO_FAR => 0x5,
            Self::ITEM_SOLD_OUT => 0x7,
            Self::CANT_CARRY_MORE => 0x8,
            Self::RANK_REQUIRE => 0xb,
            Self::REPUTATION_REQUIRE => 0xc,
        }
    }

    pub const fn new() -> Self {
        Self::CANT_FIND_ITEM
    }

}

impl ConstantSized for BuyResult {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for BuyResult {
    fn maximum_possible_size() -> usize {
        1
    }
}

impl Default for BuyResult {
    fn default() -> Self {
        Self::CANT_FIND_ITEM
    }
}

impl std::fmt::Display for BuyResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CANT_FIND_ITEM => f.write_str("CANT_FIND_ITEM"),
            Self::ITEM_ALREADY_SOLD => f.write_str("ITEM_ALREADY_SOLD"),
            Self::NOT_ENOUGHT_MONEY => f.write_str("NOT_ENOUGHT_MONEY"),
            Self::SELLER_DONT_LIKE_YOU => f.write_str("SELLER_DONT_LIKE_YOU"),
            Self::DISTANCE_TOO_FAR => f.write_str("DISTANCE_TOO_FAR"),
            Self::ITEM_SOLD_OUT => f.write_str("ITEM_SOLD_OUT"),
            Self::CANT_CARRY_MORE => f.write_str("CANT_CARRY_MORE"),
            Self::RANK_REQUIRE => f.write_str("RANK_REQUIRE"),
            Self::REPUTATION_REQUIRE => f.write_str("REPUTATION_REQUIRE"),
        }
    }
}

impl TryFrom<u8> for BuyResult {
    type Error = TryFromBuyResultError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::CANT_FIND_ITEM),
            1 => Ok(Self::ITEM_ALREADY_SOLD),
            2 => Ok(Self::NOT_ENOUGHT_MONEY),
            4 => Ok(Self::SELLER_DONT_LIKE_YOU),
            5 => Ok(Self::DISTANCE_TOO_FAR),
            7 => Ok(Self::ITEM_SOLD_OUT),
            8 => Ok(Self::CANT_CARRY_MORE),
            11 => Ok(Self::RANK_REQUIRE),
            12 => Ok(Self::REPUTATION_REQUIRE),
            _ => Err(TryFromBuyResultError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromBuyResultError {
    value: u8,
}

impl TryFromBuyResultError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum BuyResultError {
    Read(std::io::Error),
    TryFrom(TryFromBuyResultError),
}

impl std::error::Error for BuyResultError {}
impl std::fmt::Display for TryFromBuyResultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'BuyResult': '{}'", self.value))
    }
}

impl std::fmt::Display for BuyResultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for BuyResultError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromBuyResultError> for BuyResultError {
    fn from(value: TryFromBuyResultError) -> Self {
        Self::TryFrom(value)
    }
}

