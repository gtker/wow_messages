use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::vanilla::SpellMiss;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spelllogmiss.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spelllogmiss.wowm#L3):
/// ```text
/// smsg SMSG_SPELLLOGMISS = 0x024B {
///     u32 id;
///     Guid caster_guid;
///     u8 unknown1;
///     u32 amount_of_targets;
///     SpellMiss[amount_of_targets] targets;
/// }
/// ```
pub struct SMSG_SPELLLOGMISS {
    pub id: u32,
    pub caster_guid: Guid,
    /// cmangos/mangoszero: can be 0 or 1
    ///
    pub unknown1: u8,
    pub targets: Vec<SpellMiss>,
}

impl crate::Message for SMSG_SPELLLOGMISS {
    const OPCODE: u32 = 0x024b;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // caster_guid: Guid
        w.write_all(&self.caster_guid.guid().to_le_bytes())?;

        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes())?;

        // amount_of_targets: u32
        w.write_all(&(self.targets.len() as u32).to_le_bytes())?;

        // targets: SpellMiss[amount_of_targets]
        for i in self.targets.iter() {
            i.write_into_vec(w)?;
        }

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // id: u32
        let id = crate::util::read_u32_le(r)?;

        // caster_guid: Guid
        let caster_guid = Guid::read(r)?;

        // unknown1: u8
        let unknown1 = crate::util::read_u8_le(r)?;

        // amount_of_targets: u32
        let amount_of_targets = crate::util::read_u32_le(r)?;

        // targets: SpellMiss[amount_of_targets]
        let mut targets = Vec::with_capacity(amount_of_targets as usize);
        for i in 0..amount_of_targets {
            targets.push(SpellMiss::read(r)?);
        }

        Ok(Self {
            id,
            caster_guid,
            unknown1,
            targets,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_SPELLLOGMISS {}

impl SMSG_SPELLLOGMISS {
    pub(crate) fn size(&self) -> usize {
        4 // id: u32
        + 8 // caster_guid: Guid
        + 1 // unknown1: u8
        + 4 // amount_of_targets: u32
        + self.targets.len() * 12 // targets: SpellMiss[amount_of_targets]
    }
}

