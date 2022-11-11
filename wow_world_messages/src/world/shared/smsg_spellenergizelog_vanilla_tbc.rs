use std::convert::{TryFrom, TryInto};
use crate::Guid;
use wow_world_base::vanilla::Power;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spellenergizelog.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spellenergizelog.wowm#L1):
/// ```text
/// smsg SMSG_SPELLENERGIZELOG = 0x0151 {
///     PackedGuid victim;
///     PackedGuid caster;
///     u32 spell;
///     (u32)Power power;
///     u32 damage;
/// }
/// ```
pub struct SMSG_SPELLENERGIZELOG {
    pub victim: Guid,
    pub caster: Guid,
    pub spell: u32,
    pub power: Power,
    pub damage: u32,
}

impl crate::Message for SMSG_SPELLENERGIZELOG {
    const OPCODE: u32 = 0x0151;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // victim: PackedGuid
        self.victim.write_packed_guid_into_vec(w);

        // caster: PackedGuid
        self.caster.write_packed_guid_into_vec(w);

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // power: Power
        w.write_all(&(self.power.as_int() as u32).to_le_bytes())?;

        // damage: u32
        w.write_all(&self.damage.to_le_bytes())?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(16..=30).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0151, size: body_size as u32 });
        }

        // victim: PackedGuid
        let victim = Guid::read_packed(r)?;

        // caster: PackedGuid
        let caster = Guid::read_packed(r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(r)?;

        // power: Power
        let power: Power = (crate::util::read_u32_le(r)? as u8).try_into()?;

        // damage: u32
        let damage = crate::util::read_u32_le(r)?;

        Ok(Self {
            victim,
            caster,
            spell,
            power,
            damage,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_SPELLENERGIZELOG {}

#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for SMSG_SPELLENERGIZELOG {}

impl SMSG_SPELLENERGIZELOG {
    pub(crate) fn size(&self) -> usize {
        self.victim.size() // victim: Guid
        + self.caster.size() // caster: Guid
        + 4 // spell: u32
        + 4 // power: Power
        + 4 // damage: u32
    }
}

