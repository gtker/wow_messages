use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{SpellMissInfo, SpellMissInfoError};
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

impl SpellMiss {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, SpellMissError> {
        // target_guid: Guid
        let target_guid = Guid::read(r)?;

        // miss_info: SpellMissInfo
        let miss_info: SpellMissInfo = crate::util::read_u32_le(r)?.try_into()?;

        Ok(Self {
            target_guid,
            miss_info,
        })
    }

    pub(crate) fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // target_guid: Guid
        self.target_guid.write(w)?;

        // miss_info: SpellMissInfo
        w.write_all(&(self.miss_info.as_int() as u32).to_le_bytes())?;

        Ok(())
    }

    pub(crate) async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, SpellMissError> {
        // target_guid: Guid
        let target_guid = Guid::tokio_read(r).await?;

        // miss_info: SpellMissInfo
        let miss_info: SpellMissInfo = crate::util::tokio_read_u32_le(r).await?.try_into()?;

        Ok(Self {
            target_guid,
            miss_info,
        })
    }

    pub(crate) async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // target_guid: Guid
        self.target_guid.tokio_write(w).await?;

        // miss_info: SpellMissInfo
        w.write_all(&(self.miss_info.as_int() as u32).to_le_bytes()).await?;

        Ok(())
    }

    pub(crate) async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, SpellMissError> {
        // target_guid: Guid
        let target_guid = Guid::astd_read(r).await?;

        // miss_info: SpellMissInfo
        let miss_info: SpellMissInfo = crate::util::astd_read_u32_le(r).await?.try_into()?;

        Ok(Self {
            target_guid,
            miss_info,
        })
    }

    pub(crate) async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // target_guid: Guid
        self.target_guid.astd_write(w).await?;

        // miss_info: SpellMissInfo
        w.write_all(&(self.miss_info.as_int() as u32).to_le_bytes()).await?;

        Ok(())
    }

}

impl SpellMiss {
    pub(crate) fn size() -> usize {
        0
        + 8 // target_guid: Guid
        + 4 // miss_info: SpellMissInfo
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

