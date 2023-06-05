use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::SpellSchool;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spelldamageshield.wowm:20`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spelldamageshield.wowm#L20):
/// ```text
/// smsg SMSG_SPELLDAMAGESHIELD = 0x024F {
///     Guid victim;
///     Guid caster;
///     u32 spell;
///     u32 damage;
///     u32 overkill;
///     (u32)SpellSchool school;
/// }
/// ```
pub struct SMSG_SPELLDAMAGESHIELD {
    pub victim: Guid,
    pub caster: Guid,
    pub spell: u32,
    pub damage: u32,
    pub overkill: u32,
    pub school: SpellSchool,
}

impl crate::private::Sealed for SMSG_SPELLDAMAGESHIELD {}
impl crate::Message for SMSG_SPELLDAMAGESHIELD {
    const OPCODE: u32 = 0x024f;

    fn size_without_header(&self) -> u32 {
        32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // victim: Guid
        w.write_all(&self.victim.guid().to_le_bytes())?;

        // caster: Guid
        w.write_all(&self.caster.guid().to_le_bytes())?;

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // damage: u32
        w.write_all(&self.damage.to_le_bytes())?;

        // overkill: u32
        w.write_all(&self.overkill.to_le_bytes())?;

        // school: SpellSchool
        w.write_all(&u32::from(self.school.as_int()).to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 32 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x024F, size: body_size });
        }

        // victim: Guid
        let victim = crate::util::read_guid(&mut r)?;

        // caster: Guid
        let caster = crate::util::read_guid(&mut r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(&mut r)?;

        // damage: u32
        let damage = crate::util::read_u32_le(&mut r)?;

        // overkill: u32
        let overkill = crate::util::read_u32_le(&mut r)?;

        // school: SpellSchool
        let school = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

        Ok(Self {
            victim,
            caster,
            spell,
            damage,
            overkill,
            school,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_SPELLDAMAGESHIELD {}

