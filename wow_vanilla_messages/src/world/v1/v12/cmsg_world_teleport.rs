use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::Map;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMSG_WORLD_TELEPORT {
    pub time_in_msec: u64,
    pub map: Map,
    pub position_x: f32,
    pub position_y: f32,
    pub position_z: f32,
    pub orientation: f32,
}

impl CMSG_WORLD_TELEPORT {
    pub(crate) fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // time_in_msec: u64
        w.write_all(&self.time_in_msec.to_le_bytes())?;

        // map: Map
        w.write_all(&(self.map.as_int() as u32).to_le_bytes())?;

        // position_x: f32
        w.write_all(&self.position_x.to_le_bytes())?;

        // position_y: f32
        w.write_all(&self.position_y.to_le_bytes())?;

        // position_z: f32
        w.write_all(&self.position_z.to_le_bytes())?;

        // orientation: f32
        w.write_all(&self.orientation.to_le_bytes())?;

        Ok(())
    }
}

impl ClientMessage for CMSG_WORLD_TELEPORT {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // time_in_msec: u64
        w.write_all(&self.time_in_msec.to_le_bytes())?;

        // map: Map
        w.write_all(&(self.map.as_int() as u32).to_le_bytes())?;

        // position_x: f32
        w.write_all(&self.position_x.to_le_bytes())?;

        // position_y: f32
        w.write_all(&self.position_y.to_le_bytes())?;

        // position_z: f32
        w.write_all(&self.position_z.to_le_bytes())?;

        // orientation: f32
        w.write_all(&self.orientation.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0008;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        28
    }

    type Error = crate::errors::ParseError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // time_in_msec: u64
        let time_in_msec = crate::util::read_u64_le(r)?;

        // map: Map
        let map: Map = crate::util::read_u32_le(r)?.try_into()?;

        // position_x: f32
        let position_x = crate::util::read_f32_le(r)?;
        // position_y: f32
        let position_y = crate::util::read_f32_le(r)?;
        // position_z: f32
        let position_z = crate::util::read_f32_le(r)?;
        // orientation: f32
        let orientation = crate::util::read_f32_le(r)?;
        Ok(Self {
            time_in_msec,
            map,
            position_x,
            position_y,
            position_z,
            orientation,
        })
    }

}

#[cfg(test)]
mod test {
    use super::CMSG_WORLD_TELEPORT;
    use crate::world::v1::v12::Map;
    use super::*;
    use super::super::*;
    use crate::world::v1::v12::opcodes::ClientOpcodeMessage;
    use crate::{ClientMessage, ServerMessage};

    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMSG_WORLD_TELEPORT0() {
        let raw: Vec<u8> = vec![ 0x00, 0x20, 0x08, 0x00, 0x00, 0x00, 0xEF, 0xBE,
             0xAD, 0xDE, 0xDE, 0xCA, 0xFA, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
             0x80, 0x3F, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x40, 0x40, 0x00, 0x00,
             0x80, 0x40, ];

        let expected = CMSG_WORLD_TELEPORT {
            time_in_msec: 0xFACADEDEADBEEF,
            map: Map::KALIMDOR,
            position_x: 1.0,
            position_y: 2.0,
            position_z: 3.0,
            orientation: 4.0,
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&raw)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_WORLD_TELEPORT(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_WORLD_TELEPORT, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.time_in_msec, expected.time_in_msec);
        assert_eq!(t.map, expected.map);
        assert_eq!(t.position_x, expected.position_x);
        assert_eq!(t.position_y, expected.position_y);
        assert_eq!(t.position_z, expected.position_z);
        assert_eq!(t.orientation, expected.orientation);

        assert_eq!(28 + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMSG_WORLD_TELEPORT0() {
        let raw: Vec<u8> = vec![ 0x00, 0x20, 0x08, 0x00, 0x00, 0x00, 0xEF, 0xBE,
             0xAD, 0xDE, 0xDE, 0xCA, 0xFA, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
             0x80, 0x3F, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x40, 0x40, 0x00, 0x00,
             0x80, 0x40, ];

        let expected = CMSG_WORLD_TELEPORT {
            time_in_msec: 0xFACADEDEADBEEF,
            map: Map::KALIMDOR,
            position_x: 1.0,
            position_y: 2.0,
            position_z: 3.0,
            orientation: 4.0,
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&raw)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_WORLD_TELEPORT(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_WORLD_TELEPORT, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.time_in_msec, expected.time_in_msec);
        assert_eq!(t.map, expected.map);
        assert_eq!(t.position_x, expected.position_x);
        assert_eq!(t.position_y, expected.position_y);
        assert_eq!(t.position_z, expected.position_z);
        assert_eq!(t.orientation, expected.orientation);

        assert_eq!(28 + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMSG_WORLD_TELEPORT0() {
        let raw: Vec<u8> = vec![ 0x00, 0x20, 0x08, 0x00, 0x00, 0x00, 0xEF, 0xBE,
             0xAD, 0xDE, 0xDE, 0xCA, 0xFA, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
             0x80, 0x3F, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x40, 0x40, 0x00, 0x00,
             0x80, 0x40, ];

        let expected = CMSG_WORLD_TELEPORT {
            time_in_msec: 0xFACADEDEADBEEF,
            map: Map::KALIMDOR,
            position_x: 1.0,
            position_y: 2.0,
            position_z: 3.0,
            orientation: 4.0,
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&raw)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_WORLD_TELEPORT(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_WORLD_TELEPORT, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.time_in_msec, expected.time_in_msec);
        assert_eq!(t.map, expected.map);
        assert_eq!(t.position_x, expected.position_x);
        assert_eq!(t.position_y, expected.position_y);
        assert_eq!(t.position_z, expected.position_z);
        assert_eq!(t.orientation, expected.orientation);

        assert_eq!(28 + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, raw);
    }

}
