use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::shared::spell_steal_tbc_wrath::SpellSteal;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_spellsteallog.wowm:15`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_spellsteallog.wowm#L15):
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

impl crate::Message for SMSG_SPELLSTEALLOG {
    const OPCODE: u32 = 0x0333;

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

        // unknown: u8
        w.write_all(&self.unknown.to_le_bytes())?;

        // amount_of_spell_steals: u32
        w.write_all(&(self.spell_steals.len() as u32).to_le_bytes())?;

        // spell_steals: SpellSteal[amount_of_spell_steals]
        for i in self.spell_steals.iter() {
            i.write_into_vec(w)?;
        }

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(13..=4294967294).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0333, size: body_size as u32 });
        }

        // victim: PackedGuid
        let victim = Guid::read_packed(r)?;

        // caster: PackedGuid
        let caster = Guid::read_packed(r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(r)?;

        // unknown: u8
        let unknown = crate::util::read_u8_le(r)?;

        // amount_of_spell_steals: u32
        let amount_of_spell_steals = crate::util::read_u32_le(r)?;

        // spell_steals: SpellSteal[amount_of_spell_steals]
        let mut spell_steals = Vec::with_capacity(amount_of_spell_steals as usize);
        for i in 0..amount_of_spell_steals {
            spell_steals.push(SpellSteal::read(r)?);
        }

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
impl crate::world::tbc::ServerMessage for SMSG_SPELLSTEALLOG {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_SPELLSTEALLOG {}

impl SMSG_SPELLSTEALLOG {
    pub(crate) fn size(&self) -> usize {
        self.victim.size() // victim: Guid
        + self.caster.size() // caster: Guid
        + 4 // spell: u32
        + 1 // unknown: u8
        + 4 // amount_of_spell_steals: u32
        + self.spell_steals.len() * 5 // spell_steals: SpellSteal[amount_of_spell_steals]
    }
}

