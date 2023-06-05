use std::io::{Read, Write};

use wow_world_base::shared::gm_ticket_status_response_vanilla_tbc_wrath::GmTicketStatusResponse;

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

#[cfg(feature = "print-testcase")]
impl SMSG_GM_TICKET_STATUS_UPDATE {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_GM_TICKET_STATUS_UPDATE {{").unwrap();
        // Members
        writeln!(s, "    response = {};", self.response.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 6_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 808_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "response", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"1 2 3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for SMSG_GM_TICKET_STATUS_UPDATE {}
impl crate::Message for SMSG_GM_TICKET_STATUS_UPDATE {
    const OPCODE: u32 = 0x0328;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        SMSG_GM_TICKET_STATUS_UPDATE::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // response: GmTicketStatusResponse
        w.write_all(&(self.response.as_int().to_le_bytes()))?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0328, size: body_size });
        }

        // response: GmTicketStatusResponse
        let response = crate::util::read_u32_le(&mut r)?.try_into()?;

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

