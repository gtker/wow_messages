use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::tbc::MovementInfo;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/msg/msg_move_set_flight_back_speed.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/msg/msg_move_set_flight_back_speed.wowm#L1):
/// ```text
/// msg MSG_MOVE_SET_FLIGHT_BACK_SPEED = 0x0380 {
///     PackedGuid player;
///     MovementInfo info;
///     f32 new_speed;
/// }
/// ```
pub struct MSG_MOVE_SET_FLIGHT_BACK_SPEED {
    pub player: Guid,
    pub info: MovementInfo,
    pub new_speed: f32,
}

impl crate::Message for MSG_MOVE_SET_FLIGHT_BACK_SPEED {
    const OPCODE: u32 = 0x0380;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // player: PackedGuid
        self.player.write_packed_guid_into_vec(w);

        // info: MovementInfo
        self.info.write_into_vec(w)?;

        // new_speed: f32
        w.write_all(&self.new_speed.to_le_bytes())?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(35..=95).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0380, size: body_size as u32 });
        }

        // player: PackedGuid
        let player = Guid::read_packed(r)?;

        // info: MovementInfo
        let info = MovementInfo::read(r)?;

        // new_speed: f32
        let new_speed = crate::util::read_f32_le(r)?;
        Ok(Self {
            player,
            info,
            new_speed,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for MSG_MOVE_SET_FLIGHT_BACK_SPEED {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for MSG_MOVE_SET_FLIGHT_BACK_SPEED {}

impl MSG_MOVE_SET_FLIGHT_BACK_SPEED {
    pub(crate) fn size(&self) -> usize {
        self.player.size() // player: Guid
        + self.info.size() // info: MovementInfo
        + 4 // new_speed: f32
    }
}

