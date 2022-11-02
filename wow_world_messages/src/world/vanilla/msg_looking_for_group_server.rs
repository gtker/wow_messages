use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/msg_looking_for_group_server.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/msg_looking_for_group_server.wowm#L3):
/// ```text
/// smsg MSG_LOOKING_FOR_GROUP_Server = 0x01FF {
///     u32 unknown1;
/// }
/// ```
pub struct MSG_LOOKING_FOR_GROUP_Server {
    /// vmangos sets to 0. cmangos/mangoszero don't implement
    ///
    pub unknown1: u32,
}

impl crate::Message for MSG_LOOKING_FOR_GROUP_Server {
    const OPCODE: u32 = 0x01ff;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01FF, size: body_size as u32 });
        }

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(r)?;

        Ok(Self {
            unknown1,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for MSG_LOOKING_FOR_GROUP_Server {}

