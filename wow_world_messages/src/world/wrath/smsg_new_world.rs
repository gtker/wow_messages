use crate::wrath::Vector3d;
use crate::wrath::Map;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_new_world.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_new_world.wowm#L1):
/// ```text
/// smsg SMSG_NEW_WORLD = 0x003E {
///     Map map;
///     Vector3d position;
///     f32 orientation;
/// }
/// ```
pub struct SMSG_NEW_WORLD {
    pub map: Map,
    pub position: Vector3d,
    pub orientation: f32,
}

impl crate::Message for SMSG_NEW_WORLD {
    const OPCODE: u32 = 0x003e;

    fn size_without_header(&self) -> u32 {
        20
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // map: Map
        w.write_all(&u32::from(self.map.as_int()).to_le_bytes())?;

        // position: Vector3d
        self.position.write_into_vec(&mut w)?;

        // orientation: f32
        w.write_all(&self.orientation.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 20 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x003E, size: body_size as u32 });
        }

        // map: Map
        let map: Map = crate::util::read_u32_le(r)?.try_into()?;

        // position: Vector3d
        let position = Vector3d::read(r)?;

        // orientation: f32
        let orientation = crate::util::read_f32_le(r)?;

        Ok(Self {
            map,
            position,
            orientation,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_NEW_WORLD {}

