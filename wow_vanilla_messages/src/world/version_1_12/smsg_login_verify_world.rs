use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::Map;
use crate::world::version_1_12::Vector3d;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/smsg_login_verify_world.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/smsg_login_verify_world.wowm#L3):
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

impl ServerMessage for SMSG_LOGIN_VERIFY_WORLD {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // map: Map
        w.write_all(&(self.map.as_int() as u32).to_le_bytes())?;

        // position: Vector3d
        self.position.write_into_vec(w)?;

        // orientation: f32
        w.write_all(&self.orientation.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0236;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        20
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
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

#[cfg(test)]
mod test {
    use super::SMSG_LOGIN_VERIFY_WORLD;
    use crate::world::version_1_12::Map;
    use crate::world::version_1_12::Vector3d;
    use super::*;
    use super::super::*;
    use crate::world::version_1_12::opcodes::ServerOpcodeMessage;
    use crate::{Guid, UpdateMask, UpdateContainer, UpdateItem, UpdateCorpse, UpdateGameObject, UpdateDynamicObject, UpdateUnit, UpdatePlayer};
    use crate::{ClientMessage, ServerMessage};

    const RAW0: [u8; 24] = [ 0x00, 0x16, 0x36, 0x02, 0x00, 0x00, 0x00, 0x00, 0xCD,
         0xD7, 0x0B, 0xC6, 0x35, 0x7E, 0x04, 0xC3, 0xF9, 0x0F, 0xA7, 0x42, 0x00,
         0x00, 0x00, 0x00, ];

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_login_verify_world.wowm` line 12.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn SMSG_LOGIN_VERIFY_WORLD0() {
        let expected = SMSG_LOGIN_VERIFY_WORLD {
            map: Map::EASTERN_KINGDOMS,
            position: Vector3d {
                x: -8949.95_f32,
                y: -132.493_f32,
                z: 83.5312_f32,
            },
            orientation: 0_f32,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_LOGIN_VERIFY_WORLD(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_LOGIN_VERIFY_WORLD, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.map, expected.map);
        assert_eq!(t.position, expected.position);
        assert_eq!(t.orientation, expected.orientation);

        assert_eq!(20 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_login_verify_world.wowm` line 12.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_SMSG_LOGIN_VERIFY_WORLD0() {
        let expected = SMSG_LOGIN_VERIFY_WORLD {
            map: Map::EASTERN_KINGDOMS,
            position: Vector3d {
                x: -8949.95_f32,
                y: -132.493_f32,
                z: 83.5312_f32,
            },
            orientation: 0_f32,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_LOGIN_VERIFY_WORLD(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_LOGIN_VERIFY_WORLD, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.map, expected.map);
        assert_eq!(t.position, expected.position);
        assert_eq!(t.orientation, expected.orientation);

        assert_eq!(20 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_login_verify_world.wowm` line 12.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_SMSG_LOGIN_VERIFY_WORLD0() {
        let expected = SMSG_LOGIN_VERIFY_WORLD {
            map: Map::EASTERN_KINGDOMS,
            position: Vector3d {
                x: -8949.95_f32,
                y: -132.493_f32,
                z: 83.5312_f32,
            },
            orientation: 0_f32,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_LOGIN_VERIFY_WORLD(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_LOGIN_VERIFY_WORLD, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.map, expected.map);
        assert_eq!(t.position, expected.position);
        assert_eq!(t.orientation, expected.orientation);

        assert_eq!(20 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}
