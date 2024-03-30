use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/cmsg_tutorial_flag.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/cmsg_tutorial_flag.wowm#L3):
/// ```text
/// cmsg CMSG_TUTORIAL_FLAG = 0x00FE {
///     u32 tutorial_flag;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMSG_TUTORIAL_FLAG {
    /// arcemu indexes into the tutorials by dividing by 32 and modulo 32.
    pub tutorial_flag: u32,
}

impl crate::private::Sealed for CMSG_TUTORIAL_FLAG {}
impl CMSG_TUTORIAL_FLAG {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 4 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // tutorial_flag: u32
        let tutorial_flag = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            tutorial_flag,
        })
    }

}

impl crate::Message for CMSG_TUTORIAL_FLAG {
    const OPCODE: u32 = 0x00fe;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_TUTORIAL_FLAG"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_TUTORIAL_FLAG {{").unwrap();
        // Members
        writeln!(s, "    tutorial_flag = {};", self.tutorial_flag).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 8_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 254_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "tutorial_flag", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // tutorial_flag: u32
        w.write_all(&self.tutorial_flag.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(254, "CMSG_TUTORIAL_FLAG", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_TUTORIAL_FLAG {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_TUTORIAL_FLAG {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_TUTORIAL_FLAG {}

