use std::io::{Read, Write};

use crate::vanilla::{
    Map, Vector3d,
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
/// Message to the client that is has successfully logged into the world and that it should load the map and coordinates.
///
/// The positions and orientations do not matter since they can be overwritten in the [`SMSG_UPDATE_OBJECT`](crate::vanilla::SMSG_UPDATE_OBJECT), but the map determines which map the client loads and this is not changeable in [`SMSG_UPDATE_OBJECT`](crate::vanilla::SMSG_UPDATE_OBJECT).
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/smsg_login_verify_world.wowm:2`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/smsg_login_verify_world.wowm#L2):
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

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // map: Map
        w.write_all(&u32::from(self.map.as_int()).to_le_bytes())?;

        // position: Vector3d
        self.position.write_into_vec(&mut w)?;

        // orientation: f32
        w.write_all(&self.orientation.to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 20 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0236, size: body_size as u32 });
        }

        // map: Map
        let map: Map = crate::util::read_u32_le(&mut r)?.try_into()?;

        // position: Vector3d
        let position = Vector3d::read(&mut r)?;

        // orientation: f32
        let orientation = crate::util::read_f32_le(&mut r)?;

        Ok(Self {
            map,
            position,
            orientation,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_LOGIN_VERIFY_WORLD {}

#[cfg(test)]
mod test {
    use super::SMSG_LOGIN_VERIFY_WORLD;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ServerOpcodeMessage;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const RAW0: [u8; 24] = [ 0x00, 0x16, 0x36, 0x02, 0x00, 0x00, 0x00, 0x00, 0xCD,
         0xD7, 0x0B, 0xC6, 0x35, 0x7E, 0x04, 0xC3, 0xF9, 0x0F, 0xA7, 0x42, 0x00,
         0x00, 0x00, 0x00, ];

    pub(crate) fn expected0() -> SMSG_LOGIN_VERIFY_WORLD {
        SMSG_LOGIN_VERIFY_WORLD {
            map: Map::EasternKingdoms,
            position: Vector3d {
                x: -8949.95_f32,
                y: -132.493_f32,
                z: 83.5312_f32,
            },
            orientation: 0_f32,
        }

    }

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_login_verify_world.wowm` line 12.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn SMSG_LOGIN_VERIFY_WORLD0() {
        let expected = expected0();
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
        let expected = expected0();
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
        let expected = expected0();
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

