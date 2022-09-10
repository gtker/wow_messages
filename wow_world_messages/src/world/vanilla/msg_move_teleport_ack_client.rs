use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Response to [`MSG_MOVE_TELEPORT_ACK_Server`](crate::world::vanilla::MSG_MOVE_TELEPORT_ACK_Server), at which point [`MSG_MOVE_TELEPORT_ACK_Server`](crate::world::vanilla::MSG_MOVE_TELEPORT_ACK_Server) should be sent to observing players.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/msg/msg_move_teleport_ack.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/msg/msg_move_teleport_ack.wowm#L3):
/// ```text
/// cmsg MSG_MOVE_TELEPORT_ACK_Client = 0x00C7 {
///     Guid guid;
///     u32 movement_counter;
///     u32 time_in_msecs;
/// }
/// ```
pub struct MSG_MOVE_TELEPORT_ACK_Client {
    pub guid: Guid,
    pub movement_counter: u32,
    pub time_in_msecs: u32,
}

impl crate::Message for MSG_MOVE_TELEPORT_ACK_Client {
    const OPCODE: u32 = 0x00c7;

    fn size_without_header(&self) -> u32 {
        16
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // movement_counter: u32
        w.write_all(&self.movement_counter.to_le_bytes())?;

        // time_in_msecs: u32
        w.write_all(&self.time_in_msecs.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 16 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // guid: Guid
        let guid = Guid::read(r)?;

        // movement_counter: u32
        let movement_counter = crate::util::read_u32_le(r)?;

        // time_in_msecs: u32
        let time_in_msecs = crate::util::read_u32_le(r)?;

        Ok(Self {
            guid,
            movement_counter,
            time_in_msecs,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for MSG_MOVE_TELEPORT_ACK_Client {}

