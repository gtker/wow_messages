use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{PetitionResult, PetitionResultError};
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_PETITION_SIGN_RESULTS {
    pub petition_guid: Guid,
    pub owner_guid: Guid,
    pub result: PetitionResult,
}

impl SMSG_PETITION_SIGN_RESULTS {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 20], std::io::Error> {
        let mut array_w = [0u8; 20];
        let mut w = array_w.as_mut_slice();
        // petition_guid: Guid
        w.write_all(&self.petition_guid.guid().to_le_bytes())?;

        // owner_guid: Guid
        w.write_all(&self.owner_guid.guid().to_le_bytes())?;

        // result: PetitionResult
        w.write_all(&(self.result.as_int() as u32).to_le_bytes())?;

        Ok(array_w)
    }
}

impl ServerMessage for SMSG_PETITION_SIGN_RESULTS {
    fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(20);
        // petition_guid: Guid
        w.write_all(&self.petition_guid.guid().to_le_bytes())?;

        // owner_guid: Guid
        w.write_all(&self.owner_guid.guid().to_le_bytes())?;

        // result: PetitionResult
        w.write_all(&(self.result.as_int() as u32).to_le_bytes())?;

        Ok(w)
    }
    const OPCODE: u16 = 0x01c1;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        20
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

