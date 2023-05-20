use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_pet_learned_spell.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_pet_learned_spell.wowm#L1):
/// ```text
/// smsg SMSG_PET_LEARNED_SPELL = 0x0499 {
///     u32 spell;
/// }
/// ```
pub struct SMSG_PET_LEARNED_SPELL {
    pub spell: u32,
}

impl crate::private::Sealed for SMSG_PET_LEARNED_SPELL {}
impl crate::Message for SMSG_PET_LEARNED_SPELL {
    const OPCODE: u32 = 0x0499;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0499, size: body_size });
        }

        // spell: u32
        let spell = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            spell,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_PET_LEARNED_SPELL {}

