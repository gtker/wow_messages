use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/cmsg_search_lfg_join.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/cmsg_search_lfg_join.wowm#L1):
/// ```text
/// cmsg CMSG_SEARCH_LFG_JOIN = 0x035E {
///     u32 dungeon_id;
/// }
/// ```
pub struct CMSG_SEARCH_LFG_JOIN {
    pub dungeon_id: u32,
}

impl crate::private::Sealed for CMSG_SEARCH_LFG_JOIN {}
impl CMSG_SEARCH_LFG_JOIN {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 4 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // dungeon_id: u32
        let dungeon_id = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            dungeon_id,
        })
    }

}

impl crate::Message for CMSG_SEARCH_LFG_JOIN {
    const OPCODE: u32 = 0x035e;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_SEARCH_LFG_JOIN {{").unwrap();
        // Members
        writeln!(s, "    dungeon_id = {};", self.dungeon_id).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 8_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 862_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "dungeon_id", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // dungeon_id: u32
        w.write_all(&self.dungeon_id.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(862, "CMSG_SEARCH_LFG_JOIN", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_SEARCH_LFG_JOIN {}

