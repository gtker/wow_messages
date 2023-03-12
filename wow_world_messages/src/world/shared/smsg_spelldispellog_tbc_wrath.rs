use std::io::{Read, Write};

use crate::Guid;
use crate::shared::dispelled_spell_tbc_wrath::DispelledSpell;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spelldispellog.wowm:10`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spelldispellog.wowm#L10):
/// ```text
/// smsg SMSG_SPELLDISPELLOG = 0x027B {
///     PackedGuid victim;
///     PackedGuid caster;
///     u32 dispell_spell;
///     u8 unknown;
///     u32 amount_of_spells;
///     DispelledSpell[amount_of_spells] spells;
/// }
/// ```
pub struct SMSG_SPELLDISPELLOG {
    pub victim: Guid,
    pub caster: Guid,
    pub dispell_spell: u32,
    /// mangosone: unused
    ///
    pub unknown: u8,
    pub spells: Vec<DispelledSpell>,
}

impl crate::Message for SMSG_SPELLDISPELLOG {
    const OPCODE: u32 = 0x027b;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // victim: PackedGuid
        self.victim.write_packed_guid_into_vec(&mut w)?;

        // caster: PackedGuid
        self.caster.write_packed_guid_into_vec(&mut w)?;

        // dispell_spell: u32
        w.write_all(&self.dispell_spell.to_le_bytes())?;

        // unknown: u8
        w.write_all(&self.unknown.to_le_bytes())?;

        // amount_of_spells: u32
        w.write_all(&(self.spells.len() as u32).to_le_bytes())?;

        // spells: DispelledSpell[amount_of_spells]
        for i in self.spells.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(13..=4294967294).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x027B, size: body_size as u32 });
        }

        // victim: PackedGuid
        let victim = Guid::read_packed(&mut r)?;

        // caster: PackedGuid
        let caster = Guid::read_packed(&mut r)?;

        // dispell_spell: u32
        let dispell_spell = crate::util::read_u32_le(&mut r)?;

        // unknown: u8
        let unknown = crate::util::read_u8_le(&mut r)?;

        // amount_of_spells: u32
        let amount_of_spells = crate::util::read_u32_le(&mut r)?;

        // spells: DispelledSpell[amount_of_spells]
        let spells = {
            let mut spells = Vec::with_capacity(amount_of_spells as usize);
            for i in 0..amount_of_spells {
                spells.push(DispelledSpell::read(&mut r)?);
            }
            spells
        };

        Ok(Self {
            victim,
            caster,
            dispell_spell,
            unknown,
            spells,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_SPELLDISPELLOG {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_SPELLDISPELLOG {}

impl SMSG_SPELLDISPELLOG {
    pub(crate) fn size(&self) -> usize {
        self.victim.size() // victim: PackedGuid
        + self.caster.size() // caster: PackedGuid
        + 4 // dispell_spell: u32
        + 1 // unknown: u8
        + 4 // amount_of_spells: u32
        + self.spells.len() * 5 // spells: DispelledSpell[amount_of_spells]
    }
}

