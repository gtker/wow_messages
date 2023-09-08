use std::io::{Read, Write};

use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/smsg_pet_spells.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/smsg_pet_spells.wowm#L1):
/// ```text
/// struct PetSpellCooldown {
///     u16 spell;
///     u16 spell_category;
///     Milliseconds cooldown;
///     Milliseconds category_cooldown;
/// }
/// ```
pub struct PetSpellCooldown {
    pub spell: u16,
    /// mangoszero: sets to 0
    pub spell_category: u16,
    pub cooldown: Duration,
    pub category_cooldown: Duration,
}

impl PetSpellCooldown {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // spell: u16
        w.write_all(&self.spell.to_le_bytes())?;

        // spell_category: u16
        w.write_all(&self.spell_category.to_le_bytes())?;

        // cooldown: Milliseconds
        w.write_all((self.cooldown.as_millis() as u32).to_le_bytes().as_slice())?;

        // category_cooldown: Milliseconds
        w.write_all((self.category_cooldown.as_millis() as u32).to_le_bytes().as_slice())?;

        Ok(())
    }
}

impl PetSpellCooldown {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, std::io::Error> {
        // spell: u16
        let spell = crate::util::read_u16_le(&mut r)?;

        // spell_category: u16
        let spell_category = crate::util::read_u16_le(&mut r)?;

        // cooldown: Milliseconds
        let cooldown = Duration::from_millis(crate::util::read_u32_le(&mut r)?.into());

        // category_cooldown: Milliseconds
        let category_cooldown = Duration::from_millis(crate::util::read_u32_le(&mut r)?.into());

        Ok(Self {
            spell,
            spell_category,
            cooldown,
            category_cooldown,
        })
    }

}

