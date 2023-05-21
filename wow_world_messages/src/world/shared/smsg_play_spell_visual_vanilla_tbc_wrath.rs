use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_play_spell_visual.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_play_spell_visual.wowm#L3):
/// ```text
/// smsg SMSG_PLAY_SPELL_VISUAL = 0x01F3 {
///     Guid guid;
///     u32 spell_art_kit;
/// }
/// ```
pub struct SMSG_PLAY_SPELL_VISUAL {
    pub guid: Guid,
    /// mangoszero/mangosone: index from SpellVisualKit.dbc. Set to 0xB3 when buying spells.
    pub spell_art_kit: u32,
}

impl crate::private::Sealed for SMSG_PLAY_SPELL_VISUAL {}
impl SMSG_PLAY_SPELL_VISUAL {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 12 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        // spell_art_kit: u32
        let spell_art_kit = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            guid,
            spell_art_kit,
        })
    }

}

impl crate::Message for SMSG_PLAY_SPELL_VISUAL {
    const OPCODE: u32 = 0x01f3;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_PLAY_SPELL_VISUAL {{").unwrap();
        // Members
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();
        writeln!(s, "    spell_art_kit = {};", self.spell_art_kit).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 14_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 499_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "spell_art_kit", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // spell_art_kit: u32
        w.write_all(&self.spell_art_kit.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(499, "SMSG_PLAY_SPELL_VISUAL", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_PLAY_SPELL_VISUAL {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_PLAY_SPELL_VISUAL {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_PLAY_SPELL_VISUAL {}

