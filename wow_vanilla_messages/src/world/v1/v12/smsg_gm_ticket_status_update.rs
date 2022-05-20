use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{GmTicketStatusResponse, GmTicketStatusResponseError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_GM_TICKET_STATUS_UPDATE {
    pub response: GmTicketStatusResponse,
}

impl ServerMessageWrite for SMSG_GM_TICKET_STATUS_UPDATE {}

impl SMSG_GM_TICKET_STATUS_UPDATE {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(8000);
        // response: GmTicketStatusResponse
        w.write_all(&(self.response.as_int() as u32).to_le_bytes())?;

        Ok(w)
    }
}

impl MessageBody for SMSG_GM_TICKET_STATUS_UPDATE {
    const OPCODE: u16 = 0x0328;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = SMSG_GM_TICKET_STATUS_UPDATEError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // response: GmTicketStatusResponse
        let response: GmTicketStatusResponse = crate::util::read_u32_le(r)?.try_into()?;

        Ok(Self {
            response,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let inner = self.as_bytes()?;
        w.write_all(&inner)
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
            // response: GmTicketStatusResponse
            let response: GmTicketStatusResponse = crate::util::tokio_read_u32_le(r).await?.try_into()?;

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
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let inner = self.as_bytes()?;
            w.write_all(&inner).await
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
            // response: GmTicketStatusResponse
            let response: GmTicketStatusResponse = crate::util::astd_read_u32_le(r).await?.try_into()?;

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
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let inner = self.as_bytes()?;
            w.write_all(&inner).await
        })
    }

}

impl SMSG_GM_TICKET_STATUS_UPDATE {
    pub(crate) fn size() -> usize {
        0
        + 4 // response: GmTicketStatusResponse
    }
}

#[derive(Debug)]
pub enum SMSG_GM_TICKET_STATUS_UPDATEError {
    Io(std::io::Error),
    GmTicketStatusResponse(GmTicketStatusResponseError),
}

impl std::error::Error for SMSG_GM_TICKET_STATUS_UPDATEError {}
impl std::fmt::Display for SMSG_GM_TICKET_STATUS_UPDATEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::GmTicketStatusResponse(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_GM_TICKET_STATUS_UPDATEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<GmTicketStatusResponseError> for SMSG_GM_TICKET_STATUS_UPDATEError {
    fn from(e: GmTicketStatusResponseError) -> Self {
        Self::GmTicketStatusResponse(e)
    }
}

