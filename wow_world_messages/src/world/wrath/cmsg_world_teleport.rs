use std::io::{Read, Write};

use crate::wrath::{
    Map, Vector3d,
};
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
/// Sent when using the `worldport` console command.
///
/// The 3.3.5 client includes some extra padding.
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/cmsg/cmsg_world_teleport_3_3_5.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/cmsg/cmsg_world_teleport_3_3_5.wowm#L3):
/// ```text
/// cmsg CMSG_WORLD_TELEPORT = 0x0008 {
///     Milliseconds time;
///     Map map;
///     u64 unknown;
///     Vector3d position;
///     f32 orientation;
/// }
/// ```
pub struct CMSG_WORLD_TELEPORT {
    pub time: Duration,
    pub map: Map,
    pub unknown: u64,
    pub position: Vector3d,
    pub orientation: f32,
}

impl crate::private::Sealed for CMSG_WORLD_TELEPORT {}
impl CMSG_WORLD_TELEPORT {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 32 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // time: Milliseconds
        let time = Duration::from_millis(crate::util::read_u32_le(&mut r)?.into());

        // map: Map
        let map = crate::util::read_u32_le(&mut r)?.try_into()?;

        // unknown: u64
        let unknown = crate::util::read_u64_le(&mut r)?;

        // position: Vector3d
        let position = Vector3d::read(&mut r)?;

        // orientation: f32
        let orientation = crate::util::read_f32_le(&mut r)?;

        Ok(Self {
            time,
            map,
            unknown,
            position,
            orientation,
        })
    }

}

impl crate::Message for CMSG_WORLD_TELEPORT {
    const OPCODE: u32 = 0x0008;

    fn size_without_header(&self) -> u32 {
        32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // time: Milliseconds
        w.write_all((self.time.as_millis() as u32).to_le_bytes().as_slice())?;

        // map: Map
        w.write_all(&(self.map.as_int().to_le_bytes()))?;

        // unknown: u64
        w.write_all(&self.unknown.to_le_bytes())?;

        // position: Vector3d
        self.position.write_into_vec(&mut w)?;

        // orientation: f32
        w.write_all(&self.orientation.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(8, "CMSG_WORLD_TELEPORT", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_WORLD_TELEPORT {}

#[cfg(test)]
mod test {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMSG_WORLD_TELEPORT;
    use super::*;
    use super::super::*;
    use crate::wrath::opcodes::ClientOpcodeMessage;
    use crate::wrath::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    fn assert(t: &CMSG_WORLD_TELEPORT, expected: &CMSG_WORLD_TELEPORT) {
        assert_eq!(t.time, expected.time);
        assert_eq!(t.map, expected.map);
        assert_eq!(t.unknown, expected.unknown);
        assert_eq!(t.position, expected.position);
        assert_eq!(t.orientation, expected.orientation);
    }

    const RAW0: [u8; 38] = [ 0x00, 0x24, 0x08, 0x00, 0x00, 0x00, 0xEF, 0xBE, 0xAD,
         0xDE, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x80, 0x3F, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x40,
         0x40, 0x00, 0x00, 0x80, 0x40, ];

    pub(crate) fn expected0() -> CMSG_WORLD_TELEPORT {
        CMSG_WORLD_TELEPORT {
            time: Duration::from_millis(0xDEADBEEF),
            map: Map::Kalimdor,
            unknown: 0x0,
            position: Vector3d {
                x: 1_f32,
                y: 2_f32,
                z: 3_f32,
            },
            orientation: 4_f32,
        }

    }

    // Generated from `wow_message_parser/wowm/world/movement/cmsg/cmsg_world_teleport_3_3_5.wowm` line 14.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_world_teleport0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_WORLD_TELEPORT(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_WORLD_TELEPORT, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(32 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/cmsg/cmsg_world_teleport_3_3_5.wowm` line 14.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_world_teleport0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_WORLD_TELEPORT(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_WORLD_TELEPORT, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(32 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/cmsg/cmsg_world_teleport_3_3_5.wowm` line 14.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_world_teleport0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_WORLD_TELEPORT(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_WORLD_TELEPORT, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(32 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    const RAW1: [u8; 38] = [ 0x00, 0x24, 0x08, 0x00, 0x00, 0x00, 0x9A, 0x3D, 0x09,
         0x02, 0xD5, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0xE2, 0x43, 0x00, 0xB0, 0xC9, 0x45, 0x00, 0x80, 0x1E,
         0x45, 0xDB, 0x0F, 0x49, 0x40, ];

    pub(crate) fn expected1() -> CMSG_WORLD_TELEPORT {
        CMSG_WORLD_TELEPORT {
            time: Duration::from_millis(0x2093D9A),
            map: Map::BlackwingLair,
            unknown: 0x0,
            position: Vector3d {
                x: 452_f32,
                y: 6454_f32,
                z: 2536_f32,
            },
            orientation: 3.1415927_f32,
        }

    }

    // Generated from `wow_message_parser/wowm/world/movement/cmsg/cmsg_world_teleport_3_3_5.wowm` line 37.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_world_teleport1() {
        let expected = expected1();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW1)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_WORLD_TELEPORT(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_WORLD_TELEPORT, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(32 + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/world/movement/cmsg/cmsg_world_teleport_3_3_5.wowm` line 37.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_world_teleport1() {
        let expected = expected1();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_WORLD_TELEPORT(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_WORLD_TELEPORT, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(32 + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/world/movement/cmsg/cmsg_world_teleport_3_3_5.wowm` line 37.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_world_teleport1() {
        let expected = expected1();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_WORLD_TELEPORT(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_WORLD_TELEPORT, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(32 + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

}

