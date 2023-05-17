use std::io::{Read, Write};

use crate::tbc::MovementInfo;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/msg/msg_move_start_turn_left.wowm:7`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/msg/msg_move_start_turn_left.wowm#L7):
/// ```text
/// cmsg MSG_MOVE_START_TURN_LEFT_Client = 0x00BC {
///     MovementInfo info;
/// }
/// ```
pub struct MSG_MOVE_START_TURN_LEFT_Client {
    pub info: MovementInfo,
}

impl crate::private::Sealed for MSG_MOVE_START_TURN_LEFT_Client {}
impl crate::Message for MSG_MOVE_START_TURN_LEFT_Client {
    const OPCODE: u32 = 0x00bc;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // info: MovementInfo
        self.info.write_into_vec(&mut w)?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(29..=82).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x00BC, size: body_size });
        }

        // info: MovementInfo
        let info = MovementInfo::read(&mut r)?;

        Ok(Self {
            info,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for MSG_MOVE_START_TURN_LEFT_Client {}

impl MSG_MOVE_START_TURN_LEFT_Client {
    pub(crate) const fn size(&self) -> usize {
        self.info.size() // info: MovementInfo
    }
}

