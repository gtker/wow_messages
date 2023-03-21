use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::MovementInfo;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Can be response to [`CMSG_TELEPORT_TO_UNIT`](crate::vanilla::CMSG_TELEPORT_TO_UNIT).
/// Can also be a response to [`MSG_MOVE_TELEPORT_ACK_Client`](crate::vanilla::MSG_MOVE_TELEPORT_ACK_Client) after being sent.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/msg/msg_move_teleport_ack.wowm:10`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/msg/msg_move_teleport_ack.wowm#L10):
/// ```text
/// smsg MSG_MOVE_TELEPORT_ACK_Server = 0x00C7 {
///     PackedGuid guid;
///     u32 movement_counter;
///     MovementInfo info;
/// }
/// ```
pub struct MSG_MOVE_TELEPORT_ACK_Server {
    pub guid: Guid,
    pub movement_counter: u32,
    pub info: MovementInfo,
}

impl crate::Message for MSG_MOVE_TELEPORT_ACK_Server {
    const OPCODE: u32 = 0x00c7;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        self.guid.write_packed_guid_into_vec(&mut w)?;

        // movement_counter: u32
        w.write_all(&self.movement_counter.to_le_bytes())?;

        // info: MovementInfo
        self.info.write_into_vec(&mut w)?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(36..=101).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x00C7, size: body_size as u32 });
        }

        // guid: PackedGuid
        let guid = Guid::read_packed(&mut r)?;

        // movement_counter: u32
        let movement_counter = crate::util::read_u32_le(&mut r)?;

        // info: MovementInfo
        let info = MovementInfo::read(&mut r)?;

        Ok(Self {
            guid,
            movement_counter,
            info,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for MSG_MOVE_TELEPORT_ACK_Server {}

impl MSG_MOVE_TELEPORT_ACK_Server {
    pub(crate) const fn size(&self) -> usize {
        self.guid.size() // guid: PackedGuid
        + 4 // movement_counter: u32
        + self.info.size() // info: MovementInfo
    }
}

