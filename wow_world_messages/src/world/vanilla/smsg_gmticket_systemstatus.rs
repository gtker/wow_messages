use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gamemaster/smsg_gmticket_systemstatus.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gamemaster/smsg_gmticket_systemstatus.wowm#L3):
/// ```text
/// smsg SMSG_GMTICKET_SYSTEMSTATUS = 0x021B {
///     u32 will_accept_tickets;
/// }
/// ```
pub struct SMSG_GMTICKET_SYSTEMSTATUS {
    /// mangoszero/cmangos/vmangos all only send 1 for true and 0 for false. vmangos: Note: This only disables the ticket UI at client side and is not fully reliable are we sure this is a uint32? Should ask Zor
    ///
    pub will_accept_tickets: u32,
}

impl crate::Message for SMSG_GMTICKET_SYSTEMSTATUS {
    const OPCODE: u32 = 0x021b;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // will_accept_tickets: u32
        w.write_all(&self.will_accept_tickets.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // will_accept_tickets: u32
        let will_accept_tickets = crate::util::read_u32_le(r)?;

        Ok(Self {
            will_accept_tickets,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_GMTICKET_SYSTEMSTATUS {}

