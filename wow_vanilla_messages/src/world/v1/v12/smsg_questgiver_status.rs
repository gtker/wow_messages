use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::QuestGiverStatus;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_QUESTGIVER_STATUS {
    pub guid: Guid,
    pub status: QuestGiverStatus,
}

impl SMSG_QUESTGIVER_STATUS {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 9], std::io::Error> {
        let mut array_w = [0u8; 9];
        let mut w = array_w.as_mut_slice();
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // status: QuestGiverStatus
        w.write_all(&(self.status.as_int() as u32).to_le_bytes())?;

        Ok(array_w)
    }
}

impl ServerMessage for SMSG_QUESTGIVER_STATUS {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // status: QuestGiverStatus
        w.write_all(&(self.status.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0183;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        9
    }

    type Error = SMSG_QUESTGIVER_STATUSError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // status: QuestGiverStatus
        let status: QuestGiverStatus = (crate::util::read_u32_le(r)? as u8).try_into()?;

        Ok(Self {
            guid,
            status,
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

            // status: QuestGiverStatus
            let status: QuestGiverStatus = (crate::util::tokio_read_u32_le(r).await? as u8).try_into()?;

            Ok(Self {
                guid,
                status,
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

            // status: QuestGiverStatus
            let status: QuestGiverStatus = (crate::util::astd_read_u32_le(r).await? as u8).try_into()?;

            Ok(Self {
                guid,
                status,
            })
        })
    }

}

#[derive(Debug)]
pub enum SMSG_QUESTGIVER_STATUSError {
    Io(std::io::Error),
    Enum(crate::errors::EnumError),
}

impl std::error::Error for SMSG_QUESTGIVER_STATUSError {}
impl std::fmt::Display for SMSG_QUESTGIVER_STATUSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::Enum(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_QUESTGIVER_STATUSError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<crate::errors::EnumError> for SMSG_QUESTGIVER_STATUSError {
    fn from(e: crate::errors::EnumError) -> Self {
        Self::Enum(e)
    }
}

