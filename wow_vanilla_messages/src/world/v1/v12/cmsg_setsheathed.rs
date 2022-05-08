use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{SheathState, SheathStateError};
use crate::{ClientMessageWrite, MessageBody};
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
pub struct CMSG_SETSHEATHED {
    pub sheathed: SheathState,
}

impl ClientMessageWrite for CMSG_SETSHEATHED {}

impl MessageBody for CMSG_SETSHEATHED {
    const OPCODE: u16 = 0x01e0;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = CMSG_SETSHEATHEDError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // sheathed: SheathState
        let sheathed = SheathState::read_u32_le(r)?;

        Ok(Self {
            sheathed,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // sheathed: SheathState
        self.sheathed.write_u32_le(w)?;

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
            // sheathed: SheathState
            let sheathed = SheathState::tokio_read_u32_le(r).await?;

            Ok(Self {
                sheathed,
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
            // sheathed: SheathState
            self.sheathed.tokio_write_u32_le(w).await?;

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
            // sheathed: SheathState
            let sheathed = SheathState::astd_read_u32_le(r).await?;

            Ok(Self {
                sheathed,
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
            // sheathed: SheathState
            self.sheathed.astd_write_u32_le(w).await?;

            Ok(())
        })
    }

}

impl ConstantSized for CMSG_SETSHEATHED {}

impl MaximumPossibleSized for CMSG_SETSHEATHED {
    fn maximum_possible_size() -> usize {
        0
        + 1 // sheathed: SheathState
    }
}

#[derive(Debug)]
pub enum CMSG_SETSHEATHEDError {
    Io(std::io::Error),
    SheathState(SheathStateError),
}

impl std::error::Error for CMSG_SETSHEATHEDError {}
impl std::fmt::Display for CMSG_SETSHEATHEDError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::SheathState(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_SETSHEATHEDError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<SheathStateError> for CMSG_SETSHEATHEDError {
    fn from(e: SheathStateError) -> Self {
        Self::SheathState(e)
    }
}

