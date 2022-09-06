use std::convert::{TryFrom, TryInto};
use crate::world::wrath::Map;
use crate::world::wrath::Vector3d;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Copy, Default)]
/// Message to the client that is has successfully logged into the world and that it should load the map and coordinates.
///
/// The positions and orientations do not matter since they can be overwritten in the `SMSG_UPDATE_OBJECT`, but the map determines which map the client loads and this is not changeable in `SMSG_UPDATE_OBJECT`.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/smsg_login_verify_world.wowm:42`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/smsg_login_verify_world.wowm#L42):
/// ```text
/// smsg SMSG_LOGIN_VERIFY_WORLD = 0x0236 {
///     Map map;
///     Vector3d position;
///     f32 orientation;
/// }
/// ```
pub struct SMSG_LOGIN_VERIFY_WORLD {
    pub map: Map,
    pub position: Vector3d,
    pub orientation: f32,
}

impl crate::Message for SMSG_LOGIN_VERIFY_WORLD {
    const OPCODE: u32 = 0x0236;

    fn size_without_header(&self) -> u32 {
        20
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // map: Map
        w.write_all(&(self.map.as_int() as u32).to_le_bytes())?;

        // position: Vector3d
        self.position.write_into_vec(w)?;

        // orientation: f32
        w.write_all(&self.orientation.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 20 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
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
impl crate::world::wrath::ServerMessage for SMSG_LOGIN_VERIFY_WORLD {}

