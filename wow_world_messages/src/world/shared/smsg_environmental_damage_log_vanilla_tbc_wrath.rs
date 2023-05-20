use std::io::{Read, Write};

use crate::Guid;
use wow_world_base::shared::environmental_damage_type_vanilla_tbc_wrath::EnvironmentalDamageType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/combat/smsg_environmentaldamagelog.wowm:12`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/combat/smsg_environmentaldamagelog.wowm#L12):
/// ```text
/// smsg SMSG_ENVIRONMENTAL_DAMAGE_LOG = 0x01FC {
///     Guid guid;
///     EnvironmentalDamageType damage_type;
///     u32 damage;
///     u32 absorb;
///     u32 resist;
/// }
/// ```
pub struct SMSG_ENVIRONMENTAL_DAMAGE_LOG {
    pub guid: Guid,
    pub damage_type: EnvironmentalDamageType,
    pub damage: u32,
    pub absorb: u32,
    pub resist: u32,
}

impl crate::private::Sealed for SMSG_ENVIRONMENTAL_DAMAGE_LOG {}
impl crate::Message for SMSG_ENVIRONMENTAL_DAMAGE_LOG {
    const OPCODE: u32 = 0x01fc;

    fn size_without_header(&self) -> u32 {
        21
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // damage_type: EnvironmentalDamageType
        w.write_all(&(self.damage_type.as_int().to_le_bytes()))?;

        // damage: u32
        w.write_all(&self.damage.to_le_bytes())?;

        // absorb: u32
        w.write_all(&self.absorb.to_le_bytes())?;

        // resist: u32
        w.write_all(&self.resist.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 21 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01FC, size: body_size });
        }

        // guid: Guid
        let guid = Guid::read(&mut r)?;

        // damage_type: EnvironmentalDamageType
        let damage_type: EnvironmentalDamageType = crate::util::read_u8_le(&mut r)?.try_into()?;

        // damage: u32
        let damage = crate::util::read_u32_le(&mut r)?;

        // absorb: u32
        let absorb = crate::util::read_u32_le(&mut r)?;

        // resist: u32
        let resist = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            guid,
            damage_type,
            damage,
            absorb,
            resist,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_ENVIRONMENTAL_DAMAGE_LOG {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_ENVIRONMENTAL_DAMAGE_LOG {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_ENVIRONMENTAL_DAMAGE_LOG {}

