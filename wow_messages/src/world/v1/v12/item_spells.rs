use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/9needs_optional/needs_optional.wowm:36`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/9needs_optional/needs_optional.wowm#L36):
/// ```text
/// struct ItemSpells {
///     u32 spell;
///     u32 spell_trigger;
///     u32 spell_charges;
///     u32 spell_cooldown;
///     u32 spell_category;
///     u32 spell_category_cooldown;
/// }
/// ```
pub struct ItemSpells {
    pub spell: u32,
    pub spell_trigger: u32,
    pub spell_charges: u32,
    pub spell_cooldown: u32,
    pub spell_category: u32,
    pub spell_category_cooldown: u32,
}

impl ReadableAndWritable for ItemSpells {
    type Error = std::io::Error;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // spell: u32
        let spell = crate::util::read_u32_le(r)?;

        // spell_trigger: u32
        let spell_trigger = crate::util::read_u32_le(r)?;

        // spell_charges: u32
        let spell_charges = crate::util::read_u32_le(r)?;

        // spell_cooldown: u32
        let spell_cooldown = crate::util::read_u32_le(r)?;

        // spell_category: u32
        let spell_category = crate::util::read_u32_le(r)?;

        // spell_category_cooldown: u32
        let spell_category_cooldown = crate::util::read_u32_le(r)?;

        Ok(Self {
            spell,
            spell_trigger,
            spell_charges,
            spell_cooldown,
            spell_category,
            spell_category_cooldown,
        })
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // spell_trigger: u32
        w.write_all(&self.spell_trigger.to_le_bytes())?;

        // spell_charges: u32
        w.write_all(&self.spell_charges.to_le_bytes())?;

        // spell_cooldown: u32
        w.write_all(&self.spell_cooldown.to_le_bytes())?;

        // spell_category: u32
        w.write_all(&self.spell_category.to_le_bytes())?;

        // spell_category_cooldown: u32
        w.write_all(&self.spell_category_cooldown.to_le_bytes())?;

        Ok(())
    }

}

impl ConstantSized for ItemSpells {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for ItemSpells {
    fn maximum_possible_size() -> usize {
        4 // spell: u32
        + 4 // spell_trigger: u32
        + 4 // spell_charges: u32
        + 4 // spell_cooldown: u32
        + 4 // spell_category: u32
        + 4 // spell_category_cooldown: u32
    }
}

