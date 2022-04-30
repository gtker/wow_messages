use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use crate::AsyncReadWrite;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum Class {
    WARRIOR,
    PALADIN,
    HUNTER,
    ROGUE,
    PRIEST,
    SHAMAN,
    MAGE,
    WARLOCK,
    DRUID,
}

impl ReadableAndWritable for Class {
    type Error = ClassError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u8_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u8().to_le_bytes())?;
        Ok(())
    }

}

#[cfg(any(feature = "async_tokio", feature = "async_std"))]
#[async_trait]
impl AsyncReadWrite for Class {
    type Error = ClassError;

    async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::tokio_read_u8_le(r).await?;

        Ok(a.try_into()?)
    }

}

impl Class {
    pub fn read_u16_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, ClassError> {
        let a = crate::util::read_u16_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_le(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u16_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, ClassError> {
        let a = crate::util::read_u16_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_be(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u32_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, ClassError> {
        let a = crate::util::read_u32_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_le(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, ClassError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, ClassError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, ClassError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub const fn as_u8(&self) -> u8 {
        match self {
            Self::WARRIOR => 0x1,
            Self::PALADIN => 0x2,
            Self::HUNTER => 0x3,
            Self::ROGUE => 0x4,
            Self::PRIEST => 0x5,
            Self::SHAMAN => 0x7,
            Self::MAGE => 0x8,
            Self::WARLOCK => 0x9,
            Self::DRUID => 0xb,
        }
    }

    pub const fn new() -> Self {
        Self::WARRIOR
    }

}

impl ConstantSized for Class {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for Class {
    fn maximum_possible_size() -> usize {
        1
    }
}

impl Default for Class {
    fn default() -> Self {
        Self::WARRIOR
    }
}

impl std::fmt::Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WARRIOR => f.write_str("WARRIOR"),
            Self::PALADIN => f.write_str("PALADIN"),
            Self::HUNTER => f.write_str("HUNTER"),
            Self::ROGUE => f.write_str("ROGUE"),
            Self::PRIEST => f.write_str("PRIEST"),
            Self::SHAMAN => f.write_str("SHAMAN"),
            Self::MAGE => f.write_str("MAGE"),
            Self::WARLOCK => f.write_str("WARLOCK"),
            Self::DRUID => f.write_str("DRUID"),
        }
    }
}

impl TryFrom<u8> for Class {
    type Error = TryFromClassError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::WARRIOR),
            2 => Ok(Self::PALADIN),
            3 => Ok(Self::HUNTER),
            4 => Ok(Self::ROGUE),
            5 => Ok(Self::PRIEST),
            7 => Ok(Self::SHAMAN),
            8 => Ok(Self::MAGE),
            9 => Ok(Self::WARLOCK),
            11 => Ok(Self::DRUID),
            _ => Err(TryFromClassError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromClassError {
    value: u8,
}

impl TryFromClassError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum ClassError {
    Read(std::io::Error),
    TryFrom(TryFromClassError),
}

impl std::error::Error for ClassError {}
impl std::fmt::Display for TryFromClassError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'Class': '{}'", self.value))
    }
}

impl std::fmt::Display for ClassError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for ClassError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromClassError> for ClassError {
    fn from(value: TryFromClassError) -> Self {
        Self::TryFrom(value)
    }
}

