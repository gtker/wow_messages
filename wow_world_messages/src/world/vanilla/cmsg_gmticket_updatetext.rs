use std::io::{Read, Write};

use crate::vanilla::GmTicketType;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gamemaster/cmsg_gmticket_updatetext.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gamemaster/cmsg_gmticket_updatetext.wowm#L1):
/// ```text
/// cmsg CMSG_GMTICKET_UPDATETEXT = 0x0207 {
///     GmTicketType ticket_type;
///     CString message;
/// }
/// ```
pub struct CMSG_GMTICKET_UPDATETEXT {
    /// cmangos does not have this field, vmangos does.
    ///
    pub ticket_type: GmTicketType,
    pub message: String,
}

#[cfg(feature = "print-testcase")]
impl CMSG_GMTICKET_UPDATETEXT {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_GMTICKET_UPDATETEXT {{").unwrap();
        // Members
        writeln!(s, "    ticket_type = {};", self.ticket_type.as_test_case_value()).unwrap();
        writeln!(s, "    message = \"{}\";", self.message).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 519_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "ticket_type", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.message.len() + 1, "message", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"1\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for CMSG_GMTICKET_UPDATETEXT {}
impl crate::Message for CMSG_GMTICKET_UPDATETEXT {
    const OPCODE: u32 = 0x0207;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        CMSG_GMTICKET_UPDATETEXT::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // ticket_type: GmTicketType
        w.write_all(&(self.ticket_type.as_int().to_le_bytes()))?;

        // message: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.message.as_bytes().iter().rev().next(), Some(&0_u8), "String `message` must not be null-terminated.");
        w.write_all(self.message.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(2..=257).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0207, size: body_size });
        }

        // ticket_type: GmTicketType
        let ticket_type = crate::util::read_u8_le(&mut r)?.try_into()?;

        // message: CString
        let message = {
            let message = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(message)?
        };

        Ok(Self {
            ticket_type,
            message,
        })
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_GMTICKET_UPDATETEXT {}

impl CMSG_GMTICKET_UPDATETEXT {
    pub(crate) fn size(&self) -> usize {
        1 // ticket_type: GmTicketType
        + self.message.len() + 1 // message: CString
    }
}

