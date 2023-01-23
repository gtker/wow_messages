use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::tbc::MovementInfo;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/msg_move_knock_back.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/msg_move_knock_back.wowm#L1):
/// ```text
/// smsg MSG_MOVE_KNOCK_BACK_Server = 0x00F1 {
///     PackedGuid player;
///     MovementInfo info;
///     f32 sin_angle;
///     f32 cos_angle;
///     f32 x_y_speed;
///     f32 velocity;
/// }
/// ```
pub struct MSG_MOVE_KNOCK_BACK_Server {
    pub player: Guid,
    pub info: MovementInfo,
    pub sin_angle: f32,
    pub cos_angle: f32,
    pub x_y_speed: f32,
    pub velocity: f32,
}

impl crate::Message for MSG_MOVE_KNOCK_BACK_Server {
    const OPCODE: u32 = 0x00f1;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // player: PackedGuid
        self.player.write_packed_guid_into_vec(w);

        // info: MovementInfo
        self.info.write_into_vec(w)?;

        // sin_angle: f32
        w.write_all(&self.sin_angle.to_le_bytes())?;

        // cos_angle: f32
        w.write_all(&self.cos_angle.to_le_bytes())?;

        // x_y_speed: f32
        w.write_all(&self.x_y_speed.to_le_bytes())?;

        // velocity: f32
        w.write_all(&self.velocity.to_le_bytes())?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(47..=107).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x00F1, size: body_size as u32 });
        }

        // player: PackedGuid
        let player = Guid::read_packed(r)?;

        // info: MovementInfo
        let info = MovementInfo::read(r)?;

        // sin_angle: f32
        let sin_angle = crate::util::read_f32_le(r)?;
        // cos_angle: f32
        let cos_angle = crate::util::read_f32_le(r)?;
        // x_y_speed: f32
        let x_y_speed = crate::util::read_f32_le(r)?;
        // velocity: f32
        let velocity = crate::util::read_f32_le(r)?;
        Ok(Self {
            player,
            info,
            sin_angle,
            cos_angle,
            x_y_speed,
            velocity,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for MSG_MOVE_KNOCK_BACK_Server {}

impl MSG_MOVE_KNOCK_BACK_Server {
    pub(crate) fn size(&self) -> usize {
        self.player.size() // player: Guid
        + self.info.size() // info: MovementInfo
        + 4 // sin_angle: f32
        + 4 // cos_angle: f32
        + 4 // x_y_speed: f32
        + 4 // velocity: f32
    }
}

