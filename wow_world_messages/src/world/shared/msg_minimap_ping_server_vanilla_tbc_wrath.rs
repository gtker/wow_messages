use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/msg_minimap_ping_server.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/msg_minimap_ping_server.wowm#L3):
/// ```text
/// smsg MSG_MINIMAP_PING_Server = 0x01D5 {
///     Guid guid;
///     f32 position_x;
///     f32 position_y;
/// }
/// ```
pub struct MSG_MINIMAP_PING_Server {
    pub guid: Guid,
    pub position_x: f32,
    pub position_y: f32,
}

impl crate::private::Sealed for MSG_MINIMAP_PING_Server {}
impl crate::Message for MSG_MINIMAP_PING_Server {
    const OPCODE: u32 = 0x01d5;

    fn size_without_header(&self) -> u32 {
        16
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // position_x: f32
        w.write_all(&self.position_x.to_le_bytes())?;

        // position_y: f32
        w.write_all(&self.position_y.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 16 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01D5, size: body_size });
        }

        // guid: Guid
        let guid = Guid::read(&mut r)?;

        // position_x: f32
        let position_x = crate::util::read_f32_le(&mut r)?;

        // position_y: f32
        let position_y = crate::util::read_f32_le(&mut r)?;

        Ok(Self {
            guid,
            position_x,
            position_y,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for MSG_MINIMAP_PING_Server {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for MSG_MINIMAP_PING_Server {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for MSG_MINIMAP_PING_Server {}

