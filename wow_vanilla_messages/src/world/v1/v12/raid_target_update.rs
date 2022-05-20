use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{RaidTargetIndex, RaidTargetIndexError};
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct RaidTargetUpdate {
    pub index: RaidTargetIndex,
    pub guid: Guid,
}

impl RaidTargetUpdate {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(8000);
        // index: RaidTargetIndex
        w.write_all(&(self.index.as_int() as u8).to_le_bytes())?;

        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(w)
    }
}

impl RaidTargetUpdate {
    #[cfg(feature = "sync")]
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, RaidTargetUpdateError> {
        // index: RaidTargetIndex
        let index: RaidTargetIndex = crate::util::read_u8_le(r)?.try_into()?;

        // guid: Guid
        let guid = Guid::read(r)?;

        Ok(Self {
            index,
            guid,
        })
    }

    #[cfg(feature = "tokio")]
    pub(crate) async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, RaidTargetUpdateError> {
        // index: RaidTargetIndex
        let index: RaidTargetIndex = crate::util::tokio_read_u8_le(r).await?.try_into()?;

        // guid: Guid
        let guid = Guid::tokio_read(r).await?;

        Ok(Self {
            index,
            guid,
        })
    }

    #[cfg(feature = "async-std")]
    pub(crate) async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, RaidTargetUpdateError> {
        // index: RaidTargetIndex
        let index: RaidTargetIndex = crate::util::astd_read_u8_le(r).await?.try_into()?;

        // guid: Guid
        let guid = Guid::astd_read(r).await?;

        Ok(Self {
            index,
            guid,
        })
    }

}

impl RaidTargetUpdate {
    pub(crate) fn size() -> usize {
        0
        + 1 // index: RaidTargetIndex
        + 8 // guid: Guid
    }
}

#[derive(Debug)]
pub enum RaidTargetUpdateError {
    Io(std::io::Error),
    RaidTargetIndex(RaidTargetIndexError),
}

impl std::error::Error for RaidTargetUpdateError {}
impl std::fmt::Display for RaidTargetUpdateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::RaidTargetIndex(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for RaidTargetUpdateError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<RaidTargetIndexError> for RaidTargetUpdateError {
    fn from(e: RaidTargetIndexError) -> Self {
        Self::RaidTargetIndex(e)
    }
}

