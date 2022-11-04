use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::wrath::MovementInfo;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Default)]
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

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // counter: u32
        w.write_all(&self.counter.to_le_bytes())?;

        // info: MovementInfo
        self.info.write_into_vec(w)?;

        // is_applied: u32
        w.write_all(&self.is_applied.to_le_bytes())?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(46..=100).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x00F6, size: body_size as u32 });
        }

        // guid: Guid
        let guid = Guid::read(r)?;

        // counter: u32
        let counter = crate::util::read_u32_le(r)?;

        // info: MovementInfo
        let info = MovementInfo::read(r)?;

        // is_applied: u32
        let is_applied = crate::util::read_u32_le(r)?;

        Ok(Self {
            guid,
            counter,
            info,
            is_applied,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_MOVE_HOVER_ACK {}

impl CMSG_MOVE_HOVER_ACK {
    pub(crate) fn size(&self) -> usize {
        8 // guid: Guid
        + 4 // counter: u32
        + self.info.size() // info: MovementInfo
        + 4 // is_applied: u32
    }
}

