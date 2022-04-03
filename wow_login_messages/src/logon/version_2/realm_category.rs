use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_realm/sever.wowm:43`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_realm/sever.wowm#L43):
/// ```text
/// enum RealmCategory : u8 {
///     DEFAULT = 0x0;
///     ONE = 0x1;
///     TWO = 0x2;
///     THREE = 0x3;
///     FIVE = 0x5;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum RealmCategory {
    DEFAULT,
    ONE,
    TWO,
    THREE,
    FIVE,
}

impl ReadableAndWritable for RealmCategory {
    type Error = RealmCategoryError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u8_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u8().to_le_bytes())?;
        Ok(())
    }

}

impl RealmCategory {
    pub fn read_u16_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, RealmCategoryError> {
        let a = crate::util::read_u16_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_le(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u16_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, RealmCategoryError> {
        let a = crate::util::read_u16_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_be(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u32_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, RealmCategoryError> {
        let a = crate::util::read_u32_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_le(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, RealmCategoryError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, RealmCategoryError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, RealmCategoryError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub const fn as_u8(&self) -> u8 {
        match self {
            Self::DEFAULT => 0x0,
            Self::ONE => 0x1,
            Self::TWO => 0x2,
            Self::THREE => 0x3,
            Self::FIVE => 0x5,
        }
    }

    pub const fn new() -> Self {
        Self::DEFAULT
    }

}

impl ConstantSized for RealmCategory {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for RealmCategory {
    fn maximum_possible_size() -> usize {
        1
    }
}

impl Default for RealmCategory {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl std::fmt::Display for RealmCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DEFAULT => f.write_str("DEFAULT"),
            Self::ONE => f.write_str("ONE"),
            Self::TWO => f.write_str("TWO"),
            Self::THREE => f.write_str("THREE"),
            Self::FIVE => f.write_str("FIVE"),
        }
    }
}

impl TryFrom<u8> for RealmCategory {
    type Error = TryFromRealmCategoryError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::DEFAULT),
            1 => Ok(Self::ONE),
            2 => Ok(Self::TWO),
            3 => Ok(Self::THREE),
            5 => Ok(Self::FIVE),
            _ => Err(TryFromRealmCategoryError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromRealmCategoryError {
    value: u8,
}

impl TryFromRealmCategoryError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum RealmCategoryError {
    Read(std::io::Error),
    TryFrom(TryFromRealmCategoryError),
}

impl std::error::Error for RealmCategoryError {}
impl std::fmt::Display for TryFromRealmCategoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'RealmCategory': '{}'", self.value))
    }
}

impl std::fmt::Display for RealmCategoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for RealmCategoryError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromRealmCategoryError> for RealmCategoryError {
    fn from(value: TryFromRealmCategoryError) -> Self {
        Self::TryFrom(value)
    }
}

