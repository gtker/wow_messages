use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{PetitionResult, PetitionResultError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_PETITION_SIGN_RESULTS {
    pub petition_guid: Guid,
    pub owner_guid: Guid,
    pub result: PetitionResult,
}

impl ServerMessageWrite for SMSG_PETITION_SIGN_RESULTS {}

impl MessageBody for SMSG_PETITION_SIGN_RESULTS {
    const OPCODE: u16 = 0x01c1;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = SMSG_PETITION_SIGN_RESULTSError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // petition_guid: Guid
        let petition_guid = Guid::read(r)?;

        // owner_guid: Guid
        let owner_guid = Guid::read(r)?;

        // result: PetitionResult
        let result: PetitionResult = crate::util::read_u32_le(r)?.try_into()?;

        Ok(Self {
            petition_guid,
            owner_guid,
            result,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // petition_guid: Guid
        self.petition_guid.write(w)?;

        // owner_guid: Guid
        self.owner_guid.write(w)?;

        // result: PetitionResult
        w.write_all(&(self.result.as_int() as u32).to_le_bytes())?;

        Ok(())
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
            // petition_guid: Guid
            let petition_guid = Guid::tokio_read(r).await?;

            // owner_guid: Guid
            let owner_guid = Guid::tokio_read(r).await?;

            // result: PetitionResult
            let result: PetitionResult = crate::util::tokio_read_u32_le(r).await?.try_into()?;

            Ok(Self {
                petition_guid,
                owner_guid,
                result,
            })
        })
    }

    #[cfg(feature = "tokio")]
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
            // petition_guid: Guid
            self.petition_guid.tokio_write(w).await?;

            // owner_guid: Guid
            self.owner_guid.tokio_write(w).await?;

            // result: PetitionResult
            w.write_all(&(self.result.as_int() as u32).to_le_bytes()).await?;

            Ok(())
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
            // petition_guid: Guid
            let petition_guid = Guid::astd_read(r).await?;

            // owner_guid: Guid
            let owner_guid = Guid::astd_read(r).await?;

            // result: PetitionResult
            let result: PetitionResult = crate::util::astd_read_u32_le(r).await?.try_into()?;

            Ok(Self {
                petition_guid,
                owner_guid,
                result,
            })
        })
    }

    #[cfg(feature = "async-std")]
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
            // petition_guid: Guid
            self.petition_guid.astd_write(w).await?;

            // owner_guid: Guid
            self.owner_guid.astd_write(w).await?;

            // result: PetitionResult
            w.write_all(&(self.result.as_int() as u32).to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl SMSG_PETITION_SIGN_RESULTS {
    pub(crate) fn size() -> usize {
        0
        + 8 // petition_guid: Guid
        + 8 // owner_guid: Guid
        + 4 // result: PetitionResult
    }
}

#[derive(Debug)]
pub enum SMSG_PETITION_SIGN_RESULTSError {
    Io(std::io::Error),
    PetitionResult(PetitionResultError),
}

impl std::error::Error for SMSG_PETITION_SIGN_RESULTSError {}
impl std::fmt::Display for SMSG_PETITION_SIGN_RESULTSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::PetitionResult(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_PETITION_SIGN_RESULTSError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<PetitionResultError> for SMSG_PETITION_SIGN_RESULTSError {
    fn from(e: PetitionResultError) -> Self {
        Self::PetitionResult(e)
    }
}

