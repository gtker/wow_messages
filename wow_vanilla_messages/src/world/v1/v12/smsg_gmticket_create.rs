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
pub struct SMSG_GMTICKET_CREATE {
    pub response: GmTicketResponse,
}

impl ServerMessageWrite for SMSG_GMTICKET_CREATE {}

impl MessageBody for SMSG_GMTICKET_CREATE {
    const OPCODE: u16 = 0x0206;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = SMSG_GMTICKET_CREATEError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // response: GmTicketResponse
        let response = GmTicketResponse::read(r)?;

        Ok(Self {
            response,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // response: GmTicketResponse
        self.response.write(w)?;

        Ok(())
    }
}

impl ConstantSized for SMSG_GMTICKET_CREATE {}

impl MaximumPossibleSized for SMSG_GMTICKET_CREATE {
    fn maximum_possible_size() -> usize {
        GmTicketResponse::size() // response: GmTicketResponse
    }
}

#[derive(Debug)]
pub enum SMSG_GMTICKET_CREATEError {
    Io(std::io::Error),
    GmTicketResponse(GmTicketResponseError),
}

impl std::error::Error for SMSG_GMTICKET_CREATEError {}
impl std::fmt::Display for SMSG_GMTICKET_CREATEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::GmTicketResponse(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_GMTICKET_CREATEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<GmTicketResponseError> for SMSG_GMTICKET_CREATEError {
    fn from(e: GmTicketResponseError) -> Self {
        Self::GmTicketResponse(e)
    }
}

