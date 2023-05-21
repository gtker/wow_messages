use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/cmsg_played_time.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/cmsg_played_time.wowm#L1):
/// ```text
/// cmsg CMSG_PLAYED_TIME = 0x01CC {
/// }
/// ```
pub struct CMSG_PLAYED_TIME {
}

impl crate::private::Sealed for CMSG_PLAYED_TIME {}
impl CMSG_PLAYED_TIME {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 0 {
            return Err(crate::errors::ParseErrorKind::InvalidSize { opcode: 0x01CC, size: body_size });
        }

        Ok(Self {
        })
    }

}

impl crate::Message for CMSG_PLAYED_TIME {
    const OPCODE: u32 = 0x01cc;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_PLAYED_TIME {{").unwrap();
        // Members

        writeln!(s, "}} [").unwrap();

        let [a, b] = 4_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 460_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();



        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1.12 2".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_PLAYED_TIME {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_PLAYED_TIME {}

