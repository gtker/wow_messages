use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{RaidTargetIndex, RaidTargetIndexError};
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
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

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl ReadableAndWritable for RaidTargetUpdate {
    type Error = RaidTargetUpdateError;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // index: RaidTargetIndex
        let index = RaidTargetIndex::read(r)?;

        // guid: Guid
        let guid = Guid::read(r)?;

        Ok(Self {
            index,
            guid,
        })
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // index: RaidTargetIndex
        self.index.write(w)?;

        // guid: Guid
        self.guid.write(w)?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // index: RaidTargetIndex
        let index = RaidTargetIndex::tokio_read(r).await?;

        // guid: Guid
        let guid = Guid::tokio_read(r).await?;

        Ok(Self {
            index,
            guid,
        })
    }

    fn tokio_write<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // index: RaidTargetIndex
            self.index.tokio_write(w).await?;

            // guid: Guid
            self.guid.tokio_write(w).await?;

            Ok(())
        })
    }
    #[cfg(feature = "async_std")]
    async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // index: RaidTargetIndex
        let index = RaidTargetIndex::astd_read(r).await?;

        // guid: Guid
        let guid = Guid::astd_read(r).await?;

        Ok(Self {
            index,
            guid,
        })
    }

    fn astd_write<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // index: RaidTargetIndex
            self.index.astd_write(w).await?;

            // guid: Guid
            self.guid.astd_write(w).await?;

            Ok(())
        })
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

