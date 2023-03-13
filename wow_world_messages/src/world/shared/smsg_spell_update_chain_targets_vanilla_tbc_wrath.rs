use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spell_update_chain_targets.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spell_update_chain_targets.wowm#L3):
/// ```text
/// smsg SMSG_SPELL_UPDATE_CHAIN_TARGETS = 0x0330 {
///     Guid caster;
///     u32 spell;
///     u32 amount_of_targets;
///     Guid[amount_of_targets] targets;
/// }
/// ```
pub struct SMSG_SPELL_UPDATE_CHAIN_TARGETS {
    pub caster: Guid,
    pub spell: u32,
    pub targets: Vec<Guid>,
}

impl crate::Message for SMSG_SPELL_UPDATE_CHAIN_TARGETS {
    const OPCODE: u32 = 0x0330;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // caster: Guid
        w.write_all(&self.caster.guid().to_le_bytes())?;

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // amount_of_targets: u32
        w.write_all(&(self.targets.len() as u32).to_le_bytes())?;

        // targets: Guid[amount_of_targets]
        for i in self.targets.iter() {
            w.write_all(&i.guid().to_le_bytes())?;
        }

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(16..=16777215).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0330, size: body_size as u32 });
        }

        // caster: Guid
        let caster = Guid::read(&mut r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(&mut r)?;

        // amount_of_targets: u32
        let amount_of_targets = crate::util::read_u32_le(&mut r)?;

        // targets: Guid[amount_of_targets]
        let targets = {
            let mut targets = Vec::with_capacity(amount_of_targets as usize);
            for i in 0..amount_of_targets {
                targets.push(Guid::read(&mut r)?);
            }
            targets
        };

        Ok(Self {
            caster,
            spell,
            targets,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_SPELL_UPDATE_CHAIN_TARGETS {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_SPELL_UPDATE_CHAIN_TARGETS {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_SPELL_UPDATE_CHAIN_TARGETS {}

impl SMSG_SPELL_UPDATE_CHAIN_TARGETS {
    pub(crate) fn size(&self) -> usize {
        8 // caster: Guid
        + 4 // spell: u32
        + 4 // amount_of_targets: u32
        + self.targets.len() *  8 // targets: Guid[amount_of_targets]
    }
}

