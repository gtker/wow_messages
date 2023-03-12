use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spellinstakilllog.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spellinstakilllog.wowm#L8):
/// ```text
/// smsg SMSG_SPELLINSTAKILLLOG = 0x032F {
///     Guid caster;
///     Guid target;
///     u32 spell;
/// }
/// ```
pub struct SMSG_SPELLINSTAKILLLOG {
    pub caster: Guid,
    pub target: Guid,
    pub spell: u32,
}

impl crate::Message for SMSG_SPELLINSTAKILLLOG {
    const OPCODE: u32 = 0x032f;

    fn size_without_header(&self) -> u32 {
        20
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // caster: Guid
        w.write_all(&self.caster.guid().to_le_bytes())?;

        // target: Guid
        w.write_all(&self.target.guid().to_le_bytes())?;

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 20 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x032F, size: body_size as u32 });
        }

        // caster: Guid
        let caster = Guid::read(&mut r)?;

        // target: Guid
        let target = Guid::read(&mut r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            caster,
            target,
            spell,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_SPELLINSTAKILLLOG {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_SPELLINSTAKILLLOG {}

