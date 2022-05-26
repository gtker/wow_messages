use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{TrainingFailureReason, TrainingFailureReasonError};
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_TRAINER_BUY_FAILED {
    pub guid: Guid,
    pub id: u32,
    pub error: TrainingFailureReason,
}

impl SMSG_TRAINER_BUY_FAILED {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 16], std::io::Error> {
        let mut array_w = [0u8; 16];
        let mut w = array_w.as_mut_slice();
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // error: TrainingFailureReason
        w.write_all(&(self.error.as_int() as u32).to_le_bytes())?;

        Ok(array_w)
    }
}

impl ServerMessage for SMSG_TRAINER_BUY_FAILED {
    fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(16);
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // error: TrainingFailureReason
        w.write_all(&(self.error.as_int() as u32).to_le_bytes())?;

        Ok(w)
    }
    const OPCODE: u16 = 0x01b4;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        16
    }

    type Error = SMSG_TRAINER_BUY_FAILEDError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // id: u32
        let id = crate::util::read_u32_le(r)?;

        // error: TrainingFailureReason
        let error: TrainingFailureReason = crate::util::read_u32_le(r)?.try_into()?;

        Ok(Self {
            guid,
            id,
            error,
        })
    }

    #[cfg(feature = "tokio")]
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

            // id: u32
            let id = crate::util::tokio_read_u32_le(r).await?;

            // error: TrainingFailureReason
            let error: TrainingFailureReason = crate::util::tokio_read_u32_le(r).await?.try_into()?;

            Ok(Self {
                guid,
                id,
                error,
            })
        })
    }

    #[cfg(feature = "async-std")]
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

            // id: u32
            let id = crate::util::astd_read_u32_le(r).await?;

            // error: TrainingFailureReason
            let error: TrainingFailureReason = crate::util::astd_read_u32_le(r).await?.try_into()?;

            Ok(Self {
                guid,
                id,
                error,
            })
        })
    }

}

#[derive(Debug)]
pub enum SMSG_TRAINER_BUY_FAILEDError {
    Io(std::io::Error),
    TrainingFailureReason(TrainingFailureReasonError),
}

impl std::error::Error for SMSG_TRAINER_BUY_FAILEDError {}
impl std::fmt::Display for SMSG_TRAINER_BUY_FAILEDError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::TrainingFailureReason(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_TRAINER_BUY_FAILEDError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<TrainingFailureReasonError> for SMSG_TRAINER_BUY_FAILEDError {
    fn from(e: TrainingFailureReasonError) -> Self {
        Self::TrainingFailureReason(e)
    }
}

