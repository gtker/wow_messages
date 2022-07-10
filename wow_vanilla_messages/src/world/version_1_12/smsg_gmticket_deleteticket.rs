use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::GmTicketResponse;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gamemaster/smsg_gmticket_deleteticket.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gamemaster/smsg_gmticket_deleteticket.wowm#L3):
/// ```text
/// smsg SMSG_GMTICKET_DELETETICKET = 0x0218 {
///     GmTicketResponse response;
/// }
/// ```
pub struct SMSG_GMTICKET_DELETETICKET {
    pub response: GmTicketResponse,
}

impl ServerMessage for SMSG_GMTICKET_DELETETICKET {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // response: GmTicketResponse
        w.write_all(&(self.response.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0218;

    fn server_size(&self) -> u16 {
        8
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // response: GmTicketResponse
        let response: GmTicketResponse = crate::util::read_u32_le(r)?.try_into()?;

        Ok(Self {
            response,
        })
    }

}

