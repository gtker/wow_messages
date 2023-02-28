use wow_world_base::shared::vector3d_vanilla_tbc_wrath::Vector3d;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
/// There does not appear to be a CMSG version of this MSG.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/msg/msg_move_teleport_cheat.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/msg/msg_move_teleport_cheat.wowm#L1):
/// ```text
/// smsg MSG_MOVE_TELEPORT_CHEAT_Server = 0x00C6 {
///     Vector3d position;
///     f32 orientation;
/// }
/// ```
pub struct MSG_MOVE_TELEPORT_CHEAT_Server {
    pub position: Vector3d,
    pub orientation: f32,
}

impl crate::Message for MSG_MOVE_TELEPORT_CHEAT_Server {
    const OPCODE: u32 = 0x00c6;

    fn size_without_header(&self) -> u32 {
        16
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // position: Vector3d
        self.position.write_into_vec(&mut w)?;

        // orientation: f32
        w.write_all(&self.orientation.to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 16 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x00C6, size: body_size as u32 });
        }

        // position: Vector3d
        let position = Vector3d::read(&mut r)?;

        // orientation: f32
        let orientation = crate::util::read_f32_le(&mut r)?;

        Ok(Self {
            position,
            orientation,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for MSG_MOVE_TELEPORT_CHEAT_Server {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for MSG_MOVE_TELEPORT_CHEAT_Server {}

