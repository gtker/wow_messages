use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::MovementInfo;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/msg/msg_move_heartbeat.wowm:39`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/msg/msg_move_heartbeat.wowm#L39):
/// ```text
/// msg MSG_MOVE_HEARTBEAT = 0x00EE {
///     PackedGuid guid;
///     MovementInfo info;
/// }
/// ```
pub struct MSG_MOVE_HEARTBEAT {
    pub guid: Guid,
    pub info: MovementInfo,
}

impl crate::Message for MSG_MOVE_HEARTBEAT {
    const OPCODE: u32 = 0x00ee;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        self.guid.write_packed_guid_into_vec(&mut w)?;

        // info: MovementInfo
        self.info.write_into_vec(&mut w)?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(32..=93).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x00EE, size: body_size as u32 });
        }

        // guid: PackedGuid
        let guid = Guid::read_packed(&mut r)?;

        // info: MovementInfo
        let info = MovementInfo::read(&mut r)?;

        Ok(Self {
            guid,
            info,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for MSG_MOVE_HEARTBEAT {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for MSG_MOVE_HEARTBEAT {}

impl MSG_MOVE_HEARTBEAT {
    pub(crate) fn size(&self) -> usize {
        self.guid.size() // guid: PackedGuid
        + self.info.size() // info: MovementInfo
    }
}

