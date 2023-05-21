use std::io::{Read, Write};

use wow_world_base::shared::gm_ticket_response_vanilla_tbc_wrath::GmTicketResponse;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gamemaster/smsg_gmticket_create.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gamemaster/smsg_gmticket_create.wowm#L3):
/// ```text
/// smsg SMSG_GMTICKET_CREATE = 0x0206 {
///     GmTicketResponse response;
/// }
/// ```
pub struct SMSG_GMTICKET_CREATE {
    pub response: GmTicketResponse,
}

impl crate::private::Sealed for SMSG_GMTICKET_CREATE {}
impl SMSG_GMTICKET_CREATE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 4 {
            return Err(crate::errors::ParseErrorKind::InvalidSize { opcode: 0x0206, size: body_size });
        }

        // response: GmTicketResponse
        let response = crate::util::read_u32_le(&mut r)?.try_into()?;

        Ok(Self {
            response,
        })
    }

}

impl crate::Message for SMSG_GMTICKET_CREATE {
    const OPCODE: u32 = 0x0206;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_GMTICKET_CREATE {{").unwrap();
        // Members
        writeln!(s, "    response = {};", self.response.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 6_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 518_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "response", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // response: GmTicketResponse
        w.write_all(&(self.response.as_int().to_le_bytes()))?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_GMTICKET_CREATE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_GMTICKET_CREATE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_GMTICKET_CREATE {}

