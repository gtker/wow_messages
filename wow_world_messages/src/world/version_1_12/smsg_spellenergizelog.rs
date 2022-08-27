use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::version_1_12::PowerType;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spellenergizelog.wowm:24`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spellenergizelog.wowm#L24):
/// ```text
/// smsg SMSG_SPELLENERGIZELOG = 0x0151 {
///     PackedGuid victim_guid;
///     PackedGuid caster_guid;
///     u32 spell;
///     PowerType power;
///     u32 damage;
/// }
/// ```
pub struct SMSG_SPELLENERGIZELOG {
    pub victim_guid: Guid,
    pub caster_guid: Guid,
    pub spell: u32,
    pub power: PowerType,
    pub damage: u32,
}

impl ServerMessage for SMSG_SPELLENERGIZELOG {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // victim_guid: PackedGuid
        self.victim_guid.write_packed_guid_into_vec(w);

        // caster_guid: PackedGuid
        self.caster_guid.write_packed_guid_into_vec(w);

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // power: PowerType
        w.write_all(&(self.power.as_int() as u32).to_le_bytes())?;

        // damage: u32
        w.write_all(&self.damage.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0151;

    fn server_size(&self) -> u16 {
        (self.size() + 4) as u16
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // victim_guid: PackedGuid
        let victim_guid = Guid::read_packed(r)?;

        // caster_guid: PackedGuid
        let caster_guid = Guid::read_packed(r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(r)?;

        // power: PowerType
        let power: PowerType = crate::util::read_u32_le(r)?.try_into()?;

        // damage: u32
        let damage = crate::util::read_u32_le(r)?;

        Ok(Self {
            victim_guid,
            caster_guid,
            spell,
            power,
            damage,
        })
    }

}

impl SMSG_SPELLENERGIZELOG {
    pub(crate) fn size(&self) -> usize {
        self.victim_guid.size() // victim_guid: Guid
        + self.caster_guid.size() // caster_guid: Guid
        + 4 // spell: u32
        + 4 // power: PowerType
        + 4 // damage: u32
    }
}

