use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
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

impl ServerMessage for MSG_MINIMAP_PING_Server {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // position_x: f32
        w.write_all(&self.position_x.to_le_bytes())?;

        // position_y: f32
        w.write_all(&self.position_y.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x01d5;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        16
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 16 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // guid: Guid
        let guid = Guid::read(r)?;

        // position_x: f32
        let position_x = crate::util::read_f32_le(r)?;
        // position_y: f32
        let position_y = crate::util::read_f32_le(r)?;
        Ok(Self {
            guid,
            position_x,
            position_y,
        })
    }

}

