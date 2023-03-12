use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_superceded_spell.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_superceded_spell.wowm#L1):
/// ```text
/// smsg SMSG_SUPERCEDED_SPELL = 0x012C {
///     u16 new_spell_id;
///     u16 old_spell_id;
/// }
/// ```
pub struct SMSG_SUPERCEDED_SPELL {
    pub new_spell_id: u16,
    pub old_spell_id: u16,
}

impl crate::Message for SMSG_SUPERCEDED_SPELL {
    const OPCODE: u32 = 0x012c;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // new_spell_id: u16
        w.write_all(&self.new_spell_id.to_le_bytes())?;

        // old_spell_id: u16
        w.write_all(&self.old_spell_id.to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x012C, size: body_size as u32 });
        }

        // new_spell_id: u16
        let new_spell_id = crate::util::read_u16_le(&mut r)?;

        // old_spell_id: u16
        let old_spell_id = crate::util::read_u16_le(&mut r)?;

        Ok(Self {
            new_spell_id,
            old_spell_id,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_SUPERCEDED_SPELL {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_SUPERCEDED_SPELL {}

