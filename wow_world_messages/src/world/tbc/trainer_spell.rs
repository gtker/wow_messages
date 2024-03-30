use std::io::{Read, Write};

use crate::tbc::{
    Skill, TrainerSpellState,
};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_trainer_list.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_trainer_list.wowm#L9):
/// ```text
/// struct TrainerSpell {
///     Spell spell;
///     TrainerSpellState state;
///     u32 spell_cost;
///     u32 talent_point_cost;
///     u32 first_rank;
///     u8 required_level;
///     (u32)Skill required_skill;
///     u32 required_skill_value;
///     u32[3] required_spells;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct TrainerSpell {
    /// cmangos: learned spell (or cast-spell in profession case)
    pub spell: u32,
    pub state: TrainerSpellState,
    pub spell_cost: u32,
    /// cmangos: spells don't cost talent points
    /// cmangos: set to 0
    pub talent_point_cost: u32,
    /// cmangos: must be equal prev. field to have learn button in enabled state
    /// cmangos: 1 for true 0 for false
    pub first_rank: u32,
    pub required_level: u8,
    pub required_skill: Skill,
    pub required_skill_value: u32,
    pub required_spells: [u32; 3],
}

impl TrainerSpell {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // spell: Spell
        w.write_all(&self.spell.to_le_bytes())?;

        // state: TrainerSpellState
        w.write_all(&(self.state.as_int().to_le_bytes()))?;

        // spell_cost: u32
        w.write_all(&self.spell_cost.to_le_bytes())?;

        // talent_point_cost: u32
        w.write_all(&self.talent_point_cost.to_le_bytes())?;

        // first_rank: u32
        w.write_all(&self.first_rank.to_le_bytes())?;

        // required_level: u8
        w.write_all(&self.required_level.to_le_bytes())?;

        // required_skill: Skill
        w.write_all(&u32::from(self.required_skill.as_int()).to_le_bytes())?;

        // required_skill_value: u32
        w.write_all(&self.required_skill_value.to_le_bytes())?;

        // required_spells: u32[3]
        for i in self.required_spells.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }
}

impl TrainerSpell {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // spell: Spell
        let spell = crate::util::read_u32_le(&mut r)?;

        // state: TrainerSpellState
        let state = crate::util::read_u8_le(&mut r)?.try_into()?;

        // spell_cost: u32
        let spell_cost = crate::util::read_u32_le(&mut r)?;

        // talent_point_cost: u32
        let talent_point_cost = crate::util::read_u32_le(&mut r)?;

        // first_rank: u32
        let first_rank = crate::util::read_u32_le(&mut r)?;

        // required_level: u8
        let required_level = crate::util::read_u8_le(&mut r)?;

        // required_skill: Skill
        let required_skill = (crate::util::read_u32_le(&mut r)? as u16).try_into()?;

        // required_skill_value: u32
        let required_skill_value = crate::util::read_u32_le(&mut r)?;

        // required_spells: u32[3]
        let required_spells = {
            let mut required_spells = [u32::default(); 3];
            for i in required_spells.iter_mut() {
                *i = crate::util::read_u32_le(&mut r)?;
            }
            required_spells
        };

        Ok(Self {
            spell,
            state,
            spell_cost,
            talent_point_cost,
            first_rank,
            required_level,
            required_skill,
            required_skill_value,
            required_spells,
        })
    }

}

