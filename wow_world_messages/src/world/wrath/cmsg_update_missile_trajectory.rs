use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::Vector3d;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/cmsg_update_missile_trajectory.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/cmsg_update_missile_trajectory.wowm#L1):
/// ```text
/// cmsg CMSG_UPDATE_MISSILE_TRAJECTORY = 0x0462 {
///     Guid guid;
///     u32 spell;
///     f32 elevation;
///     f32 speed;
///     Vector3d position;
///     Vector3d target;
/// }
/// ```
pub struct CMSG_UPDATE_MISSILE_TRAJECTORY {
    pub guid: Guid,
    pub spell: u32,
    pub elevation: f32,
    pub speed: f32,
    pub position: Vector3d,
    pub target: Vector3d,
}

impl crate::private::Sealed for CMSG_UPDATE_MISSILE_TRAJECTORY {}
impl crate::Message for CMSG_UPDATE_MISSILE_TRAJECTORY {
    const OPCODE: u32 = 0x0462;

    fn size_without_header(&self) -> u32 {
        44
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // elevation: f32
        w.write_all(&self.elevation.to_le_bytes())?;

        // speed: f32
        w.write_all(&self.speed.to_le_bytes())?;

        // position: Vector3d
        self.position.write_into_vec(&mut w)?;

        // target: Vector3d
        self.target.write_into_vec(&mut w)?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 44 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0462, size: body_size as u32 });
        }

        // guid: Guid
        let guid = Guid::read(&mut r)?;

        // spell: u32
        let spell = crate::util::read_u32_le(&mut r)?;

        // elevation: f32
        let elevation = crate::util::read_f32_le(&mut r)?;

        // speed: f32
        let speed = crate::util::read_f32_le(&mut r)?;

        // position: Vector3d
        let position = Vector3d::read(&mut r)?;

        // target: Vector3d
        let target = Vector3d::read(&mut r)?;

        Ok(Self {
            guid,
            spell,
            elevation,
            speed,
            position,
            target,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_UPDATE_MISSILE_TRAJECTORY {}

