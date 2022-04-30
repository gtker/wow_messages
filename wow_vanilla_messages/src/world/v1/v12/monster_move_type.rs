use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use crate::AsyncReadWrite;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum MonsterMoveType {
    NORMAL,
    STOP,
    FACING_SPOT,
    FACING_TARGET,
    FACING_ANGLE,
}

impl ReadableAndWritable for MonsterMoveType {
    type Error = MonsterMoveTypeError;

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
impl AsyncReadWrite for MonsterMoveType {
    type Error = MonsterMoveTypeError;

    async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::tokio_read_u8_le(r).await?;

        Ok(a.try_into()?)
    }

}

impl MonsterMoveType {
    pub fn read_u16_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, MonsterMoveTypeError> {
        let a = crate::util::read_u16_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_le(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u16_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, MonsterMoveTypeError> {
        let a = crate::util::read_u16_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u16_be(w, self.as_u8() as u16)?;
        Ok(())
    }

    pub fn read_u32_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, MonsterMoveTypeError> {
        let a = crate::util::read_u32_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_le(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, MonsterMoveTypeError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u8() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, MonsterMoveTypeError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u8).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u8() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, MonsterMoveTypeError> {
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
            Self::STOP => 0x1,
            Self::FACING_SPOT => 0x2,
            Self::FACING_TARGET => 0x3,
            Self::FACING_ANGLE => 0x4,
        }
    }

    pub const fn new() -> Self {
        Self::NORMAL
    }

}

impl ConstantSized for MonsterMoveType {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for MonsterMoveType {
    fn maximum_possible_size() -> usize {
        1
    }
}

impl Default for MonsterMoveType {
    fn default() -> Self {
        Self::NORMAL
    }
}

impl std::fmt::Display for MonsterMoveType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NORMAL => f.write_str("NORMAL"),
            Self::STOP => f.write_str("STOP"),
            Self::FACING_SPOT => f.write_str("FACING_SPOT"),
            Self::FACING_TARGET => f.write_str("FACING_TARGET"),
            Self::FACING_ANGLE => f.write_str("FACING_ANGLE"),
        }
    }
}

impl TryFrom<u8> for MonsterMoveType {
    type Error = TryFromMonsterMoveTypeError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NORMAL),
            1 => Ok(Self::STOP),
            2 => Ok(Self::FACING_SPOT),
            3 => Ok(Self::FACING_TARGET),
            4 => Ok(Self::FACING_ANGLE),
            _ => Err(TryFromMonsterMoveTypeError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromMonsterMoveTypeError {
    value: u8,
}

impl TryFromMonsterMoveTypeError {
    pub const fn new(value: u8) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum MonsterMoveTypeError {
    Read(std::io::Error),
    TryFrom(TryFromMonsterMoveTypeError),
}

impl std::error::Error for MonsterMoveTypeError {}
impl std::fmt::Display for TryFromMonsterMoveTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'MonsterMoveType': '{}'", self.value))
    }
}

impl std::fmt::Display for MonsterMoveTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for MonsterMoveTypeError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromMonsterMoveTypeError> for MonsterMoveTypeError {
    fn from(value: TryFromMonsterMoveTypeError) -> Self {
        Self::TryFrom(value)
    }
}

