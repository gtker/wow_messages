use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/combat/smsg_attackstart.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/combat/smsg_attackstart.wowm#L3):
/// ```text
/// smsg SMSG_ATTACKSTART = 0x0143 {
///     Guid attacker;
///     Guid victim;
/// }
/// ```
pub struct SMSG_ATTACKSTART {
    pub attacker: Guid,
    pub victim: Guid,
}

impl crate::Message for SMSG_ATTACKSTART {
    const OPCODE: u32 = 0x0143;

    fn size_without_header(&self) -> u32 {
        16
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // attacker: Guid
        w.write_all(&self.attacker.guid().to_le_bytes())?;

        // victim: Guid
        w.write_all(&self.victim.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 16 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0143, size: body_size as u32 });
        }

        // attacker: Guid
        let attacker = Guid::read(r)?;

        // victim: Guid
        let victim = Guid::read(r)?;

        Ok(Self {
            attacker,
            victim,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_ATTACKSTART {}

#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for SMSG_ATTACKSTART {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_ATTACKSTART {}

