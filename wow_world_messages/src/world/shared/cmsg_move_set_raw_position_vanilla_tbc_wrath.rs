use crate:: {
};
use wow_world_base::shared::vector3d_vanilla_tbc_wrath::Vector3d;
use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
/// vmangos/mangoszero: write in client console: setrawpos x y z o. For now, it is implemented like worldport but on the same map. Consider using `MSG_MOVE_SET_RAW_POSITION_ACK`.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/cmsg/cmsg_move_set_raw_position.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/cmsg/cmsg_move_set_raw_position.wowm#L3):
/// ```text
/// cmsg CMSG_MOVE_SET_RAW_POSITION = 0x00E1 {
///     Vector3d position;
///     f32 orientation;
/// }
/// ```
pub struct CMSG_MOVE_SET_RAW_POSITION {
    pub position: Vector3d,
    pub orientation: f32,
}

impl crate::Message for CMSG_MOVE_SET_RAW_POSITION {
    const OPCODE: u32 = 0x00e1;

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
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x00E1, size: body_size as u32 });
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
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_MOVE_SET_RAW_POSITION {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_MOVE_SET_RAW_POSITION {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_MOVE_SET_RAW_POSITION {}

