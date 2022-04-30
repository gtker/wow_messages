use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use crate::AsyncReadWrite;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum PetitionResult {
    OK,
    ALREADY_SIGNED,
    ALREADY_IN_GUILD,
    CANT_SIGN_OWN,
    NEED_MORE,
    NOT_SERVER,
}

impl ReadableAndWritable for PetitionResult {
    type Error = PetitionResultError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u32_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u32().to_le_bytes())?;
        Ok(())
    }

}

#[cfg(any(feature = "async_tokio", feature = "async_std"))]
#[async_trait]
impl AsyncReadWrite for PetitionResult {
    type Error = PetitionResultError;

    #[cfg(feature = "async_tokio")]
    async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::tokio_read_u32_le(r).await?;

        Ok(a.try_into()?)
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u32().to_le_bytes()).await?;
        Ok(())
    }

}

impl PetitionResult {
    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, PetitionResultError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub async fn tokio_read_u32_be<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, PetitionResultError> {
        let a = crate::util::tokio_read_u32_be(r).await?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u32() as u32)?;
        Ok(())
    }

    pub async fn tokio_write_u32_be<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::tokio_write_u32_be(w, self.as_u32() as u32).await?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, PetitionResultError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u32).try_into()?)
    }

    pub async fn tokio_read_u64_le<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, PetitionResultError> {
        let a = crate::util::tokio_read_u64_le(r).await?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub async fn tokio_write_u64_le<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::tokio_write_u64_le(w, self.as_u32() as u64).await?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, PetitionResultError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub async fn tokio_read_u64_be<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, PetitionResultError> {
        let a = crate::util::tokio_read_u64_be(r).await?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub async fn tokio_write_u64_be<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::tokio_write_u64_be(w, self.as_u32() as u64).await?;
        Ok(())
    }

    pub const fn as_u32(&self) -> u32 {
        match self {
            Self::OK => 0x0,
            Self::ALREADY_SIGNED => 0x1,
            Self::ALREADY_IN_GUILD => 0x2,
            Self::CANT_SIGN_OWN => 0x3,
            Self::NEED_MORE => 0x4,
            Self::NOT_SERVER => 0x5,
        }
    }

    pub const fn new() -> Self {
        Self::OK
    }

}

impl ConstantSized for PetitionResult {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for PetitionResult {
    fn maximum_possible_size() -> usize {
        4
    }
}

impl Default for PetitionResult {
    fn default() -> Self {
        Self::OK
    }
}

impl std::fmt::Display for PetitionResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OK => f.write_str("OK"),
            Self::ALREADY_SIGNED => f.write_str("ALREADY_SIGNED"),
            Self::ALREADY_IN_GUILD => f.write_str("ALREADY_IN_GUILD"),
            Self::CANT_SIGN_OWN => f.write_str("CANT_SIGN_OWN"),
            Self::NEED_MORE => f.write_str("NEED_MORE"),
            Self::NOT_SERVER => f.write_str("NOT_SERVER"),
        }
    }
}

impl TryFrom<u32> for PetitionResult {
    type Error = TryFromPetitionResultError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::OK),
            1 => Ok(Self::ALREADY_SIGNED),
            2 => Ok(Self::ALREADY_IN_GUILD),
            3 => Ok(Self::CANT_SIGN_OWN),
            4 => Ok(Self::NEED_MORE),
            5 => Ok(Self::NOT_SERVER),
            _ => Err(TryFromPetitionResultError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromPetitionResultError {
    value: u32,
}

impl TryFromPetitionResultError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum PetitionResultError {
    Read(std::io::Error),
    TryFrom(TryFromPetitionResultError),
}

impl std::error::Error for PetitionResultError {}
impl std::fmt::Display for TryFromPetitionResultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'PetitionResult': '{}'", self.value))
    }
}

impl std::fmt::Display for PetitionResultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for PetitionResultError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromPetitionResultError> for PetitionResultError {
    fn from(value: TryFromPetitionResultError) -> Self {
        Self::TryFrom(value)
    }
}

