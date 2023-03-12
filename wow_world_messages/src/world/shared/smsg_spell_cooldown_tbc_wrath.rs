use std::io::{Read, Write};

use crate::Guid;
use crate::shared::spell_cooldown_status_vanilla_tbc_wrath::SpellCooldownStatus;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spell_cooldown.wowm:15`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spell_cooldown.wowm#L15):
/// ```text
/// smsg SMSG_SPELL_COOLDOWN = 0x0134 {
///     Guid guid;
///     u8 flags;
///     SpellCooldownStatus[-] cooldowns;
/// }
/// ```
pub struct SMSG_SPELL_COOLDOWN {
    pub guid: Guid,
    pub flags: u8,
    pub cooldowns: Vec<SpellCooldownStatus>,
}

impl crate::Message for SMSG_SPELL_COOLDOWN {
    const OPCODE: u32 = 0x0134;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // flags: u8
        w.write_all(&self.flags.to_le_bytes())?;

        // cooldowns: SpellCooldownStatus[-]
        for i in self.cooldowns.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(9..=65544).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0134, size: body_size as u32 });
        }

        // guid: Guid
        let guid = Guid::read(&mut r)?;

        // flags: u8
        let flags = crate::util::read_u8_le(&mut r)?;

        // cooldowns: SpellCooldownStatus[-]
        let cooldowns = {
            let mut current_size = {
                8 // guid: Guid
                + 1 // flags: u8
            };
            let mut cooldowns = Vec::with_capacity(body_size as usize - current_size);
            while current_size < (body_size as usize) {
                cooldowns.push(SpellCooldownStatus::read(&mut r)?);
                current_size += 1;
            }
            cooldowns
        };

        Ok(Self {
            guid,
            flags,
            cooldowns,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_SPELL_COOLDOWN {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_SPELL_COOLDOWN {}

impl SMSG_SPELL_COOLDOWN {
    pub(crate) fn size(&self) -> usize {
        8 // guid: Guid
        + 1 // flags: u8
        + self.cooldowns.len() * 8 // cooldowns: SpellCooldownStatus[-]
    }
}

