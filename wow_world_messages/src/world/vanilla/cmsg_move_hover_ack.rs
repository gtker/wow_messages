use std::io::{Read, Write};

use crate::Guid;
use crate::vanilla::MovementInfo;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/cmsg/cmsg_move_hover_ack.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/cmsg/cmsg_move_hover_ack.wowm#L3):
/// ```text
/// cmsg CMSG_MOVE_HOVER_ACK = 0x00F6 {
///     Guid guid;
///     u32 counter;
///     MovementInfo info;
///     u32 is_applied;
/// }
/// ```
pub struct CMSG_MOVE_HOVER_ACK {
    pub guid: Guid,
    pub counter: u32,
    pub info: MovementInfo,
    pub is_applied: u32,
}

impl crate::Message for CMSG_MOVE_HOVER_ACK {
    const OPCODE: u32 = 0x00f6;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // counter: u32
        w.write_all(&self.counter.to_le_bytes())?;

        // info: MovementInfo
        self.info.write_into_vec(&mut w)?;

        // is_applied: u32
        w.write_all(&self.is_applied.to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(44..=97).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x00F6, size: body_size as u32 });
        }

        // guid: Guid
        let guid = Guid::read(&mut r)?;

        // counter: u32
        let counter = crate::util::read_u32_le(&mut r)?;

        // info: MovementInfo
        let info = MovementInfo::read(&mut r)?;

        // is_applied: u32
        let is_applied = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            guid,
            counter,
            info,
            is_applied,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_MOVE_HOVER_ACK {}

impl CMSG_MOVE_HOVER_ACK {
    pub(crate) const fn size(&self) -> usize {
        8 // guid: Guid
        + 4 // counter: u32
        + self.info.size() // info: MovementInfo
        + 4 // is_applied: u32
    }
}

