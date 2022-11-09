use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_initial_spells.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_initial_spells.wowm#L1):
/// ```text
/// struct CooldownSpell {
///     u16 spell_id;
///     u16 item_id;
///     u16 spell_category;
///     u32 cooldown_in_msecs;
///     u32 category_cooldown_in_msecs;
/// }
/// ```
pub struct CooldownSpell {
    pub spell_id: u16,
    /// cmangos/mangoszero: cast item id
    ///
    pub item_id: u16,
    pub spell_category: u16,
    pub cooldown_in_msecs: u32,
    pub category_cooldown_in_msecs: u32,
}

impl CooldownSpell {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // spell_id: u16
        w.write_all(&self.spell_id.to_le_bytes())?;

        // item_id: u16
        w.write_all(&self.item_id.to_le_bytes())?;

        // spell_category: u16
        w.write_all(&self.spell_category.to_le_bytes())?;

        // cooldown_in_msecs: u32
        w.write_all(&self.cooldown_in_msecs.to_le_bytes())?;

        // category_cooldown_in_msecs: u32
        w.write_all(&self.category_cooldown_in_msecs.to_le_bytes())?;

        Ok(())
    }
}

impl CooldownSpell {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // spell_id: u16
        let spell_id = crate::util::read_u16_le(r)?;

        // item_id: u16
        let item_id = crate::util::read_u16_le(r)?;

        // spell_category: u16
        let spell_category = crate::util::read_u16_le(r)?;

        // cooldown_in_msecs: u32
        let cooldown_in_msecs = crate::util::read_u32_le(r)?;

        // category_cooldown_in_msecs: u32
        let category_cooldown_in_msecs = crate::util::read_u32_le(r)?;

        Ok(Self {
            spell_id,
            item_id,
            spell_category,
            cooldown_in_msecs,
            category_cooldown_in_msecs,
        })
    }

}

