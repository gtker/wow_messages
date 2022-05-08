use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{QuestGiverStatus, QuestGiverStatusError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_QUESTGIVER_STATUS {
    pub guid: Guid,
    pub status: QuestGiverStatus,
}

impl ServerMessageWrite for SMSG_QUESTGIVER_STATUS {}

impl MessageBody for SMSG_QUESTGIVER_STATUS {
    const OPCODE: u16 = 0x0183;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = SMSG_QUESTGIVER_STATUSError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // status: QuestGiverStatus
        let status = QuestGiverStatus::read_u32_le(r)?;

        Ok(Self {
            guid,
            status,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // status: QuestGiverStatus
        self.status.write_u32_le(w)?;

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
            // guid: Guid
            let guid = Guid::tokio_read(r).await?;

            // status: QuestGiverStatus
            let status = QuestGiverStatus::tokio_read_u32_le(r).await?;

            Ok(Self {
                guid,
                status,
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
            // guid: Guid
            self.guid.tokio_write(w).await?;

            // status: QuestGiverStatus
            self.status.tokio_write_u32_le(w).await?;

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
            // guid: Guid
            let guid = Guid::astd_read(r).await?;

            // status: QuestGiverStatus
            let status = QuestGiverStatus::astd_read_u32_le(r).await?;

            Ok(Self {
                guid,
                status,
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
            // guid: Guid
            self.guid.astd_write(w).await?;

            // status: QuestGiverStatus
            self.status.astd_write_u32_le(w).await?;

            Ok(())
        })
    }

}

impl ConstantSized for SMSG_QUESTGIVER_STATUS {}

impl MaximumPossibleSized for SMSG_QUESTGIVER_STATUS {
    fn maximum_possible_size() -> usize {
        0
        + 8 // guid: Guid
        + 1 // status: QuestGiverStatus
    }
}

#[derive(Debug)]
pub enum SMSG_QUESTGIVER_STATUSError {
    Io(std::io::Error),
    QuestGiverStatus(QuestGiverStatusError),
}

impl std::error::Error for SMSG_QUESTGIVER_STATUSError {}
impl std::fmt::Display for SMSG_QUESTGIVER_STATUSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::QuestGiverStatus(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_QUESTGIVER_STATUSError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<QuestGiverStatusError> for SMSG_QUESTGIVER_STATUSError {
    fn from(e: QuestGiverStatusError) -> Self {
        Self::QuestGiverStatus(e)
    }
}

