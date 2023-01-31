use std::convert::{TryFrom, TryInto};
use crate::shared::gm_ticket_queue_status_vanilla_tbc_wrath::GmTicketQueueStatus;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gamemaster/smsg_gmticket_systemstatus.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gamemaster/smsg_gmticket_systemstatus.wowm#L8):
/// ```text
/// smsg SMSG_GMTICKET_SYSTEMSTATUS = 0x021B {
///     GmTicketQueueStatus will_accept_tickets;
/// }
/// ```
pub struct SMSG_GMTICKET_SYSTEMSTATUS {
    /// vmangos: This only disables the ticket UI at client side and is not fully reliable are we sure this is a uint32? Should ask Zor
    ///
    pub will_accept_tickets: GmTicketQueueStatus,
}

impl crate::Message for SMSG_GMTICKET_SYSTEMSTATUS {
    const OPCODE: u32 = 0x021b;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // will_accept_tickets: GmTicketQueueStatus
        w.write_all(&(self.will_accept_tickets.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x021B, size: body_size as u32 });
        }

        // will_accept_tickets: GmTicketQueueStatus
        let will_accept_tickets: GmTicketQueueStatus = crate::util::read_u32_le(r)?.try_into()?;

        Ok(Self {
            will_accept_tickets,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_GMTICKET_SYSTEMSTATUS {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_GMTICKET_SYSTEMSTATUS {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_GMTICKET_SYSTEMSTATUS {}

