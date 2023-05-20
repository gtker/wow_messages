use std::io::{Read, Write};

use crate::Guid;
use crate::shared::spell_steal_tbc_wrath::SpellSteal;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spellsteallog.wowm:15`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spellsteallog.wowm#L15):
/// ```text
/// smsg SMSG_SPELLSTEALLOG = 0x0333 {
///     PackedGuid victim;
///     PackedGuid caster;
///     u32 spell;
///     u8 unknown;
///     u32 amount_of_spell_steals;
///     SpellSteal[amount_of_spell_steals] spell_steals;
/// }
/// ```
pub struct SMSG_SPELLSTEALLOG {
    pub victim: Guid,
    pub caster: Guid,
    pub spell: u32,
    pub unknown: u8,
    pub spell_steals: Vec<SpellSteal>,
}

impl crate::private::Sealed for SMSG_SPELLSTEALLOG {}
impl crate::Message for SMSG_SPELLSTEALLOG {
    const OPCODE: u32 = 0x0333;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // victim: PackedGuid
        self.victim.write_packed_guid_into_vec(&mut w)?;

        // caster: PackedGuid
        self.caster.write_packed_guid_into_vec(&mut w)?;

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // unknown: u8
        w.write_all(&self.unknown.to_le_bytes())?;

        // amount_of_spell_steals: u32
        w.write_all(&(self.spell_steals.len() as u32).to_le_bytes())?;

        // spell_steals: SpellSteal[amount_of_spell_steals]
        for i in self.spell_steals.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(13..=16777215).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0333, size: body_size });
        }

        // victim: PackedGuid
        let victim = Guid::read_packed(&mut r)?;

        // caster: PackedGuid
        let caster = Guid::read_packed(&mut r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(&mut r)?;

        // unknown: u8
        let unknown = crate::util::read_u8_le(&mut r)?;

        // amount_of_spell_steals: u32
        let amount_of_spell_steals = crate::util::read_u32_le(&mut r)?;

        // spell_steals: SpellSteal[amount_of_spell_steals]
        let spell_steals = {
            let mut spell_steals = Vec::with_capacity(amount_of_spell_steals as usize);
            for _ in 0..amount_of_spell_steals {
                spell_steals.push(SpellSteal::read(&mut r)?);
            }
            spell_steals
        };

        Ok(Self {
            victim,
            caster,
            spell,
            unknown,
            spell_steals,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_SPELLSTEALLOG {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_SPELLSTEALLOG {}

impl SMSG_SPELLSTEALLOG {
    pub(crate) fn size(&self) -> usize {
        self.victim.size() // victim: PackedGuid
        + self.caster.size() // caster: PackedGuid
        + 4 // spell: u32
        + 1 // unknown: u8
        + 4 // amount_of_spell_steals: u32
        + self.spell_steals.len() * 5 // spell_steals: SpellSteal[amount_of_spell_steals]
    }
}

