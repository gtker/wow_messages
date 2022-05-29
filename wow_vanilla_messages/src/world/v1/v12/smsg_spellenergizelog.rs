use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::PowerType;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
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
        w.write_all(&self.victim_guid.packed_guid())?;

        // caster_guid: PackedGuid
        w.write_all(&self.caster_guid.packed_guid())?;

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // power: PowerType
        w.write_all(&(self.power.as_int() as u32).to_le_bytes())?;

        // damage: u32
        w.write_all(&self.damage.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0151;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = crate::errors::ParseError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
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

