use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{InstanceResetFailedReason, InstanceResetFailedReasonError};
use crate::world::v1::v12::{Map, MapError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_INSTANCE_RESET_FAILED {
    pub reason: InstanceResetFailedReason,
    pub map: Map,
}

impl ServerMessageWrite for SMSG_INSTANCE_RESET_FAILED {}

impl MessageBody for SMSG_INSTANCE_RESET_FAILED {
    const OPCODE: u16 = 0x031f;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = SMSG_INSTANCE_RESET_FAILEDError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // reason: InstanceResetFailedReason
        let reason = InstanceResetFailedReason::read(r)?;

        // map: Map
        let map = Map::read(r)?;

        Ok(Self {
            reason,
            map,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // reason: InstanceResetFailedReason
        self.reason.write(w)?;

        // map: Map
        self.map.write(w)?;

        Ok(())
    }

    fn tokio_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // reason: InstanceResetFailedReason
            let reason = InstanceResetFailedReason::tokio_read(r).await?;

            // map: Map
            let map = Map::tokio_read(r).await?;

            Ok(Self {
                reason,
                map,
            })
        })
    }

    fn tokio_write_body<'life0, 'life1, 'async_trait, W>(
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
            // reason: InstanceResetFailedReason
            self.reason.tokio_write(w).await?;

            // map: Map
            self.map.tokio_write(w).await?;

            Ok(())
        })
    }

    fn astd_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // reason: InstanceResetFailedReason
            let reason = InstanceResetFailedReason::astd_read(r).await?;

            // map: Map
            let map = Map::astd_read(r).await?;

            Ok(Self {
                reason,
                map,
            })
        })
    }

    fn astd_write_body<'life0, 'life1, 'async_trait, W>(
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
            // reason: InstanceResetFailedReason
            self.reason.astd_write(w).await?;

            // map: Map
            self.map.astd_write(w).await?;

            Ok(())
        })
    }

}

impl ConstantSized for SMSG_INSTANCE_RESET_FAILED {}

impl MaximumPossibleSized for SMSG_INSTANCE_RESET_FAILED {
    fn maximum_possible_size() -> usize {
        0
        + 1 // reason: InstanceResetFailedReason
        + 4 // map: Map
    }
}

#[derive(Debug)]
pub enum SMSG_INSTANCE_RESET_FAILEDError {
    Io(std::io::Error),
    InstanceResetFailedReason(InstanceResetFailedReasonError),
    Map(MapError),
}

impl std::error::Error for SMSG_INSTANCE_RESET_FAILEDError {}
impl std::fmt::Display for SMSG_INSTANCE_RESET_FAILEDError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::InstanceResetFailedReason(i) => i.fmt(f),
            Self::Map(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_INSTANCE_RESET_FAILEDError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<InstanceResetFailedReasonError> for SMSG_INSTANCE_RESET_FAILEDError {
    fn from(e: InstanceResetFailedReasonError) -> Self {
        Self::InstanceResetFailedReason(e)
    }
}

impl From<MapError> for SMSG_INSTANCE_RESET_FAILEDError {
    fn from(e: MapError) -> Self {
        Self::Map(e)
    }
}

