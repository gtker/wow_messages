use std::io::{Read, Write};

use std::time::Duration;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spell_cooldown.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spell_cooldown.wowm#L1):
/// ```text
/// struct SpellCooldownStatus {
///     Spell id;
///     Milliseconds cooldown_time;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SpellCooldownStatus {
    pub id: u32,
    pub cooldown_time: Duration,
}

impl SpellCooldownStatus {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // id: Spell
        w.write_all(&self.id.to_le_bytes())?;

        // cooldown_time: Milliseconds
        w.write_all((self.cooldown_time.as_millis() as u32).to_le_bytes().as_slice())?;

        Ok(())
    }
}

impl SpellCooldownStatus {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, std::io::Error> {
        // id: Spell
        let id = crate::util::read_u32_le(&mut r)?;

        // cooldown_time: Milliseconds
        let cooldown_time = Duration::from_millis(crate::util::read_u32_le(&mut r)?.into());

        Ok(Self {
            id,
            cooldown_time,
        })
    }

}

