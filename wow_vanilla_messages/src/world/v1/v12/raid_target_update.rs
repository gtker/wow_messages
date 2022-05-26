use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::RaidTargetIndex;
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
    pub(crate) fn as_bytes(&self) -> Result<[u8; 9], std::io::Error> {
        let mut array_w = [0u8; 9];
        let mut w = array_w.as_mut_slice();
        // index: RaidTargetIndex
        w.write_all(&(self.index.as_int() as u8).to_le_bytes())?;

        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(array_w)
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

#[derive(Debug)]
pub enum RaidTargetUpdateError {
    Io(std::io::Error),
    Enum(crate::errors::EnumError),
}

impl std::error::Error for RaidTargetUpdateError {}
impl std::fmt::Display for RaidTargetUpdateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::Enum(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for RaidTargetUpdateError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<crate::errors::EnumError> for RaidTargetUpdateError {
    fn from(e: crate::errors::EnumError) -> Self {
        Self::Enum(e)
    }
}

