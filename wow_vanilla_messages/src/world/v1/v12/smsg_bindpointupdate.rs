use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{Area, AreaError};
use crate::world::v1::v12::{Map, MapError};
use crate::{ServerMessageWrite, MessageBody};
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
pub struct SMSG_BINDPOINTUPDATE {
    pub position_x: f32,
    pub position_y: f32,
    pub position_z: f32,
    pub map: Map,
    pub area: Area,
}

impl ServerMessageWrite for SMSG_BINDPOINTUPDATE {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_BINDPOINTUPDATE {
    const OPCODE: u16 = 0x0155;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

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

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // position_x: f32
        let position_x = crate::util::tokio_read_f32_le(r).await?;
        // position_y: f32
        let position_y = crate::util::tokio_read_f32_le(r).await?;
        // position_z: f32
        let position_z = crate::util::tokio_read_f32_le(r).await?;
        // map: Map
        let map = Map::tokio_read(r).await?;

        // area: Area
        let area = Area::tokio_read(r).await?;

        Ok(Self {
            position_x,
            position_y,
            position_z,
            map,
            area,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // position_x: f32
        w.write_all(&self.position_x.to_le_bytes()).await?;

        // position_y: f32
        w.write_all(&self.position_y.to_le_bytes()).await?;

        // position_z: f32
        w.write_all(&self.position_z.to_le_bytes()).await?;

        // map: Map
        self.map.tokio_write(w).await?;

        // area: Area
        self.area.tokio_write(w).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // position_x: f32
        let position_x = crate::util::astd_read_f32_le(r).await?;
        // position_y: f32
        let position_y = crate::util::astd_read_f32_le(r).await?;
        // position_z: f32
        let position_z = crate::util::astd_read_f32_le(r).await?;
        // map: Map
        let map = Map::astd_read(r).await?;

        // area: Area
        let area = Area::astd_read(r).await?;

        Ok(Self {
            position_x,
            position_y,
            position_z,
            map,
            area,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // position_x: f32
        w.write_all(&self.position_x.to_le_bytes()).await?;

        // position_y: f32
        w.write_all(&self.position_y.to_le_bytes()).await?;

        // position_z: f32
        w.write_all(&self.position_z.to_le_bytes()).await?;

        // map: Map
        self.map.astd_write(w).await?;

        // area: Area
        self.area.astd_write(w).await?;

        Ok(())
    }

}

impl ConstantSized for SMSG_BINDPOINTUPDATE {}

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

