use std::io::{Read, Write};

use crate::Guid;
use crate::shared::spell_log_miss_vanilla_tbc_wrath::SpellLogMiss;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spelllogmiss.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spelllogmiss.wowm#L8):
/// ```text
/// smsg SMSG_SPELLLOGMISS = 0x024B {
///     u32 id;
///     Guid caster;
///     u8 unknown1;
///     u32 amount_of_targets;
///     SpellLogMiss[amount_of_targets] targets;
/// }
/// ```
pub struct SMSG_SPELLLOGMISS {
    pub id: u32,
    pub caster: Guid,
    /// cmangos/mangoszero: can be 0 or 1
    ///
    pub unknown1: u8,
    pub targets: Vec<SpellLogMiss>,
}

impl crate::Message for SMSG_SPELLLOGMISS {
    const OPCODE: u32 = 0x024b;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // caster: Guid
        w.write_all(&self.caster.guid().to_le_bytes())?;

        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes())?;

        // amount_of_targets: u32
        w.write_all(&(self.targets.len() as u32).to_le_bytes())?;

        // targets: SpellLogMiss[amount_of_targets]
        for i in self.targets.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(17..=16777215).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x024B, size: body_size as u32 });
        }

        // id: u32
        let id = crate::util::read_u32_le(&mut r)?;

        // caster: Guid
        let caster = Guid::read(&mut r)?;

        // unknown1: u8
        let unknown1 = crate::util::read_u8_le(&mut r)?;

        // amount_of_targets: u32
        let amount_of_targets = crate::util::read_u32_le(&mut r)?;

        // targets: SpellLogMiss[amount_of_targets]
        let targets = {
            let mut targets = Vec::with_capacity(amount_of_targets as usize);
            for i in 0..amount_of_targets {
                targets.push(SpellLogMiss::read(&mut r)?);
            }
            targets
        };

        Ok(Self {
            id,
            caster,
            unknown1,
            targets,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_SPELLLOGMISS {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_SPELLLOGMISS {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_SPELLLOGMISS {}

impl SMSG_SPELLLOGMISS {
    pub(crate) fn size(&self) -> usize {
        4 // id: u32
        + 8 // caster: Guid
        + 1 // unknown1: u8
        + 4 // amount_of_targets: u32
        + self.targets.len() * 12 // targets: SpellLogMiss[amount_of_targets]
    }
}

