use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{GmTicketResponse, GmTicketResponseError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_GMTICKET_UPDATETEXT {
    pub response: GmTicketResponse,
}

impl ServerMessageWrite for SMSG_GMTICKET_UPDATETEXT {}

impl MessageBody for SMSG_GMTICKET_UPDATETEXT {
    const OPCODE: u16 = 0x0208;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = SMSG_GMTICKET_UPDATETEXTError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // response: GmTicketResponse
        let response: GmTicketResponse = crate::util::read_u32_le(r)?.try_into()?;

        Ok(Self {
            response,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // response: GmTicketResponse
        w.write_all(&(self.response.as_int() as u32).to_le_bytes())?;

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
            // response: GmTicketResponse
            let response: GmTicketResponse = crate::util::tokio_read_u32_le(r).await?.try_into()?;

            Ok(Self {
                response,
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
            // response: GmTicketResponse
            w.write_all(&(self.response.as_int() as u32).to_le_bytes()).await?;

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
            // response: GmTicketResponse
            let response: GmTicketResponse = crate::util::astd_read_u32_le(r).await?.try_into()?;

            Ok(Self {
                response,
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
            // response: GmTicketResponse
            w.write_all(&(self.response.as_int() as u32).to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl SMSG_GMTICKET_UPDATETEXT {
    pub(crate) fn size() -> usize {
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

