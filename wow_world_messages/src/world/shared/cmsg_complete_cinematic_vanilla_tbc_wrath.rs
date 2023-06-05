use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/cinematic/cmsg_complete_cinematic.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/cinematic/cmsg_complete_cinematic.wowm#L3):
/// ```text
/// cmsg CMSG_COMPLETE_CINEMATIC = 0x00FC {
/// }
/// ```
pub struct CMSG_COMPLETE_CINEMATIC {
}

#[cfg(feature = "print-testcase")]
impl CMSG_COMPLETE_CINEMATIC {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_COMPLETE_CINEMATIC {{").unwrap();
        // Members

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = 6_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 252_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();



        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"1 2 3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for CMSG_COMPLETE_CINEMATIC {}
impl crate::Message for CMSG_COMPLETE_CINEMATIC {
    const OPCODE: u32 = 0x00fc;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x00FC, size: body_size });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_COMPLETE_CINEMATIC {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_COMPLETE_CINEMATIC {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_COMPLETE_CINEMATIC {}

