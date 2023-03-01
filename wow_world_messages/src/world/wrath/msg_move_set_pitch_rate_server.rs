use crate:: {
    Guid,
};
use crate::wrath::MovementInfo;
use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/msg/msg_move_set_pitch_rate.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/msg/msg_move_set_pitch_rate.wowm#L1):
/// ```text
/// smsg MSG_MOVE_SET_PITCH_RATE_Server = 0x045B {
///     PackedGuid player;
///     MovementInfo info;
///     f32 new_speed;
/// }
/// ```
pub struct MSG_MOVE_SET_PITCH_RATE_Server {
    pub player: Guid,
    pub info: MovementInfo,
    pub new_speed: f32,
}

impl crate::Message for MSG_MOVE_SET_PITCH_RATE_Server {
    const OPCODE: u32 = 0x045b;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // player: PackedGuid
        self.player.write_packed_guid_into_vec(&mut w)?;

        // info: MovementInfo
        self.info.write_into_vec(&mut w)?;

        // new_speed: f32
        w.write_all(&self.new_speed.to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(36..=97).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x045B, size: body_size as u32 });
        }

        // player: PackedGuid
        let player = Guid::read_packed(&mut r)?;

        // info: MovementInfo
        let info = MovementInfo::read(&mut r)?;

        // new_speed: f32
        let new_speed = crate::util::read_f32_le(&mut r)?;

        Ok(Self {
            player,
            info,
            new_speed,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for MSG_MOVE_SET_PITCH_RATE_Server {}

impl MSG_MOVE_SET_PITCH_RATE_Server {
    pub(crate) fn size(&self) -> usize {
        self.player.size() // player: Guid
        + self.info.size() // info: MovementInfo
        + 4 // new_speed: f32
    }
}

