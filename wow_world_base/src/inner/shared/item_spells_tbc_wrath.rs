use std::io::{Read, Write};

use crate::shared::spell_trigger_type_tbc_wrath::SpellTriggerType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm:71`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm#L71):
/// ```text
/// struct ItemSpells {
///     u32 spell;
///     (u32)SpellTriggerType spell_trigger;
///     i32 spell_charges;
///     i32 spell_cooldown;
///     u32 spell_category;
///     i32 spell_category_cooldown;
/// }
/// ```
pub struct ItemSpells {
    pub spell: u32,
    pub spell_trigger: SpellTriggerType,
    /// let the database control the sign here. negative means that the item should be consumed once the charges are consumed.
    pub spell_charges: i32,
    pub spell_cooldown: i32,
    pub spell_category: u32,
    pub spell_category_cooldown: i32,
}

impl ItemSpells {
    pub fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // spell_trigger: SpellTriggerType
        w.write_all(&u32::from(self.spell_trigger.as_int()).to_le_bytes())?;

        // spell_charges: i32
        w.write_all(&self.spell_charges.to_le_bytes())?;

        // spell_cooldown: i32
        w.write_all(&self.spell_cooldown.to_le_bytes())?;

        // spell_category: u32
        w.write_all(&self.spell_category.to_le_bytes())?;

        // spell_category_cooldown: i32
        w.write_all(&self.spell_category_cooldown.to_le_bytes())?;

        Ok(())
    }
}

impl ItemSpells {
    pub fn read<R: Read>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // spell: u32
        let spell = crate::util::read_u32_le(&mut r)?;

        // spell_trigger: SpellTriggerType
        let spell_trigger = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

        // spell_charges: i32
        let spell_charges = crate::util::read_i32_le(&mut r)?;

        // spell_cooldown: i32
        let spell_cooldown = crate::util::read_i32_le(&mut r)?;

        // spell_category: u32
        let spell_category = crate::util::read_u32_le(&mut r)?;

        // spell_category_cooldown: i32
        let spell_category_cooldown = crate::util::read_i32_le(&mut r)?;

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

