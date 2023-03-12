use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::MovementInfo;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/cmsg/cmsg_move_set_can_fly_ack.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/cmsg/cmsg_move_set_can_fly_ack.wowm#L1):
/// ```text
/// cmsg CMSG_MOVE_SET_CAN_FLY_ACK = 0x0345 {
///     Guid player;
///     u32 counter;
///     MovementInfo info;
///     Bool32 applied;
/// }
/// ```
pub struct CMSG_MOVE_SET_CAN_FLY_ACK {
    pub player: Guid,
    pub counter: u32,
    pub info: MovementInfo,
    pub applied: bool,
}

impl crate::Message for CMSG_MOVE_SET_CAN_FLY_ACK {
    const OPCODE: u32 = 0x0345;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // player: Guid
        w.write_all(&self.player.guid().to_le_bytes())?;

        // counter: u32
        w.write_all(&self.counter.to_le_bytes())?;

        // info: MovementInfo
        self.info.write_into_vec(&mut w)?;

        // applied: Bool32
        w.write_all(u32::from(self.applied).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(46..=100).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0345, size: body_size as u32 });
        }

        // player: Guid
        let player = Guid::read(&mut r)?;

        // counter: u32
        let counter = crate::util::read_u32_le(&mut r)?;

        // info: MovementInfo
        let info = MovementInfo::read(&mut r)?;

        // applied: Bool32
        let applied = crate::util::read_u32_le(&mut r)? != 0;

        Ok(Self {
            player,
            counter,
            info,
            applied,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_MOVE_SET_CAN_FLY_ACK {}

impl CMSG_MOVE_SET_CAN_FLY_ACK {
    pub(crate) fn size(&self) -> usize {
        8 // player: Guid
        + 4 // counter: u32
        + self.info.size() // info: MovementInfo
        + 4 // applied: Bool32
    }
}

