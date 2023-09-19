use std::io::{Read, Write};

use crate::vanilla::{
    Map, Vector3d,
};

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

impl crate::private::Sealed for SMSG_NEW_WORLD {}
impl SMSG_NEW_WORLD {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 20 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // map: Map
        let map = crate::util::read_u32_le(&mut r)?.try_into()?;

        // position: Vector3d
        let position = crate::util::vanilla_tbc_wrath_vector3d_read(&mut r)?;

        // orientation: f32
        let orientation = crate::util::read_f32_le(&mut r)?;

        Ok(Self {
            map,
            position,
            orientation,
        })
    }

}

impl crate::Message for SMSG_NEW_WORLD {
    const OPCODE: u32 = 0x003e;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_NEW_WORLD"
    }

    fn size_without_header(&self) -> u32 {
        20
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // map: Map
        w.write_all(&(self.map.as_int().to_le_bytes()))?;

        // position: Vector3d
crate::util::vanilla_tbc_wrath_vector3d_write_into_vec(&self.position, &mut w)?;

        // orientation: f32
        w.write_all(&self.orientation.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(62, "SMSG_NEW_WORLD", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_NEW_WORLD {}

#[cfg(test)]
mod test {
    #![allow(clippy::missing_const_for_fn)]
    use super::SMSG_NEW_WORLD;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ServerOpcodeMessage;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    fn assert(t: &SMSG_NEW_WORLD, expected: &SMSG_NEW_WORLD) {
        assert_eq!(t.map, expected.map);
        assert_eq!(t.position, expected.position);
        assert_eq!(t.orientation, expected.orientation);
    }

    const RAW0: [u8; 24] = [ 0x00, 0x16, 0x3E, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00,
         0xA0, 0xBA, 0x44, 0x00, 0xEC, 0x89, 0xC5, 0xCD, 0xCC, 0xB8, 0x41, 0xCD,
         0xCC, 0x4C, 0x3E, ];

    pub(crate) fn expected0() -> SMSG_NEW_WORLD {
        SMSG_NEW_WORLD {
            map: Map::Kalimdor,
            position: Vector3d {
                x: 1493_f32,
                y: -4413.5_f32,
                z: 23.1_f32,
            },
            orientation: 0.2_f32,
        }

    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_new_world.wowm` line 10.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_new_world0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_NEW_WORLD(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_NEW_WORLD, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(20 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_new_world.wowm` line 10.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_new_world0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_NEW_WORLD(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_NEW_WORLD, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(20 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_new_world.wowm` line 10.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_new_world0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_NEW_WORLD(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_NEW_WORLD, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(20 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

