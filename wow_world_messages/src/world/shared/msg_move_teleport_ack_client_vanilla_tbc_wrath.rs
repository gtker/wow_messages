use std::io::{Read, Write};
use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Response to [`MSG_MOVE_TELEPORT_ACK_Server`](crate::vanilla::MSG_MOVE_TELEPORT_ACK_Server), at which point [`MSG_MOVE_TELEPORT_ACK_Server`](crate::vanilla::MSG_MOVE_TELEPORT_ACK_Server) should be sent to observing players.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/msg/msg_move_teleport_ack.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/msg/msg_move_teleport_ack.wowm#L1):
/// ```text
/// cmsg MSG_MOVE_TELEPORT_ACK_Client = 0x00C7 {
///     PackedGuid guid;
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
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        self.guid.write_packed_guid_into_vec(&mut w)?;

        // movement_counter: u32
        w.write_all(&self.movement_counter.to_le_bytes())?;

        // time_in_msecs: u32
        w.write_all(&self.time_in_msecs.to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(10..=17).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x00C7, size: body_size as u32 });
        }

        // guid: PackedGuid
        let guid = Guid::read_packed(&mut r)?;

        // movement_counter: u32
        let movement_counter = crate::util::read_u32_le(&mut r)?;

        // time_in_msecs: u32
        let time_in_msecs = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            guid,
            movement_counter,
            time_in_msecs,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for MSG_MOVE_TELEPORT_ACK_Client {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for MSG_MOVE_TELEPORT_ACK_Client {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for MSG_MOVE_TELEPORT_ACK_Client {}

impl MSG_MOVE_TELEPORT_ACK_Client {
    pub(crate) fn size(&self) -> usize {
        self.guid.size() // guid: PackedGuid
        + 4 // movement_counter: u32
        + 4 // time_in_msecs: u32
    }
}

