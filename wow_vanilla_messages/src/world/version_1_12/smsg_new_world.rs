use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::Vector3d;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_new_world.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_new_world.wowm#L3):
/// ```text
/// smsg SMSG_NEW_WORLD = 0x003E {
///     Vector3d position;
///     f32 orientation;
/// }
/// ```
pub struct SMSG_NEW_WORLD {
    pub position: Vector3d,
    pub orientation: f32,
}

impl ServerMessage for SMSG_NEW_WORLD {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // position: Vector3d
        self.position.write_into_vec(w)?;

        // orientation: f32
        w.write_all(&self.orientation.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x003e;

    fn server_size(&self) -> u16 {
        20
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 16 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // position: Vector3d
        let position = Vector3d::read(r)?;

        // orientation: f32
        let orientation = crate::util::read_f32_le(r)?;
        Ok(Self {
            position,
            orientation,
        })
    }

}

