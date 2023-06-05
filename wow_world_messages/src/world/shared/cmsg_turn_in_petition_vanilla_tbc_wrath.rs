use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/cmsg_turn_in_petition.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/cmsg_turn_in_petition.wowm#L3):
/// ```text
/// cmsg CMSG_TURN_IN_PETITION = 0x01C4 {
///     Guid petition;
/// }
/// ```
pub struct CMSG_TURN_IN_PETITION {
    pub petition: Guid,
}

#[cfg(feature = "print-testcase")]
impl CMSG_TURN_IN_PETITION {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_TURN_IN_PETITION {{").unwrap();
        // Members
        writeln!(s, "    petition = {};", self.petition.guid()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 12_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 452_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "petition", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"1 2 3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for CMSG_TURN_IN_PETITION {}
impl crate::Message for CMSG_TURN_IN_PETITION {
    const OPCODE: u32 = 0x01c4;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        CMSG_TURN_IN_PETITION::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // petition: Guid
        w.write_all(&self.petition.guid().to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01C4, size: body_size });
        }

        // petition: Guid
        let petition = crate::util::read_guid(&mut r)?;

        Ok(Self {
            petition,
        })
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_TURN_IN_PETITION {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_TURN_IN_PETITION {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_TURN_IN_PETITION {}

