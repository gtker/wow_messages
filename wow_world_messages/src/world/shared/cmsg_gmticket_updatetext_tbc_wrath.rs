use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// No TBC/Wrath emulator has a `GmTicketType` field before `message`, but vmangos does.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gamemaster/cmsg_gmticket_updatetext.wowm:10`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gamemaster/cmsg_gmticket_updatetext.wowm#L10):
/// ```text
/// cmsg CMSG_GMTICKET_UPDATETEXT = 0x0207 {
///     CString message;
/// }
/// ```
pub struct CMSG_GMTICKET_UPDATETEXT {
    pub message: String,
}

#[cfg(feature = "print-testcase")]
impl CMSG_GMTICKET_UPDATETEXT {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_GMTICKET_UPDATETEXT {{").unwrap();
        // Members
        writeln!(s, "    message = \"{}\";", self.message).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = (u16::try_from(self.size() + 6).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 519_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"2 3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for CMSG_GMTICKET_UPDATETEXT {}
impl crate::Message for CMSG_GMTICKET_UPDATETEXT {
    const OPCODE: u32 = 0x0207;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // message: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.message.as_bytes().iter().rev().next(), Some(&0_u8), "String `message` must not be null-terminated.");
        w.write_all(self.message.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(1..=256).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0207, size: body_size });
        }

        // message: CString
        let message = {
            let message = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(message)?
        };

        Ok(Self {
            message,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_GMTICKET_UPDATETEXT {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_GMTICKET_UPDATETEXT {}

impl CMSG_GMTICKET_UPDATETEXT {
    pub(crate) fn size(&self) -> usize {
        self.message.len() + 1 // message: CString
    }
}

