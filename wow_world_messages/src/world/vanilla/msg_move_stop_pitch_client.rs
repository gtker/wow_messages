use std::io::{Read, Write};

use crate::vanilla::MovementInfo;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/msg/msg_move_stop_pitch.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/msg/msg_move_stop_pitch.wowm#L1):
/// ```text
/// cmsg MSG_MOVE_STOP_PITCH_Client = 0x00C1 {
///     MovementInfo info;
/// }
/// ```
pub struct MSG_MOVE_STOP_PITCH_Client {
    pub info: MovementInfo,
}

impl crate::private::Sealed for MSG_MOVE_STOP_PITCH_Client {}
impl crate::Message for MSG_MOVE_STOP_PITCH_Client {
    const OPCODE: u32 = 0x00c1;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // info: MovementInfo
        self.info.write_into_vec(&mut w)?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(28..=81).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x00C1, size: body_size });
        }

        // info: MovementInfo
        let info = MovementInfo::read(&mut r)?;

        Ok(Self {
            info,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for MSG_MOVE_STOP_PITCH_Client {}

impl MSG_MOVE_STOP_PITCH_Client {
    pub(crate) const fn size(&self) -> usize {
        self.info.size() // info: MovementInfo
    }
}

