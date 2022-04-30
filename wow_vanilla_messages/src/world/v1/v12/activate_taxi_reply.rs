use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use crate::AsyncReadWrite;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum ActivateTaxiReply {
    OK,
    UNSPECIFIEDSERVERERROR,
    NOSUCHPATH,
    NOTENOUGHMONEY,
    TOOFARAWAY,
    NOVENDORNEARBY,
    NOTVISITED,
    PLAYERBUSY,
    PLAYERALREADYMOUNTED,
    PLAYERSHAPESHIFTED,
    PLAYERMOVING,
    SAMENODE,
    NOTSTANDING,
}

impl ReadableAndWritable for ActivateTaxiReply {
    type Error = ActivateTaxiReplyError;

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
impl AsyncReadWrite for ActivateTaxiReply {
    type Error = ActivateTaxiReplyError;

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

impl ActivateTaxiReply {
    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, ActivateTaxiReplyError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub async fn tokio_read_u32_be<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, ActivateTaxiReplyError> {
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

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, ActivateTaxiReplyError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u32).try_into()?)
    }

    pub async fn tokio_read_u64_le<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, ActivateTaxiReplyError> {
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

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, ActivateTaxiReplyError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub async fn tokio_read_u64_be<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, ActivateTaxiReplyError> {
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
            Self::UNSPECIFIEDSERVERERROR => 0x1,
            Self::NOSUCHPATH => 0x2,
            Self::NOTENOUGHMONEY => 0x3,
            Self::TOOFARAWAY => 0x4,
            Self::NOVENDORNEARBY => 0x5,
            Self::NOTVISITED => 0x6,
            Self::PLAYERBUSY => 0x7,
            Self::PLAYERALREADYMOUNTED => 0x8,
            Self::PLAYERSHAPESHIFTED => 0x9,
            Self::PLAYERMOVING => 0xa,
            Self::SAMENODE => 0xb,
            Self::NOTSTANDING => 0xc,
        }
    }

    pub const fn new() -> Self {
        Self::OK
    }

}

impl ConstantSized for ActivateTaxiReply {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for ActivateTaxiReply {
    fn maximum_possible_size() -> usize {
        4
    }
}

impl Default for ActivateTaxiReply {
    fn default() -> Self {
        Self::OK
    }
}

impl std::fmt::Display for ActivateTaxiReply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OK => f.write_str("OK"),
            Self::UNSPECIFIEDSERVERERROR => f.write_str("UNSPECIFIEDSERVERERROR"),
            Self::NOSUCHPATH => f.write_str("NOSUCHPATH"),
            Self::NOTENOUGHMONEY => f.write_str("NOTENOUGHMONEY"),
            Self::TOOFARAWAY => f.write_str("TOOFARAWAY"),
            Self::NOVENDORNEARBY => f.write_str("NOVENDORNEARBY"),
            Self::NOTVISITED => f.write_str("NOTVISITED"),
            Self::PLAYERBUSY => f.write_str("PLAYERBUSY"),
            Self::PLAYERALREADYMOUNTED => f.write_str("PLAYERALREADYMOUNTED"),
            Self::PLAYERSHAPESHIFTED => f.write_str("PLAYERSHAPESHIFTED"),
            Self::PLAYERMOVING => f.write_str("PLAYERMOVING"),
            Self::SAMENODE => f.write_str("SAMENODE"),
            Self::NOTSTANDING => f.write_str("NOTSTANDING"),
        }
    }
}

impl TryFrom<u32> for ActivateTaxiReply {
    type Error = TryFromActivateTaxiReplyError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::OK),
            1 => Ok(Self::UNSPECIFIEDSERVERERROR),
            2 => Ok(Self::NOSUCHPATH),
            3 => Ok(Self::NOTENOUGHMONEY),
            4 => Ok(Self::TOOFARAWAY),
            5 => Ok(Self::NOVENDORNEARBY),
            6 => Ok(Self::NOTVISITED),
            7 => Ok(Self::PLAYERBUSY),
            8 => Ok(Self::PLAYERALREADYMOUNTED),
            9 => Ok(Self::PLAYERSHAPESHIFTED),
            10 => Ok(Self::PLAYERMOVING),
            11 => Ok(Self::SAMENODE),
            12 => Ok(Self::NOTSTANDING),
            _ => Err(TryFromActivateTaxiReplyError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromActivateTaxiReplyError {
    value: u32,
}

impl TryFromActivateTaxiReplyError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum ActivateTaxiReplyError {
    Read(std::io::Error),
    TryFrom(TryFromActivateTaxiReplyError),
}

impl std::error::Error for ActivateTaxiReplyError {}
impl std::fmt::Display for TryFromActivateTaxiReplyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'ActivateTaxiReply': '{}'", self.value))
    }
}

impl std::fmt::Display for ActivateTaxiReplyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for ActivateTaxiReplyError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromActivateTaxiReplyError> for ActivateTaxiReplyError {
    fn from(value: TryFromActivateTaxiReplyError) -> Self {
        Self::TryFrom(value)
    }
}

