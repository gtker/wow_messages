use std::convert::{TryFrom, TryInto};
use crate::world::wrath::MovementInfo;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/msg/msg_move_start_swim.wowm:13`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/msg/msg_move_start_swim.wowm#L13):
/// ```text
/// cmsg MSG_MOVE_START_SWIM_Client = 0x00CA {
///     MovementInfo info;
/// }
/// ```
pub struct MSG_MOVE_START_SWIM_Client {
    pub info: MovementInfo,
}

impl crate::Message for MSG_MOVE_START_SWIM_Client {
    const OPCODE: u32 = 0x00ca;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // info: MovementInfo
        self.info.write_into_vec(w)?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // info: MovementInfo
        let info = MovementInfo::read(r)?;

        Ok(Self {
            info,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for MSG_MOVE_START_SWIM_Client {}

impl MSG_MOVE_START_SWIM_Client {
    pub(crate) fn size(&self) -> usize {
        self.info.size() // info: MovementInfo
    }
}

