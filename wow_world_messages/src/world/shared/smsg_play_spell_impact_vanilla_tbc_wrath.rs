use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_play_spell_impact.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_play_spell_impact.wowm#L3):
/// ```text
/// smsg SMSG_PLAY_SPELL_IMPACT = 0x01F7 {
///     Guid guid;
///     u32 spell_visual_kit;
/// }
/// ```
pub struct SMSG_PLAY_SPELL_IMPACT {
    pub guid: Guid,
    /// mangoszero/mangosone/azerothcore: index from SpellVisualKit.dbc. Used for visual effect on player with 0x016A
    pub spell_visual_kit: u32,
}

#[cfg(feature = "print-testcase")]
impl SMSG_PLAY_SPELL_IMPACT {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_PLAY_SPELL_IMPACT {{").unwrap();
        // Members
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();
        writeln!(s, "    spell_visual_kit = {};", self.spell_visual_kit).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 14_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 503_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "spell_visual_kit", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for SMSG_PLAY_SPELL_IMPACT {}
impl crate::Message for SMSG_PLAY_SPELL_IMPACT {
    const OPCODE: u32 = 0x01f7;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        SMSG_PLAY_SPELL_IMPACT::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // spell_visual_kit: u32
        w.write_all(&self.spell_visual_kit.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01F7, size: body_size });
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        // spell_visual_kit: u32
        let spell_visual_kit = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            guid,
            spell_visual_kit,
        })
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_PLAY_SPELL_IMPACT {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_PLAY_SPELL_IMPACT {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_PLAY_SPELL_IMPACT {}

