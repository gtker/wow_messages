use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::vanilla::SpellCooldownStatus;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spell_cooldown.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spell_cooldown.wowm#L8):
/// ```text
/// smsg SMSG_SPELL_COOLDOWN = 0x0134 {
///     Guid guid;
///     SpellCooldownStatus[-] cooldowns;
/// }
/// ```
pub struct SMSG_SPELL_COOLDOWN {
    pub guid: Guid,
    pub cooldowns: Vec<SpellCooldownStatus>,
}

impl crate::Message for SMSG_SPELL_COOLDOWN {
    const OPCODE: u32 = 0x0134;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // cooldowns: SpellCooldownStatus[-]
        for i in self.cooldowns.iter() {
            i.write_into_vec(w)?;
        }

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // cooldowns: SpellCooldownStatus[-]
        let mut current_size = {
            8 // guid: Guid
        };
        let mut cooldowns = Vec::with_capacity(body_size as usize - current_size);
        while current_size < (body_size as usize) {
            cooldowns.push(SpellCooldownStatus::read(r)?);
            current_size += 1;
        }

        Ok(Self {
            guid,
            cooldowns,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_SPELL_COOLDOWN {}

impl SMSG_SPELL_COOLDOWN {
    pub(crate) fn size(&self) -> usize {
        8 // guid: Guid
        + self.cooldowns.len() * 8 // cooldowns: SpellCooldownStatus[-]
    }
}

