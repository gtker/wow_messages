use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{RaidTargetIndex, RaidTargetIndexError};
use crate::{ConstantSized, MaximumPossibleSized, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct RaidTargetUpdate {
    pub index: RaidTargetIndex,
    pub guid: Guid,
}

impl RaidTargetUpdate {
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

    pub(crate) fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // index: RaidTargetIndex
        w.write_all(&(self.index.as_int() as u8).to_le_bytes())?;

        // guid: Guid
        self.guid.write(w)?;

        Ok(())
    }

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

    pub(crate) async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // index: RaidTargetIndex
        w.write_all(&(self.index.as_int() as u8).to_le_bytes()).await?;

        // guid: Guid
        self.guid.tokio_write(w).await?;

        Ok(())
    }

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

    pub(crate) async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // index: RaidTargetIndex
        w.write_all(&(self.index.as_int() as u8).to_le_bytes()).await?;

        // guid: Guid
        self.guid.astd_write(w).await?;

        Ok(())
    }

}

impl ConstantSized for RaidTargetUpdate {}

impl MaximumPossibleSized for RaidTargetUpdate {
    fn maximum_possible_size() -> usize {
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

