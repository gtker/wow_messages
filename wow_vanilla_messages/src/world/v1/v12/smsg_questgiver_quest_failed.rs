use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{QuestFailedReason, QuestFailedReasonError};
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_QUESTGIVER_QUEST_FAILED {
    pub quest_id: u32,
    pub reason: QuestFailedReason,
}

impl SMSG_QUESTGIVER_QUEST_FAILED {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 8], std::io::Error> {
        let mut array_w = [0u8; 8];
        let mut w = array_w.as_mut_slice();
        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        // reason: QuestFailedReason
        w.write_all(&(self.reason.as_int() as u32).to_le_bytes())?;

        Ok(array_w)
    }
}

impl ServerMessage for SMSG_QUESTGIVER_QUEST_FAILED {
    fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(8);
        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        // reason: QuestFailedReason
        w.write_all(&(self.reason.as_int() as u32).to_le_bytes())?;

        Ok(w)
    }
    const OPCODE: u16 = 0x0192;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        8
    }

    type Error = SMSG_QUESTGIVER_QUEST_FAILEDError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // quest_id: u32
        let quest_id = crate::util::read_u32_le(r)?;

        // reason: QuestFailedReason
        let reason: QuestFailedReason = crate::util::read_u32_le(r)?.try_into()?;

        Ok(Self {
            quest_id,
            reason,
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
            // quest_id: u32
            let quest_id = crate::util::tokio_read_u32_le(r).await?;

            // reason: QuestFailedReason
            let reason: QuestFailedReason = crate::util::tokio_read_u32_le(r).await?.try_into()?;

            Ok(Self {
                quest_id,
                reason,
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
            // quest_id: u32
            let quest_id = crate::util::astd_read_u32_le(r).await?;

            // reason: QuestFailedReason
            let reason: QuestFailedReason = crate::util::astd_read_u32_le(r).await?.try_into()?;

            Ok(Self {
                quest_id,
                reason,
            })
        })
    }

}

#[derive(Debug)]
pub enum SMSG_QUESTGIVER_QUEST_FAILEDError {
    Io(std::io::Error),
    QuestFailedReason(QuestFailedReasonError),
}

impl std::error::Error for SMSG_QUESTGIVER_QUEST_FAILEDError {}
impl std::fmt::Display for SMSG_QUESTGIVER_QUEST_FAILEDError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::QuestFailedReason(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_QUESTGIVER_QUEST_FAILEDError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<QuestFailedReasonError> for SMSG_QUESTGIVER_QUEST_FAILEDError {
    fn from(e: QuestFailedReasonError) -> Self {
        Self::QuestFailedReason(e)
    }
}

