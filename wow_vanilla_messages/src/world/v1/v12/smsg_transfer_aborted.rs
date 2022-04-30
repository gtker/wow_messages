use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{Map, MapError};
use crate::world::v1::v12::{TransferAbortReason, TransferAbortReasonError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_TRANSFER_ABORTED {
    pub map: Map,
    pub reason: TransferAbortReason,
}

impl ServerMessageWrite for SMSG_TRANSFER_ABORTED {}

impl SMSG_TRANSFER_ABORTED {
    pub const PADDING_VALUE: u8 = 0x00;

}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_TRANSFER_ABORTED {
    const OPCODE: u16 = 0x0040;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = SMSG_TRANSFER_ABORTEDError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // map: Map
        let map = Map::read(r)?;

        // reason: TransferAbortReason
        let reason = TransferAbortReason::read(r)?;

        // padding: u8
        let _padding = crate::util::read_u8_le(r)?;
        // padding is expected to always be 0 (0)

        Ok(Self {
            map,
            reason,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // map: Map
        self.map.write(w)?;

        // reason: TransferAbortReason
        self.reason.write(w)?;

        // padding: u8
        w.write_all(&Self::PADDING_VALUE.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // map: Map
        let map = Map::tokio_read(r).await?;

        // reason: TransferAbortReason
        let reason = TransferAbortReason::tokio_read(r).await?;

        // padding: u8
        let _padding = crate::util::tokio_read_u8_le(r).await?;
        // padding is expected to always be 0 (0)

        Ok(Self {
            map,
            reason,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // map: Map
        self.map.tokio_write(w).await?;

        // reason: TransferAbortReason
        self.reason.tokio_write(w).await?;

        // padding: u8
        w.write_all(&Self::PADDING_VALUE.to_le_bytes()).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // map: Map
        let map = Map::astd_read(r).await?;

        // reason: TransferAbortReason
        let reason = TransferAbortReason::astd_read(r).await?;

        // padding: u8
        let _padding = crate::util::astd_read_u8_le(r).await?;
        // padding is expected to always be 0 (0)

        Ok(Self {
            map,
            reason,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // map: Map
        self.map.astd_write(w).await?;

        // reason: TransferAbortReason
        self.reason.astd_write(w).await?;

        // padding: u8
        w.write_all(&Self::PADDING_VALUE.to_le_bytes()).await?;

        Ok(())
    }

}

impl ConstantSized for SMSG_TRANSFER_ABORTED {}

impl MaximumPossibleSized for SMSG_TRANSFER_ABORTED {
    fn maximum_possible_size() -> usize {
        Map::size() // map: Map
        + TransferAbortReason::size() // reason: TransferAbortReason
        + 1 // padding: u8
    }
}

#[derive(Debug)]
pub enum SMSG_TRANSFER_ABORTEDError {
    Io(std::io::Error),
    Map(MapError),
    TransferAbortReason(TransferAbortReasonError),
}

impl std::error::Error for SMSG_TRANSFER_ABORTEDError {}
impl std::fmt::Display for SMSG_TRANSFER_ABORTEDError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::Map(i) => i.fmt(f),
            Self::TransferAbortReason(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_TRANSFER_ABORTEDError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<MapError> for SMSG_TRANSFER_ABORTEDError {
    fn from(e: MapError) -> Self {
        Self::Map(e)
    }
}

impl From<TransferAbortReasonError> for SMSG_TRANSFER_ABORTEDError {
    fn from(e: TransferAbortReasonError) -> Self {
        Self::TransferAbortReason(e)
    }
}

