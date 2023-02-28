use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Acknowledge from the client that it has received an [`SMSG_NEW_WORLD`](crate::vanilla::SMSG_NEW_WORLD) and has loaded the new map.
///
/// Despite the name this seems to only be sent by the client.
/// The server should reply with what it normally does to log players into the world.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/msg/msg_move_worldport_ack.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/msg/msg_move_worldport_ack.wowm#L3):
/// ```text
/// msg MSG_MOVE_WORLDPORT_ACK = 0x00DC {
/// }
/// ```
pub struct MSG_MOVE_WORLDPORT_ACK {
}

impl crate::Message for MSG_MOVE_WORLDPORT_ACK {
    const OPCODE: u32 = 0x00dc;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x00DC, size: body_size as u32 });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for MSG_MOVE_WORLDPORT_ACK {}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for MSG_MOVE_WORLDPORT_ACK {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for MSG_MOVE_WORLDPORT_ACK {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for MSG_MOVE_WORLDPORT_ACK {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for MSG_MOVE_WORLDPORT_ACK {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for MSG_MOVE_WORLDPORT_ACK {}

