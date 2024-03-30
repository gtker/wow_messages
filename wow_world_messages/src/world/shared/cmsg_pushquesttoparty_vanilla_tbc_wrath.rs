use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/cmsg_pushquesttoparty.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/cmsg_pushquesttoparty.wowm#L3):
/// ```text
/// cmsg CMSG_PUSHQUESTTOPARTY = 0x019D {
///     u32 quest_id;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMSG_PUSHQUESTTOPARTY {
    pub quest_id: u32,
}

impl crate::private::Sealed for CMSG_PUSHQUESTTOPARTY {}
impl CMSG_PUSHQUESTTOPARTY {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 4 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // quest_id: u32
        let quest_id = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            quest_id,
        })
    }

}

impl crate::Message for CMSG_PUSHQUESTTOPARTY {
    const OPCODE: u32 = 0x019d;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_PUSHQUESTTOPARTY"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_PUSHQUESTTOPARTY {{").unwrap();
        // Members
        writeln!(s, "    quest_id = {};", self.quest_id).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 8_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 413_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "quest_id", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(413, "CMSG_PUSHQUESTTOPARTY", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_PUSHQUESTTOPARTY {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_PUSHQUESTTOPARTY {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_PUSHQUESTTOPARTY {}

