use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/cmsg_contact_list.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/cmsg_contact_list.wowm#L1):
/// ```text
/// cmsg CMSG_CONTACT_LIST = 0x0066 {
///     u32 flags;
/// }
/// ```
pub struct CMSG_CONTACT_LIST {
    /// Sent back in [`SMSG_CONTACT_LIST`](crate::tbc::SMSG_CONTACT_LIST).
    ///
    pub flags: u32,
}

#[cfg(feature = "print-testcase")]
impl CMSG_CONTACT_LIST {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_CONTACT_LIST {{").unwrap();
        // Members
        writeln!(s, "    flags = {};", self.flags).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 8_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 102_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "flags", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"2.4.3 3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for CMSG_CONTACT_LIST {}
impl crate::Message for CMSG_CONTACT_LIST {
    const OPCODE: u32 = 0x0066;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        CMSG_CONTACT_LIST::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // flags: u32
        w.write_all(&self.flags.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0066, size: body_size });
        }

        // flags: u32
        let flags = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            flags,
        })
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_CONTACT_LIST {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_CONTACT_LIST {}

