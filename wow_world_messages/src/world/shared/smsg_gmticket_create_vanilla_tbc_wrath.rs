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

#[cfg(feature = "print-testcase")]
impl SMSG_GMTICKET_CREATE {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_GMTICKET_CREATE {{").unwrap();
        // Members
        writeln!(s, "    response = {};", self.response.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = 8_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 518_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "response");
        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"1 2 3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for SMSG_GMTICKET_CREATE {}
impl crate::Message for SMSG_GMTICKET_CREATE {
    const OPCODE: u32 = 0x0206;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // response: GmTicketResponse
        w.write_all(&(self.response.as_int().to_le_bytes()))?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0206, size: body_size });
        }

        // response: GmTicketResponse
        let response = crate::util::read_u32_le(&mut r)?.try_into()?;

        Ok(Self {
            response,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_GMTICKET_CREATE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_GMTICKET_CREATE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_GMTICKET_CREATE {}

