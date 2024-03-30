use std::io::{Read, Write};

use crate::vanilla::Map;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/cmsg_battlefield_join.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/cmsg_battlefield_join.wowm#L3):
/// ```text
/// cmsg CMSG_BATTLEFIELD_JOIN = 0x023E {
///     Map map;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMSG_BATTLEFIELD_JOIN {
    pub map: Map,
}

impl crate::private::Sealed for CMSG_BATTLEFIELD_JOIN {}
impl CMSG_BATTLEFIELD_JOIN {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 4 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // map: Map
        let map = crate::util::read_u32_le(&mut r)?.try_into()?;

        Ok(Self {
            map,
        })
    }

}

impl crate::Message for CMSG_BATTLEFIELD_JOIN {
    const OPCODE: u32 = 0x023e;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_BATTLEFIELD_JOIN"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_BATTLEFIELD_JOIN {{").unwrap();
        // Members
        writeln!(s, "    map = {};", self.map.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 8_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 574_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "map", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1.12".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // map: Map
        w.write_all(&(self.map.as_int().to_le_bytes()))?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(574, "CMSG_BATTLEFIELD_JOIN", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_BATTLEFIELD_JOIN {}

