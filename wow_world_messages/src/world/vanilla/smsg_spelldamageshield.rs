use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::vanilla::SpellSchool;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spelldamageshield.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spelldamageshield.wowm#L3):
/// ```text
/// smsg SMSG_SPELLDAMAGESHIELD = 0x024F {
///     Guid victim_guid;
///     Guid caster_guid;
///     u32 damage;
///     SpellSchool school;
/// }
/// ```
pub struct SMSG_SPELLDAMAGESHIELD {
    pub victim_guid: Guid,
    pub caster_guid: Guid,
    pub damage: u32,
    pub school: SpellSchool,
}

impl crate::Message for SMSG_SPELLDAMAGESHIELD {
    const OPCODE: u32 = 0x024f;

    fn size_without_header(&self) -> u32 {
        24
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // victim_guid: Guid
        w.write_all(&self.victim_guid.guid().to_le_bytes())?;

        // caster_guid: Guid
        w.write_all(&self.caster_guid.guid().to_le_bytes())?;

        // damage: u32
        w.write_all(&self.damage.to_le_bytes())?;

        // school: SpellSchool
        w.write_all(&(self.school.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 24 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // victim_guid: Guid
        let victim_guid = Guid::read(r)?;

        // caster_guid: Guid
        let caster_guid = Guid::read(r)?;

        // damage: u32
        let damage = crate::util::read_u32_le(r)?;

        // school: SpellSchool
        let school: SpellSchool = (crate::util::read_u32_le(r)? as u8).try_into()?;

        Ok(Self {
            victim_guid,
            caster_guid,
            damage,
            school,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_SPELLDAMAGESHIELD {}

