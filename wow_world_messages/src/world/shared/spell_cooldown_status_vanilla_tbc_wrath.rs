use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spell_cooldown.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spell_cooldown.wowm#L1):
/// ```text
/// struct SpellCooldownStatus {
///     u32 id;
///     u32 cooldown_time_in_msecs;
/// }
/// ```
pub struct SpellCooldownStatus {
    pub id: u32,
    pub cooldown_time_in_msecs: u32,
}

impl SpellCooldownStatus {
    pub(crate) fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // cooldown_time_in_msecs: u32
        w.write_all(&self.cooldown_time_in_msecs.to_le_bytes())?;

        Ok(())
    }
}

impl SpellCooldownStatus {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> std::result::Result<Self, std::io::Error> {
        // id: u32
        let id = crate::util::read_u32_le(&mut r)?;

        // cooldown_time_in_msecs: u32
        let cooldown_time_in_msecs = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            id,
            cooldown_time_in_msecs,
        })
    }

}

