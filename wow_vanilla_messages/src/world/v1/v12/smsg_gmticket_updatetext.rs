use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{GmTicketResponse, GmTicketResponseError};
use crate::{ServerMessageWrite, MessageBody};
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
pub struct SMSG_GMTICKET_UPDATETEXT {
    pub response: GmTicketResponse,
}

impl ServerMessageWrite for SMSG_GMTICKET_UPDATETEXT {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_GMTICKET_UPDATETEXT {
    const OPCODE: u16 = 0x0208;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = SMSG_GMTICKET_UPDATETEXTError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // response: GmTicketResponse
        let response = GmTicketResponse::read(r)?;

        Ok(Self {
            response,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // response: GmTicketResponse
        self.response.write(w)?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // response: GmTicketResponse
        let response = GmTicketResponse::tokio_read(r).await?;

        Ok(Self {
            response,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // response: GmTicketResponse
        self.response.tokio_write(w).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // response: GmTicketResponse
        let response = GmTicketResponse::astd_read(r).await?;

        Ok(Self {
            response,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // response: GmTicketResponse
        self.response.astd_write(w).await?;

        Ok(())
    }

}

impl ConstantSized for SMSG_GMTICKET_UPDATETEXT {}

impl MaximumPossibleSized for SMSG_GMTICKET_UPDATETEXT {
    fn maximum_possible_size() -> usize {
        0
        + 4 // response: GmTicketResponse
    }
}

#[derive(Debug)]
pub enum SMSG_GMTICKET_UPDATETEXTError {
    Io(std::io::Error),
    GmTicketResponse(GmTicketResponseError),
}

impl std::error::Error for SMSG_GMTICKET_UPDATETEXTError {}
impl std::fmt::Display for SMSG_GMTICKET_UPDATETEXTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::GmTicketResponse(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_GMTICKET_UPDATETEXTError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<GmTicketResponseError> for SMSG_GMTICKET_UPDATETEXTError {
    fn from(e: GmTicketResponseError) -> Self {
        Self::GmTicketResponse(e)
    }
}

