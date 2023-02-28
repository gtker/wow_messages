use crate::tbc::MovementInfo;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/msg/msg_move_stop_turn.wowm:7`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/msg/msg_move_stop_turn.wowm#L7):
/// ```text
/// cmsg MSG_MOVE_STOP_TURN_Client = 0x00BE {
///     MovementInfo info;
/// }
/// ```
pub struct MSG_MOVE_STOP_TURN_Client {
    pub info: MovementInfo,
}

impl crate::Message for MSG_MOVE_STOP_TURN_Client {
    const OPCODE: u32 = 0x00be;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // info: MovementInfo
        self.info.write_into_vec(&mut w)?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(29..=82).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x00BE, size: body_size as u32 });
        }

        // info: MovementInfo
        let info = MovementInfo::read(r)?;

        Ok(Self {
            info,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for MSG_MOVE_STOP_TURN_Client {}

impl MSG_MOVE_STOP_TURN_Client {
    pub(crate) fn size(&self) -> usize {
        self.info.size() // info: MovementInfo
    }
}

