use crate:: {
};
use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_removed_spell.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_removed_spell.wowm#L1):
/// ```text
/// smsg SMSG_REMOVED_SPELL = 0x0203 {
///     u16 spell;
/// }
/// ```
pub struct SMSG_REMOVED_SPELL {
    pub spell: u16,
}

impl crate::Message for SMSG_REMOVED_SPELL {
    const OPCODE: u32 = 0x0203;

    fn size_without_header(&self) -> u32 {
        2
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // spell: u16
        w.write_all(&self.spell.to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 2 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0203, size: body_size as u32 });
        }

        // spell: u16
        let spell = crate::util::read_u16_le(&mut r)?;

        Ok(Self {
            spell,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_REMOVED_SPELL {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_REMOVED_SPELL {}

