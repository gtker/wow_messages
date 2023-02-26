use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spellinstakilllog.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spellinstakilllog.wowm#L1):
/// ```text
/// smsg SMSG_SPELLINSTAKILLLOG = 0x032F {
///     Guid target;
///     u32 spell;
/// }
/// ```
pub struct SMSG_SPELLINSTAKILLLOG {
    pub target: Guid,
    pub spell: u32,
}

impl crate::Message for SMSG_SPELLINSTAKILLLOG {
    const OPCODE: u32 = 0x032f;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // target: Guid
        w.write_all(&self.target.guid().to_le_bytes())?;

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x032F, size: body_size as u32 });
        }

        // target: Guid
        let target = Guid::read(r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(r)?;

        Ok(Self {
            target,
            spell,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_SPELLINSTAKILLLOG {}

