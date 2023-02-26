use crate::Guid;
use crate::wrath::SpellCastResult;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spell_failure.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spell_failure.wowm#L9):
/// ```text
/// smsg SMSG_SPELL_FAILURE = 0x0133 {
///     Guid guid;
///     u8 extra_casts;
///     u32 spell;
///     SpellCastResult result;
/// }
/// ```
pub struct SMSG_SPELL_FAILURE {
    pub guid: Guid,
    pub extra_casts: u8,
    pub spell: u32,
    pub result: SpellCastResult,
}

impl crate::Message for SMSG_SPELL_FAILURE {
    const OPCODE: u32 = 0x0133;

    fn size_without_header(&self) -> u32 {
        14
    }

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // extra_casts: u8
        w.write_all(&self.extra_casts.to_le_bytes())?;

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // result: SpellCastResult
        w.write_all(&(self.result.as_int() as u8).to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 14 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0133, size: body_size as u32 });
        }

        // guid: Guid
        let guid = Guid::read(r)?;

        // extra_casts: u8
        let extra_casts = crate::util::read_u8_le(r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(r)?;

        // result: SpellCastResult
        let result: SpellCastResult = crate::util::read_u8_le(r)?.try_into()?;

        Ok(Self {
            guid,
            extra_casts,
            spell,
            result,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_SPELL_FAILURE {}

