use std::convert::{TryFrom, TryInto};
use crate::world::vanilla::GmTicketResponse;
use crate::world::vanilla::ServerMessage;
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

impl crate::Message for SMSG_GMTICKET_DELETETICKET {
    const OPCODE: u32 = 0x0218;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // response: GmTicketResponse
        w.write_all(&(self.response.as_int() as u32).to_le_bytes())?;

        Ok(())
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
impl ServerMessage for SMSG_GMTICKET_DELETETICKET {}

