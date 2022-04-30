use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{Map, MapError};
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

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

impl ClientMessageWrite for CMSG_WORLD_TELEPORT {}

impl MessageBody for CMSG_WORLD_TELEPORT {
    const OPCODE: u16 = 0x0008;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = CMSG_WORLD_TELEPORTError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // time_in_msec: u64
        let time_in_msec = crate::util::read_u64_le(r)?;

        // map: Map
        let map = Map::read(r)?;

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

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // time_in_msec: u64
        w.write_all(&self.time_in_msec.to_le_bytes())?;

        // map: Map
        self.map.write(w)?;

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

impl ConstantSized for CMSG_WORLD_TELEPORT {}

impl MaximumPossibleSized for CMSG_WORLD_TELEPORT {
    fn maximum_possible_size() -> usize {
        8 // time_in_msec: u64
        + Map::size() // map: Map
        + 4 // position_x: f32
        + 4 // position_y: f32
        + 4 // position_z: f32
        + 4 // orientation: f32
    }
}

#[derive(Debug)]
pub enum CMSG_WORLD_TELEPORTError {
    Io(std::io::Error),
    Map(MapError),
}

impl std::error::Error for CMSG_WORLD_TELEPORTError {}
impl std::fmt::Display for CMSG_WORLD_TELEPORTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::Map(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_WORLD_TELEPORTError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<MapError> for CMSG_WORLD_TELEPORTError {
    fn from(e: MapError) -> Self {
        Self::Map(e)
    }
}

#[cfg(test)]
mod test {
    use crate::ReadableAndWritable;
    use std::io::Cursor;
    use super::CMSG_WORLD_TELEPORT;
    use crate::ConstantSized;
    use crate::world::v1::v12::Map;
    use super::*;
    use super::super::*;
    use crate::world::v1::v12::opcodes::ClientOpcodeMessage;
    use crate::{MessageBody, ClientMessageWrite, ServerMessageWrite, OpcodeMessage};

    #[test]
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
        let t = ClientOpcodeMessage::read_unencrypted(&mut Cursor::new(&raw)).unwrap();
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

        assert_eq!(CMSG_WORLD_TELEPORT::size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write_unencrypted_client(&mut Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

}
