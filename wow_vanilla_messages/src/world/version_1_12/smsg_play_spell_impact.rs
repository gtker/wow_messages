use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_play_spell_impact.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_play_spell_impact.wowm#L3):
/// ```text
/// smsg SMSG_PLAY_SPELL_IMPACT = 0x01F7 {
///     Guid guid;
///     u32 spell_visual_kit;
/// }
/// ```
pub struct SMSG_PLAY_SPELL_IMPACT {
    pub guid: Guid,
    /// # Comment
    ///
    /// mangoszero: index from SpellVisualKit.dbc. Used for visual effect on player with 0x016A
    pub spell_visual_kit: u32,
}

impl ServerMessage for SMSG_PLAY_SPELL_IMPACT {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // spell_visual_kit: u32
        w.write_all(&self.spell_visual_kit.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x01f7;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        12
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // spell_visual_kit: u32
        let spell_visual_kit = crate::util::read_u32_le(r)?;

        Ok(Self {
            guid,
            spell_visual_kit,
        })
    }

}

