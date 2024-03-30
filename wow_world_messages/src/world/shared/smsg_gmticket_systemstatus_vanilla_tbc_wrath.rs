use std::io::{Read, Write};

use wow_world_base::shared::gm_ticket_queue_status_vanilla_tbc_wrath::GmTicketQueueStatus;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gamemaster/smsg_gmticket_systemstatus.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gamemaster/smsg_gmticket_systemstatus.wowm#L8):
/// ```text
/// smsg SMSG_GMTICKET_SYSTEMSTATUS = 0x021B {
///     GmTicketQueueStatus will_accept_tickets;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_GMTICKET_SYSTEMSTATUS {
    /// vmangos: This only disables the ticket UI at client side and is not fully reliable are we sure this is a uint32? Should ask Zor
    pub will_accept_tickets: GmTicketQueueStatus,
}

impl crate::private::Sealed for SMSG_GMTICKET_SYSTEMSTATUS {}
impl SMSG_GMTICKET_SYSTEMSTATUS {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 4 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // will_accept_tickets: GmTicketQueueStatus
        let will_accept_tickets = crate::util::read_u32_le(&mut r)?.try_into()?;

        Ok(Self {
            will_accept_tickets,
        })
    }

}

impl crate::Message for SMSG_GMTICKET_SYSTEMSTATUS {
    const OPCODE: u32 = 0x021b;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_GMTICKET_SYSTEMSTATUS"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_GMTICKET_SYSTEMSTATUS {{").unwrap();
        // Members
        writeln!(s, "    will_accept_tickets = {};", self.will_accept_tickets.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 6_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 539_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "will_accept_tickets", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // will_accept_tickets: GmTicketQueueStatus
        w.write_all(&(self.will_accept_tickets.as_int().to_le_bytes()))?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(539, "SMSG_GMTICKET_SYSTEMSTATUS", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_GMTICKET_SYSTEMSTATUS {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_GMTICKET_SYSTEMSTATUS {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_GMTICKET_SYSTEMSTATUS {}

