use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm#L3):
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
    /// let the database control the sign here. negative means that the item should be consumed once the charges are consumed.
    ///
    pub spell_charges: u32,
    pub spell_cooldown: u32,
    pub spell_category: u32,
    pub spell_category_cooldown: u32,
}

impl ItemSpells {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
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

impl ItemSpells {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
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

}

