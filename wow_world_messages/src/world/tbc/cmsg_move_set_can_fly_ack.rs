use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::tbc::MovementInfo;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/cmsg_move_set_can_fly_ack.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/cmsg_move_set_can_fly_ack.wowm#L1):
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

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // player: Guid
        w.write_all(&self.player.guid().to_le_bytes())?;

        // counter: u32
        w.write_all(&self.counter.to_le_bytes())?;

        // info: MovementInfo
        self.info.write_into_vec(w)?;

        // applied: Bool32
        w.write_all(u32::from(self.applied).to_le_bytes().as_slice())?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(45..=98).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0345, size: body_size as u32 });
        }

        // player: Guid
        let player = Guid::read(r)?;

        // counter: u32
        let counter = crate::util::read_u32_le(r)?;

        // info: MovementInfo
        let info = MovementInfo::read(r)?;

        // applied: Bool32
        let applied = crate::util::read_u32_le(r)? != 0;
        Ok(Self {
            player,
            counter,
            info,
            applied,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ClientMessage for CMSG_MOVE_SET_CAN_FLY_ACK {}

impl CMSG_MOVE_SET_CAN_FLY_ACK {
    pub(crate) fn size(&self) -> usize {
        8 // player: Guid
        + 4 // counter: u32
        + self.info.size() // info: MovementInfo
        + 4 // applied: Bool32
    }
}

