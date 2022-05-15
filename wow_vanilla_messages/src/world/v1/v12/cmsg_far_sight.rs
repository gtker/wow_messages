use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{FarSightOperation, FarSightOperationError};
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMSG_FAR_SIGHT {
    pub operation: FarSightOperation,
}

impl ClientMessageWrite for CMSG_FAR_SIGHT {}

impl MessageBody for CMSG_FAR_SIGHT {
    const OPCODE: u16 = 0x027a;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = CMSG_FAR_SIGHTError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // operation: FarSightOperation
        let operation: FarSightOperation = crate::util::read_u8_le(r)?.try_into()?;

        Ok(Self {
            operation,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // operation: FarSightOperation
        crate::util::write_u8_le(w, self.operation.as_int() as u8)?;

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
            // operation: FarSightOperation
            let operation: FarSightOperation = crate::util::tokio_read_u8_le(r).await?.try_into()?;

            Ok(Self {
                operation,
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
            // operation: FarSightOperation
            crate::util::tokio_write_u8_le(w, self.operation.as_int() as u8).await?;

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
            // operation: FarSightOperation
            let operation: FarSightOperation = crate::util::astd_read_u8_le(r).await?.try_into()?;

            Ok(Self {
                operation,
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
            // operation: FarSightOperation
            crate::util::astd_write_u8_le(w, self.operation.as_int() as u8).await?;

            Ok(())
        })
    }

}

impl ConstantSized for CMSG_FAR_SIGHT {}

impl MaximumPossibleSized for CMSG_FAR_SIGHT {
    fn maximum_possible_size() -> usize {
        0
        + 1 // operation: FarSightOperation
    }
}

#[derive(Debug)]
pub enum CMSG_FAR_SIGHTError {
    Io(std::io::Error),
    FarSightOperation(FarSightOperationError),
}

impl std::error::Error for CMSG_FAR_SIGHTError {}
impl std::fmt::Display for CMSG_FAR_SIGHTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::FarSightOperation(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_FAR_SIGHTError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<FarSightOperationError> for CMSG_FAR_SIGHTError {
    fn from(e: FarSightOperationError) -> Self {
        Self::FarSightOperation(e)
    }
}

