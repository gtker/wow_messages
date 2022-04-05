use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/add_messages.wowm:260`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/add_messages.wowm#L260):
/// ```text
/// enum ItemQuality : u8 {
///     POOR = 0;
///     NORMAL = 1;
///     UNCOMMON = 2;
///     RARE = 3;
///     EPIC = 4;
///     LEGENDARY = 5;
///     ARTIFACT = 6;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum ItemQuality {
    POOR,
    NORMAL,
    UNCOMMON,
    RARE,
    EPIC,
    LEGENDARY,
    ARTIFACT,
}

impl ReadableAndWritable for ItemQuality {
    type Error = ItemQualityError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u8_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u8().to_le_bytes())?;
        Ok(())
    }

}

impl ItemQuality {
    pub fn read_u16_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, ItemQualityError> {
        let a = crate::util::read_u16_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_le(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u16_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, ItemQualityError> {
        let a = crate::util::read_u16_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_be(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u32_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, ItemQualityError> {
        let a = crate::util::read_u32_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_le(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, ItemQualityError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, ItemQualityError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, ItemQualityError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub const fn as_u8(&self) -> u8 {
        match self {
            Self::POOR => 0x0,
            Self::NORMAL => 0x1,
            Self::UNCOMMON => 0x2,
            Self::RARE => 0x3,
            Self::EPIC => 0x4,
            Self::LEGENDARY => 0x5,
            Self::ARTIFACT => 0x6,
        }
    }

    pub const fn new() -> Self {
        Self::POOR
    }

}

impl ConstantSized for ItemQuality {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for ItemQuality {
    fn maximum_possible_size() -> usize {
        1
    }
}

impl Default for ItemQuality {
    fn default() -> Self {
        Self::POOR
    }
}

impl std::fmt::Display for ItemQuality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::POOR => f.write_str("POOR"),
            Self::NORMAL => f.write_str("NORMAL"),
            Self::UNCOMMON => f.write_str("UNCOMMON"),
            Self::RARE => f.write_str("RARE"),
            Self::EPIC => f.write_str("EPIC"),
            Self::LEGENDARY => f.write_str("LEGENDARY"),
            Self::ARTIFACT => f.write_str("ARTIFACT"),
        }
    }
}

impl TryFrom<u8> for ItemQuality {
    type Error = TryFromItemQualityError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::POOR),
            1 => Ok(Self::NORMAL),
            2 => Ok(Self::UNCOMMON),
            3 => Ok(Self::RARE),
            4 => Ok(Self::EPIC),
            5 => Ok(Self::LEGENDARY),
            6 => Ok(Self::ARTIFACT),
            _ => Err(TryFromItemQualityError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromItemQualityError {
    value: u8,
}

impl TryFromItemQualityError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum ItemQualityError {
    Read(std::io::Error),
    TryFrom(TryFromItemQualityError),
}

impl std::error::Error for ItemQualityError {}
impl std::fmt::Display for TryFromItemQualityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'ItemQuality': '{}'", self.value))
    }
}

impl std::fmt::Display for ItemQualityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for ItemQualityError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromItemQualityError> for ItemQualityError {
    fn from(value: TryFromItemQualityError) -> Self {
        Self::TryFrom(value)
    }
}

