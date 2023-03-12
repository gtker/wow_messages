use std::io::{Read, Write};
use crate::Guid;
use crate::wrath::MovementInfo;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/cmsg/cmsg_move_set_collision_hgt_ack.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/cmsg/cmsg_move_set_collision_hgt_ack.wowm#L1):
/// ```text
/// cmsg CMSG_MOVE_SET_COLLISION_HGT_ACK = 0x0517 {
///     PackedGuid player;
///     u32 movement_counter;
///     MovementInfo info;
///     f32 new_height;
/// }
/// ```
pub struct CMSG_MOVE_SET_COLLISION_HGT_ACK {
    pub player: Guid,
    pub movement_counter: u32,
    pub info: MovementInfo,
    pub new_height: f32,
}

impl crate::Message for CMSG_MOVE_SET_COLLISION_HGT_ACK {
    const OPCODE: u32 = 0x0517;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // player: PackedGuid
        self.player.write_packed_guid_into_vec(&mut w)?;

        // movement_counter: u32
        w.write_all(&self.movement_counter.to_le_bytes())?;

        // info: MovementInfo
        self.info.write_into_vec(&mut w)?;

        // new_height: f32
        w.write_all(&self.new_height.to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(40..=101).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0517, size: body_size as u32 });
        }

        // player: PackedGuid
        let player = Guid::read_packed(&mut r)?;

        // movement_counter: u32
        let movement_counter = crate::util::read_u32_le(&mut r)?;

        // info: MovementInfo
        let info = MovementInfo::read(&mut r)?;

        // new_height: f32
        let new_height = crate::util::read_f32_le(&mut r)?;

        Ok(Self {
            player,
            movement_counter,
            info,
            new_height,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_MOVE_SET_COLLISION_HGT_ACK {}

impl CMSG_MOVE_SET_COLLISION_HGT_ACK {
    pub(crate) fn size(&self) -> usize {
        self.player.size() // player: PackedGuid
        + 4 // movement_counter: u32
        + self.info.size() // info: MovementInfo
        + 4 // new_height: f32
    }
}

