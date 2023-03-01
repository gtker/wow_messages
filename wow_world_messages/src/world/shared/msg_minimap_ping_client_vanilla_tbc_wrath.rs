use crate:: {
};
use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/msg_minimap_ping_client.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/msg_minimap_ping_client.wowm#L3):
/// ```text
/// cmsg MSG_MINIMAP_PING_Client = 0x01D5 {
///     f32 position_x;
///     f32 position_y;
/// }
/// ```
pub struct MSG_MINIMAP_PING_Client {
    pub position_x: f32,
    pub position_y: f32,
}

impl crate::Message for MSG_MINIMAP_PING_Client {
    const OPCODE: u32 = 0x01d5;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // position_x: f32
        w.write_all(&self.position_x.to_le_bytes())?;

        // position_y: f32
        w.write_all(&self.position_y.to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01D5, size: body_size as u32 });
        }

        // position_x: f32
        let position_x = crate::util::read_f32_le(&mut r)?;

        // position_y: f32
        let position_y = crate::util::read_f32_le(&mut r)?;

        Ok(Self {
            position_x,
            position_y,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for MSG_MINIMAP_PING_Client {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for MSG_MINIMAP_PING_Client {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for MSG_MINIMAP_PING_Client {}

