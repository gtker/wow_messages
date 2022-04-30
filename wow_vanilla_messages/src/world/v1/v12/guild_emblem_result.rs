use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use crate::AsyncReadWrite;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum GuildEmblemResult {
    SUCCESS,
    INVALID_TABARD_COLORS,
    NO_GUILD,
    NOT_GUILD_MASTER,
    NOT_ENOUGH_MONEY,
    NO_MESSAGE,
}

impl ReadableAndWritable for GuildEmblemResult {
    type Error = GuildEmblemResultError;

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
impl AsyncReadWrite for GuildEmblemResult {
    type Error = GuildEmblemResultError;

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

impl GuildEmblemResult {
    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, GuildEmblemResultError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub async fn tokio_read_u32_be<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, GuildEmblemResultError> {
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

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, GuildEmblemResultError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u32).try_into()?)
    }

    pub async fn tokio_read_u64_le<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, GuildEmblemResultError> {
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

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, GuildEmblemResultError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub async fn tokio_read_u64_be<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, GuildEmblemResultError> {
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
            Self::SUCCESS => 0x0,
            Self::INVALID_TABARD_COLORS => 0x1,
            Self::NO_GUILD => 0x2,
            Self::NOT_GUILD_MASTER => 0x3,
            Self::NOT_ENOUGH_MONEY => 0x4,
            Self::NO_MESSAGE => 0x5,
        }
    }

    pub const fn new() -> Self {
        Self::SUCCESS
    }

}

impl ConstantSized for GuildEmblemResult {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for GuildEmblemResult {
    fn maximum_possible_size() -> usize {
        4
    }
}

impl Default for GuildEmblemResult {
    fn default() -> Self {
        Self::SUCCESS
    }
}

impl std::fmt::Display for GuildEmblemResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SUCCESS => f.write_str("SUCCESS"),
            Self::INVALID_TABARD_COLORS => f.write_str("INVALID_TABARD_COLORS"),
            Self::NO_GUILD => f.write_str("NO_GUILD"),
            Self::NOT_GUILD_MASTER => f.write_str("NOT_GUILD_MASTER"),
            Self::NOT_ENOUGH_MONEY => f.write_str("NOT_ENOUGH_MONEY"),
            Self::NO_MESSAGE => f.write_str("NO_MESSAGE"),
        }
    }
}

impl TryFrom<u32> for GuildEmblemResult {
    type Error = TryFromGuildEmblemResultError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::SUCCESS),
            1 => Ok(Self::INVALID_TABARD_COLORS),
            2 => Ok(Self::NO_GUILD),
            3 => Ok(Self::NOT_GUILD_MASTER),
            4 => Ok(Self::NOT_ENOUGH_MONEY),
            5 => Ok(Self::NO_MESSAGE),
            _ => Err(TryFromGuildEmblemResultError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromGuildEmblemResultError {
    value: u32,
}

impl TryFromGuildEmblemResultError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum GuildEmblemResultError {
    Read(std::io::Error),
    TryFrom(TryFromGuildEmblemResultError),
}

impl std::error::Error for GuildEmblemResultError {}
impl std::fmt::Display for TryFromGuildEmblemResultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'GuildEmblemResult': '{}'", self.value))
    }
}

impl std::fmt::Display for GuildEmblemResultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for GuildEmblemResultError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromGuildEmblemResultError> for GuildEmblemResultError {
    fn from(value: TryFromGuildEmblemResultError) -> Self {
        Self::TryFrom(value)
    }
}

