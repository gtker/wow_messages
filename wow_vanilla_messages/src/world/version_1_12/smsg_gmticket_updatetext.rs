use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::GmTicketResponse;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gamemaster/smsg_gmticket_updatetext.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gamemaster/smsg_gmticket_updatetext.wowm#L3):
/// ```text
/// smsg SMSG_GMTICKET_UPDATETEXT = 0x0208 {
///     GmTicketResponse response;
/// }
/// ```
pub struct SMSG_GMTICKET_UPDATETEXT {
    pub response: GmTicketResponse,
}

impl ServerMessage for SMSG_GMTICKET_UPDATETEXT {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // response: GmTicketResponse
        w.write_all(&(self.response.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0208;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        4
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // response: GmTicketResponse
        let response: GmTicketResponse = crate::util::read_u32_le(r)?.try_into()?;

        Ok(Self {
            response,
        })
    }

}

