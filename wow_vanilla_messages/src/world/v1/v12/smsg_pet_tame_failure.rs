use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{PetTameFailureReason, PetTameFailureReasonError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_PET_TAME_FAILURE {
    pub reason: PetTameFailureReason,
}

impl ServerMessageWrite for SMSG_PET_TAME_FAILURE {}

impl MessageBody for SMSG_PET_TAME_FAILURE {
    const OPCODE: u16 = 0x0173;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = SMSG_PET_TAME_FAILUREError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // reason: PetTameFailureReason
        let reason: PetTameFailureReason = crate::util::read_u8_le(r)?.try_into()?;

        Ok(Self {
            reason,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // reason: PetTameFailureReason
        w.write_all(&(self.reason.as_int() as u8).to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
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
            // reason: PetTameFailureReason
            let reason: PetTameFailureReason = crate::util::tokio_read_u8_le(r).await?.try_into()?;

            Ok(Self {
                reason,
            })
        })
    }

    #[cfg(feature = "async_tokio")]
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
            // reason: PetTameFailureReason
            w.write_all(&(self.reason.as_int() as u8).to_le_bytes()).await?;

            Ok(())
        })
    }

    #[cfg(feature = "async_std")]
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
            // reason: PetTameFailureReason
            let reason: PetTameFailureReason = crate::util::astd_read_u8_le(r).await?.try_into()?;

            Ok(Self {
                reason,
            })
        })
    }

    #[cfg(feature = "async_std")]
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
            // reason: PetTameFailureReason
            w.write_all(&(self.reason.as_int() as u8).to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl ConstantSized for SMSG_PET_TAME_FAILURE {}

impl MaximumPossibleSized for SMSG_PET_TAME_FAILURE {
    fn maximum_possible_size() -> usize {
        0
        + 1 // reason: PetTameFailureReason
    }
}

#[derive(Debug)]
pub enum SMSG_PET_TAME_FAILUREError {
    Io(std::io::Error),
    PetTameFailureReason(PetTameFailureReasonError),
}

impl std::error::Error for SMSG_PET_TAME_FAILUREError {}
impl std::fmt::Display for SMSG_PET_TAME_FAILUREError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::PetTameFailureReason(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_PET_TAME_FAILUREError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<PetTameFailureReasonError> for SMSG_PET_TAME_FAILUREError {
    fn from(e: PetTameFailureReasonError) -> Self {
        Self::PetTameFailureReason(e)
    }
}

