use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::vanilla::ServerMessage;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spellheallog.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spellheallog.wowm#L3):
/// ```text
/// smsg SMSG_SPELLHEALLOG = 0x0150 {
///     PackedGuid victim_guid;
///     PackedGuid caster_guid;
///     u32 id;
///     u32 damage;
///     u8 critical;
/// }
/// ```
pub struct SMSG_SPELLHEALLOG {
    pub victim_guid: Guid,
    pub caster_guid: Guid,
    pub id: u32,
    pub damage: u32,
    pub critical: u8,
}

impl crate::Message for SMSG_SPELLHEALLOG {
    const OPCODE: u32 = 0x0150;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // victim_guid: PackedGuid
        self.victim_guid.write_packed_guid_into_vec(w);

        // caster_guid: PackedGuid
        self.caster_guid.write_packed_guid_into_vec(w);

        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // damage: u32
        w.write_all(&self.damage.to_le_bytes())?;

        // critical: u8
        w.write_all(&self.critical.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // victim_guid: PackedGuid
        let victim_guid = Guid::read_packed(r)?;

        // caster_guid: PackedGuid
        let caster_guid = Guid::read_packed(r)?;

        // id: u32
        let id = crate::util::read_u32_le(r)?;

        // damage: u32
        let damage = crate::util::read_u32_le(r)?;

        // critical: u8
        let critical = crate::util::read_u8_le(r)?;

        Ok(Self {
            victim_guid,
            caster_guid,
            id,
            damage,
            critical,
        })
    }

}
impl ServerMessage for SMSG_SPELLHEALLOG {}

impl SMSG_SPELLHEALLOG {
    pub(crate) fn size(&self) -> usize {
        self.victim_guid.size() // victim_guid: Guid
        + self.caster_guid.size() // caster_guid: Guid
        + 4 // id: u32
        + 4 // damage: u32
        + 1 // critical: u8
    }
}

