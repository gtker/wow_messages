use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Copy, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_clear_cooldown.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_clear_cooldown.wowm#L3):
/// ```text
/// smsg SMSG_CLEAR_COOLDOWN = 0x01DE {
///     u32 id;
///     Guid target_guid;
/// }
/// ```
pub struct SMSG_CLEAR_COOLDOWN {
    pub id: u32,
    pub target_guid: Guid,
}

impl crate::Message for SMSG_CLEAR_COOLDOWN {
    const OPCODE: u32 = 0x01de;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // target_guid: Guid
        w.write_all(&self.target_guid.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // id: u32
        let id = crate::util::read_u32_le(r)?;

        // target_guid: Guid
        let target_guid = Guid::read(r)?;

        Ok(Self {
            id,
            target_guid,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_CLEAR_COOLDOWN {}

