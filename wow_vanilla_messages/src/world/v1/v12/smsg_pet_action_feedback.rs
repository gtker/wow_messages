use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::PetFeedback;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_PET_ACTION_FEEDBACK {
    pub feedback: PetFeedback,
}

impl SMSG_PET_ACTION_FEEDBACK {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 1], std::io::Error> {
        let mut array_w = [0u8; 1];
        let mut w = array_w.as_mut_slice();
        // feedback: PetFeedback
        w.write_all(&(self.feedback.as_int() as u8).to_le_bytes())?;

        Ok(array_w)
    }
}

impl ServerMessage for SMSG_PET_ACTION_FEEDBACK {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // feedback: PetFeedback
        w.write_all(&(self.feedback.as_int() as u8).to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x02c6;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        1
    }

    type Error = SMSG_PET_ACTION_FEEDBACKError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // feedback: PetFeedback
        let feedback: PetFeedback = crate::util::read_u8_le(r)?.try_into()?;

        Ok(Self {
            feedback,
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
            // feedback: PetFeedback
            let feedback: PetFeedback = crate::util::tokio_read_u8_le(r).await?.try_into()?;

            Ok(Self {
                feedback,
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
            // feedback: PetFeedback
            let feedback: PetFeedback = crate::util::astd_read_u8_le(r).await?.try_into()?;

            Ok(Self {
                feedback,
            })
        })
    }

}

#[derive(Debug)]
pub enum SMSG_PET_ACTION_FEEDBACKError {
    Io(std::io::Error),
    Enum(crate::errors::EnumError),
}

impl std::error::Error for SMSG_PET_ACTION_FEEDBACKError {}
impl std::fmt::Display for SMSG_PET_ACTION_FEEDBACKError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::Enum(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_PET_ACTION_FEEDBACKError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<crate::errors::EnumError> for SMSG_PET_ACTION_FEEDBACKError {
    fn from(e: crate::errors::EnumError) -> Self {
        Self::Enum(e)
    }
}

