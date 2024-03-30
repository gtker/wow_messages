use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/smsg_lfg_offer_continue.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/smsg_lfg_offer_continue.wowm#L1):
/// ```text
/// smsg SMSG_LFG_OFFER_CONTINUE = 0x0293 {
///     u32 dungeon_entry;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_LFG_OFFER_CONTINUE {
    pub dungeon_entry: u32,
}

impl crate::private::Sealed for SMSG_LFG_OFFER_CONTINUE {}
impl SMSG_LFG_OFFER_CONTINUE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 4 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // dungeon_entry: u32
        let dungeon_entry = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            dungeon_entry,
        })
    }

}

impl crate::Message for SMSG_LFG_OFFER_CONTINUE {
    const OPCODE: u32 = 0x0293;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_LFG_OFFER_CONTINUE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_LFG_OFFER_CONTINUE {{").unwrap();
        // Members
        writeln!(s, "    dungeon_entry = {};", self.dungeon_entry).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 6_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 659_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "dungeon_entry", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // dungeon_entry: u32
        w.write_all(&self.dungeon_entry.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(659, "SMSG_LFG_OFFER_CONTINUE", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_LFG_OFFER_CONTINUE {}

