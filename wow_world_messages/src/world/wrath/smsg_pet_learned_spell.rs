use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_pet_learned_spell.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_pet_learned_spell.wowm#L1):
/// ```text
/// smsg SMSG_PET_LEARNED_SPELL = 0x0499 {
///     u32 spell;
/// }
/// ```
pub struct SMSG_PET_LEARNED_SPELL {
    pub spell: u32,
}

impl crate::Message for SMSG_PET_LEARNED_SPELL {
    const OPCODE: u32 = 0x0499;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0499, size: body_size as u32 });
        }

        // spell: u32
        let spell = crate::util::read_u32_le(r)?;

        Ok(Self {
            spell,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_PET_LEARNED_SPELL {}

