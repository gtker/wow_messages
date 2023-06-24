use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::{
    MovementFlags, MovementInfo, TransportInfo, Vector3d,
};

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

impl crate::private::Sealed for MSG_MOVE_SET_FLIGHT_BACK_SPEED {}
impl crate::Message for MSG_MOVE_SET_FLIGHT_BACK_SPEED {
    const OPCODE: u32 = 0x0380;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        panic!("MSG types not supported");
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // player: PackedGuid
        crate::util::write_packed_guid(&self.player, &mut w)?;

        // info: MovementInfo
        self.info.write_into_vec(&mut w)?;

        // new_speed: f32
        w.write_all(&self.new_speed.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(36..=101).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0380, size: body_size });
        }

        // player: PackedGuid
        let player = crate::util::read_packed_guid(&mut r)?;

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
impl crate::wrath::ClientMessage for MSG_MOVE_SET_FLIGHT_BACK_SPEED {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for MSG_MOVE_SET_FLIGHT_BACK_SPEED {}

impl MSG_MOVE_SET_FLIGHT_BACK_SPEED {
    pub(crate) const fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.player) // player: PackedGuid
        + self.info.size() // info: MovementInfo
        + 4 // new_speed: f32
    }
}

