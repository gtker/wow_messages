use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{FarSightOperation, FarSightOperationError};
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
pub struct CMSG_FAR_SIGHT {
    pub operation: FarSightOperation,
}

impl ClientMessageWrite for CMSG_FAR_SIGHT {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for CMSG_FAR_SIGHT {
    const OPCODE: u16 = 0x027a;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = CMSG_FAR_SIGHTError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // operation: FarSightOperation
        let operation = FarSightOperation::read(r)?;

        Ok(Self {
            operation,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // operation: FarSightOperation
        self.operation.write(w)?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // operation: FarSightOperation
        let operation = FarSightOperation::tokio_read(r).await?;

        Ok(Self {
            operation,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // operation: FarSightOperation
        self.operation.tokio_write(w).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // operation: FarSightOperation
        let operation = FarSightOperation::astd_read(r).await?;

        Ok(Self {
            operation,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // operation: FarSightOperation
        self.operation.astd_write(w).await?;

        Ok(())
    }

}

impl ConstantSized for CMSG_FAR_SIGHT {}

impl MaximumPossibleSized for CMSG_FAR_SIGHT {
    fn maximum_possible_size() -> usize {
        FarSightOperation::size() // operation: FarSightOperation
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

