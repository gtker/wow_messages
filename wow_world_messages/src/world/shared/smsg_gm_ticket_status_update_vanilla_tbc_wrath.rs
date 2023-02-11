use crate::shared::gm_ticket_status_response_vanilla_tbc_wrath::GmTicketStatusResponse;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gamemaster/smsg_gm_ticket_status_update.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gamemaster/smsg_gm_ticket_status_update.wowm#L9):
/// ```text
/// smsg SMSG_GM_TICKET_STATUS_UPDATE = 0x0328 {
///     GmTicketStatusResponse response;
/// }
/// ```
pub struct SMSG_GM_TICKET_STATUS_UPDATE {
    pub response: GmTicketStatusResponse,
}

impl crate::Message for SMSG_GM_TICKET_STATUS_UPDATE {
    const OPCODE: u32 = 0x0328;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // response: GmTicketStatusResponse
        w.write_all(&(self.response.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0328, size: body_size as u32 });
        }

        // response: GmTicketStatusResponse
        let response: GmTicketStatusResponse = crate::util::read_u32_le(r)?.try_into()?;

        Ok(Self {
            response,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_GM_TICKET_STATUS_UPDATE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_GM_TICKET_STATUS_UPDATE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_GM_TICKET_STATUS_UPDATE {}

