use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{SpellMissInfo, SpellMissInfoError};
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use crate::AsyncReadWrite;
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SpellMiss {
    pub target_guid: Guid,
    pub miss_info: SpellMissInfo,
}

impl ReadableAndWritable for SpellMiss {
    type Error = SpellMissError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // target_guid: Guid
        let target_guid = Guid::read(r)?;

        // miss_info: SpellMissInfo
        let miss_info = SpellMissInfo::read(r)?;

        Ok(Self {
            target_guid,
            miss_info,
        })
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // target_guid: Guid
        self.target_guid.write(w)?;

        // miss_info: SpellMissInfo
        self.miss_info.write(w)?;

        Ok(())
    }

}

#[cfg(any(feature = "async_tokio", feature = "async_std"))]
#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl AsyncReadWrite for SpellMiss {
    type Error = SpellMissError;

    #[cfg(feature = "async_tokio")]
    async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // target_guid: Guid
        let target_guid = Guid::tokio_read(r).await?;

        // miss_info: SpellMissInfo
        let miss_info = SpellMissInfo::tokio_read(r).await?;

        Ok(Self {
            target_guid,
            miss_info,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // target_guid: Guid
        self.target_guid.tokio_write(w).await?;

        // miss_info: SpellMissInfo
        self.miss_info.tokio_write(w).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // target_guid: Guid
        let target_guid = Guid::astd_read(r).await?;

        // miss_info: SpellMissInfo
        let miss_info = SpellMissInfo::astd_read(r).await?;

        Ok(Self {
            target_guid,
            miss_info,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // target_guid: Guid
        self.target_guid.astd_write(w).await?;

        // miss_info: SpellMissInfo
        self.miss_info.astd_write(w).await?;

        Ok(())
    }

}

impl ConstantSized for SpellMiss {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SpellMiss {
    fn maximum_possible_size() -> usize {
        8 // target_guid: Guid
        + SpellMissInfo::size() // miss_info: SpellMissInfo
    }
}

#[derive(Debug)]
pub enum SpellMissError {
    Io(std::io::Error),
    SpellMissInfo(SpellMissInfoError),
}

impl std::error::Error for SpellMissError {}
impl std::fmt::Display for SpellMissError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::SpellMissInfo(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SpellMissError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<SpellMissInfoError> for SpellMissError {
    fn from(e: SpellMissInfoError) -> Self {
        Self::SpellMissInfo(e)
    }
}

