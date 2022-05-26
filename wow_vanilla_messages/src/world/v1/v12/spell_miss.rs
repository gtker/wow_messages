use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::SpellMissInfo;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SpellMiss {
    pub target_guid: Guid,
    pub miss_info: SpellMissInfo,
}

impl SpellMiss {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 12], std::io::Error> {
        let mut array_w = [0u8; 12];
        let mut w = array_w.as_mut_slice();
        // target_guid: Guid
        w.write_all(&self.target_guid.guid().to_le_bytes())?;

        // miss_info: SpellMissInfo
        w.write_all(&(self.miss_info.as_int() as u32).to_le_bytes())?;

        Ok(array_w)
    }
}

impl SpellMiss {
    #[cfg(feature = "sync")]
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

    #[cfg(feature = "tokio")]
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

    #[cfg(feature = "async-std")]
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

}

#[derive(Debug)]
pub enum SpellMissError {
    Io(std::io::Error),
    Enum(crate::errors::EnumError),
}

impl std::error::Error for SpellMissError {}
impl std::fmt::Display for SpellMissError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::Enum(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for SpellMissError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<crate::errors::EnumError> for SpellMissError {
    fn from(e: crate::errors::EnumError) -> Self {
        Self::Enum(e)
    }
}

