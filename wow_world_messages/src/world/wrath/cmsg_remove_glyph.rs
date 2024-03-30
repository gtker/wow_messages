use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/cmsg_remove_glyph.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/cmsg_remove_glyph.wowm#L1):
/// ```text
/// cmsg CMSG_REMOVE_GLYPH = 0x048A {
///     u32 glyph;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMSG_REMOVE_GLYPH {
    pub glyph: u32,
}

impl crate::private::Sealed for CMSG_REMOVE_GLYPH {}
impl CMSG_REMOVE_GLYPH {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 4 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // glyph: u32
        let glyph = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            glyph,
        })
    }

}

impl crate::Message for CMSG_REMOVE_GLYPH {
    const OPCODE: u32 = 0x048a;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_REMOVE_GLYPH"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_REMOVE_GLYPH {{").unwrap();
        // Members
        writeln!(s, "    glyph = {};", self.glyph).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 8_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 1162_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "glyph", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // glyph: u32
        w.write_all(&self.glyph.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1162, "CMSG_REMOVE_GLYPH", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_REMOVE_GLYPH {}

