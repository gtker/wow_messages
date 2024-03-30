use std::io::{Read, Write};

use std::time::Duration;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_initial_spells.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_initial_spells.wowm#L1):
/// ```text
/// struct CooldownSpell {
///     u16 spell_id;
///     u16 item_id;
///     u16 spell_category;
///     Milliseconds cooldown;
///     Milliseconds category_cooldown;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CooldownSpell {
    pub spell_id: u16,
    /// cmangos/mangoszero: cast item id
    pub item_id: u16,
    pub spell_category: u16,
    pub cooldown: Duration,
    pub category_cooldown: Duration,
}

impl CooldownSpell {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // spell_id: u16
        w.write_all(&self.spell_id.to_le_bytes())?;

        // item_id: u16
        w.write_all(&self.item_id.to_le_bytes())?;

        // spell_category: u16
        w.write_all(&self.spell_category.to_le_bytes())?;

        // cooldown: Milliseconds
        w.write_all((self.cooldown.as_millis() as u32).to_le_bytes().as_slice())?;

        // category_cooldown: Milliseconds
        w.write_all((self.category_cooldown.as_millis() as u32).to_le_bytes().as_slice())?;

        Ok(())
    }
}

impl CooldownSpell {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, std::io::Error> {
        // spell_id: u16
        let spell_id = crate::util::read_u16_le(&mut r)?;

        // item_id: u16
        let item_id = crate::util::read_u16_le(&mut r)?;

        // spell_category: u16
        let spell_category = crate::util::read_u16_le(&mut r)?;

        // cooldown: Milliseconds
        let cooldown = Duration::from_millis(crate::util::read_u32_le(&mut r)?.into());

        // category_cooldown: Milliseconds
        let category_cooldown = Duration::from_millis(crate::util::read_u32_le(&mut r)?.into());

        Ok(Self {
            spell_id,
            item_id,
            spell_category,
            cooldown,
            category_cooldown,
        })
    }

}

