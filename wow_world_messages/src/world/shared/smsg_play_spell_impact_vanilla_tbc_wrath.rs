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
    ///
    pub spell_visual_kit: u32,
}

impl crate::private::Sealed for SMSG_PLAY_SPELL_IMPACT {}
impl crate::Message for SMSG_PLAY_SPELL_IMPACT {
    const OPCODE: u32 = 0x01f7;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
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
        let guid = Guid::read(&mut r)?;

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

