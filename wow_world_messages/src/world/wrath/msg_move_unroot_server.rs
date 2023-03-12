use std::io::{Read, Write};
use crate::wrath::MovementInfo;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/msg/msg_move_unroot.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/msg/msg_move_unroot.wowm#L8):
/// ```text
/// smsg MSG_MOVE_UNROOT_Server = 0x00ED {
///     MovementInfo info;
/// }
/// ```
pub struct MSG_MOVE_UNROOT_Server {
    pub info: MovementInfo,
}

impl crate::Message for MSG_MOVE_UNROOT_Server {
    const OPCODE: u32 = 0x00ed;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // info: MovementInfo
        self.info.write_into_vec(&mut w)?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(30..=84).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x00ED, size: body_size as u32 });
        }

        // info: MovementInfo
        let info = MovementInfo::read(&mut r)?;

        Ok(Self {
            info,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for MSG_MOVE_UNROOT_Server {}

impl MSG_MOVE_UNROOT_Server {
    pub(crate) fn size(&self) -> usize {
        self.info.size() // info: MovementInfo
    }
}

