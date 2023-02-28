use crate::wrath::MovementInfo;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/cmsg/cmsg_move_set_fly.wowm:7`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/cmsg/cmsg_move_set_fly.wowm#L7):
/// ```text
/// cmsg CMSG_MOVE_SET_FLY = 0x0346 {
///     MovementInfo info;
/// }
/// ```
pub struct CMSG_MOVE_SET_FLY {
    pub info: MovementInfo,
}

impl crate::Message for CMSG_MOVE_SET_FLY {
    const OPCODE: u32 = 0x0346;

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
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0346, size: body_size as u32 });
        }

        // info: MovementInfo
        let info = MovementInfo::read(&mut r)?;

        Ok(Self {
            info,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_MOVE_SET_FLY {}

impl CMSG_MOVE_SET_FLY {
    pub(crate) fn size(&self) -> usize {
        self.info.size() // info: MovementInfo
    }
}

