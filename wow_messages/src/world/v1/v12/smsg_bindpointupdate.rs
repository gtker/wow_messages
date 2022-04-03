use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{Area, AreaError};
use crate::world::v1::v12::{Map, MapError};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/new.wowm:300`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/new.wowm#L300):
/// ```text
/// smsg SMSG_BINDPOINTUPDATE = 0x155 {
///     f32 position_x;
///     f32 position_y;
///     f32 position_z;
///     Map map;
///     Area area;
/// }
/// ```
pub struct SMSG_BINDPOINTUPDATE {
    pub position_x: f32,
    pub position_y: f32,
    pub position_z: f32,
    pub map: Map,
    pub area: Area,
}

impl WorldServerMessageWrite for SMSG_BINDPOINTUPDATE {
    const OPCODE: u16 = 0x155;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (Self::size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (Self::size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for SMSG_BINDPOINTUPDATE {
    type Error = SMSG_BINDPOINTUPDATEError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // position_x: f32
        let position_x = crate::util::read_f32_le(r)?;
        // position_y: f32
        let position_y = crate::util::read_f32_le(r)?;
        // position_z: f32
        let position_z = crate::util::read_f32_le(r)?;
        // map: Map
        let map = Map::read(r)?;

        // area: Area
        let area = Area::read(r)?;

        Ok(Self {
            position_x,
            position_y,
            position_z,
            map,
            area,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // position_x: f32
        w.write_all(&self.position_x.to_le_bytes())?;

        // position_y: f32
        w.write_all(&self.position_y.to_le_bytes())?;

        // position_z: f32
        w.write_all(&self.position_z.to_le_bytes())?;

        // map: Map
        self.map.write(w)?;

        // area: Area
        self.area.write(w)?;

        Ok(())
    }
}

impl ConstantSized for SMSG_BINDPOINTUPDATE {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_BINDPOINTUPDATE {
    fn maximum_possible_size() -> usize {
        4 // position_x: f32
        + 4 // position_y: f32
        + 4 // position_z: f32
        + Map::size() // map: Map
        + Area::size() // area: Area
    }
}

#[derive(Debug)]
pub enum SMSG_BINDPOINTUPDATEError {
    Io(std::io::Error),
    Area(AreaError),
    Map(MapError),
}

impl std::error::Error for SMSG_BINDPOINTUPDATEError {}
impl std::fmt::Display for SMSG_BINDPOINTUPDATEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::Area(i) => i.fmt(f),
            Self::Map(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_BINDPOINTUPDATEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<AreaError> for SMSG_BINDPOINTUPDATEError {
    fn from(e: AreaError) -> Self {
        Self::Area(e)
    }
}

impl From<MapError> for SMSG_BINDPOINTUPDATEError {
    fn from(e: MapError) -> Self {
        Self::Map(e)
    }
}

