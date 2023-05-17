use std::io::{Read, Write};

use crate::Guid;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_modify_cooldown.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_modify_cooldown.wowm#L1):
/// ```text
/// smsg SMSG_MODIFY_COOLDOWN = 0x0491 {
///     u32 spell;
///     Guid player;
///     Milliseconds cooldown;
/// }
/// ```
pub struct SMSG_MODIFY_COOLDOWN {
    pub spell: u32,
    pub player: Guid,
    pub cooldown: Duration,
}

impl crate::private::Sealed for SMSG_MODIFY_COOLDOWN {}
impl crate::Message for SMSG_MODIFY_COOLDOWN {
    const OPCODE: u32 = 0x0491;

    fn size_without_header(&self) -> u32 {
        16
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // player: Guid
        w.write_all(&self.player.guid().to_le_bytes())?;

        // cooldown: Milliseconds
        w.write_all((self.cooldown.as_millis() as u32).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 16 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0491, size: body_size });
        }

        // spell: u32
        let spell = crate::util::read_u32_le(&mut r)?;

        // player: Guid
        let player = Guid::read(&mut r)?;

        // cooldown: Milliseconds
        let cooldown = Duration::from_millis(crate::util::read_u32_le(&mut r)?.into());

        Ok(Self {
            spell,
            player,
            cooldown,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_MODIFY_COOLDOWN {}

