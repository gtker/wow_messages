use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{GmTicketStatusResponse, GmTicketStatusResponseError};
use crate::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_GM_TICKET_STATUS_UPDATE {
    pub response: GmTicketStatusResponse,
}

impl WorldServerMessageWrite for SMSG_GM_TICKET_STATUS_UPDATE {
    const OPCODE: u16 = 0x328;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (Self::size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (Self::size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for SMSG_GM_TICKET_STATUS_UPDATE {
    type Error = SMSG_GM_TICKET_STATUS_UPDATEError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // response: GmTicketStatusResponse
        let response = GmTicketStatusResponse::read(r)?;

        Ok(Self {
            response,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // response: GmTicketStatusResponse
        self.response.write(w)?;

        Ok(())
    }
}

impl ConstantSized for SMSG_GM_TICKET_STATUS_UPDATE {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_GM_TICKET_STATUS_UPDATE {
    fn maximum_possible_size() -> usize {
        GmTicketStatusResponse::size() // response: GmTicketStatusResponse
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

