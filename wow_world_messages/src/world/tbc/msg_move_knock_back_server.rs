use std::io::{Read, Write};

use crate::Guid;
use crate::tbc::MovementInfo;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/msg/msg_move_knock_back.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/msg/msg_move_knock_back.wowm#L1):
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

impl crate::private::Sealed for MSG_MOVE_KNOCK_BACK_Server {}
impl crate::Message for MSG_MOVE_KNOCK_BACK_Server {
    const OPCODE: u32 = 0x00f1;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // player: PackedGuid
        self.player.write_packed_guid_into_vec(&mut w)?;

        // info: MovementInfo
        self.info.write_into_vec(&mut w)?;

        // sin_angle: f32
        w.write_all(&self.sin_angle.to_le_bytes())?;

        // cos_angle: f32
        w.write_all(&self.cos_angle.to_le_bytes())?;

        // x_y_speed: f32
        w.write_all(&self.x_y_speed.to_le_bytes())?;

        // velocity: f32
        w.write_all(&self.velocity.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(47..=107).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x00F1, size: body_size });
        }

        // player: PackedGuid
        let player = Guid::read_packed(&mut r)?;

        // info: MovementInfo
        let info = MovementInfo::read(&mut r)?;

        // sin_angle: f32
        let sin_angle = crate::util::read_f32_le(&mut r)?;

        // cos_angle: f32
        let cos_angle = crate::util::read_f32_le(&mut r)?;

        // x_y_speed: f32
        let x_y_speed = crate::util::read_f32_le(&mut r)?;

        // velocity: f32
        let velocity = crate::util::read_f32_le(&mut r)?;

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
impl crate::tbc::ServerMessage for MSG_MOVE_KNOCK_BACK_Server {}

impl MSG_MOVE_KNOCK_BACK_Server {
    pub(crate) const fn size(&self) -> usize {
        self.player.size() // player: PackedGuid
        + self.info.size() // info: MovementInfo
        + 4 // sin_angle: f32
        + 4 // cos_angle: f32
        + 4 // x_y_speed: f32
        + 4 // velocity: f32
    }
}

