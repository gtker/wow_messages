use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{Area, AreaError};
use crate::world::v1::v12::{Map, MapError};
use crate::world::v1::v12::WorldState;
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
pub struct SMSG_INIT_WORLD_STATES {
    pub map: Map,
    pub area: Area,
    pub states: Vec<WorldState>,
}

impl ServerMessageWrite for SMSG_INIT_WORLD_STATES {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_INIT_WORLD_STATES {
    const OPCODE: u16 = 0x02c2;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_INIT_WORLD_STATESError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // map: Map
        let map = Map::read(r)?;

        // area: Area
        let area = Area::read(r)?;

        // amount_of_states: u16
        let amount_of_states = crate::util::read_u16_le(r)?;

        // states: WorldState[amount_of_states]
        let mut states = Vec::with_capacity(amount_of_states as usize);
        for i in 0..amount_of_states {
            states.push(WorldState::read(r)?);
        }

        Ok(Self {
            map,
            area,
            states,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // map: Map
        self.map.write(w)?;

        // area: Area
        self.area.write(w)?;

        // amount_of_states: u16
        w.write_all(&(self.states.len() as u16).to_le_bytes())?;

        // states: WorldState[amount_of_states]
        for i in self.states.iter() {
            i.write(w)?;
        }

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // map: Map
        let map = Map::tokio_read(r).await?;

        // area: Area
        let area = Area::tokio_read(r).await?;

        // amount_of_states: u16
        let amount_of_states = crate::util::tokio_read_u16_le(r).await?;

        // states: WorldState[amount_of_states]
        let mut states = Vec::with_capacity(amount_of_states as usize);
        for i in 0..amount_of_states {
            states.push(WorldState::tokio_read(r).await?);
        }

        Ok(Self {
            map,
            area,
            states,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // map: Map
        self.map.tokio_write(w).await?;

        // area: Area
        self.area.tokio_write(w).await?;

        // amount_of_states: u16
        w.write_all(&(self.states.len() as u16).to_le_bytes()).await?;

        // states: WorldState[amount_of_states]
        for i in self.states.iter() {
            i.tokio_write(w).await?;
        }

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // map: Map
        let map = Map::astd_read(r).await?;

        // area: Area
        let area = Area::astd_read(r).await?;

        // amount_of_states: u16
        let amount_of_states = crate::util::astd_read_u16_le(r).await?;

        // states: WorldState[amount_of_states]
        let mut states = Vec::with_capacity(amount_of_states as usize);
        for i in 0..amount_of_states {
            states.push(WorldState::astd_read(r).await?);
        }

        Ok(Self {
            map,
            area,
            states,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // map: Map
        self.map.astd_write(w).await?;

        // area: Area
        self.area.astd_write(w).await?;

        // amount_of_states: u16
        w.write_all(&(self.states.len() as u16).to_le_bytes()).await?;

        // states: WorldState[amount_of_states]
        for i in self.states.iter() {
            i.astd_write(w).await?;
        }

        Ok(())
    }

}

impl VariableSized for SMSG_INIT_WORLD_STATES {
    fn size(&self) -> usize {
        Map::size() // map: Map
        + Area::size() // area: Area
        + 2 // amount_of_states: u16
        + self.states.iter().fold(0, |acc, x| acc + WorldState::size()) // states: WorldState[amount_of_states]
    }
}

impl MaximumPossibleSized for SMSG_INIT_WORLD_STATES {
    fn maximum_possible_size() -> usize {
        Map::maximum_possible_size() // map: Map
        + Area::maximum_possible_size() // area: Area
        + 2 // amount_of_states: u16
        + 65535 * WorldState::maximum_possible_size() // states: WorldState[amount_of_states]
    }
}

#[derive(Debug)]
pub enum SMSG_INIT_WORLD_STATESError {
    Io(std::io::Error),
    Area(AreaError),
    Map(MapError),
}

impl std::error::Error for SMSG_INIT_WORLD_STATESError {}
impl std::fmt::Display for SMSG_INIT_WORLD_STATESError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::Area(i) => i.fmt(f),
            Self::Map(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_INIT_WORLD_STATESError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<AreaError> for SMSG_INIT_WORLD_STATESError {
    fn from(e: AreaError) -> Self {
        Self::Area(e)
    }
}

impl From<MapError> for SMSG_INIT_WORLD_STATESError {
    fn from(e: MapError) -> Self {
        Self::Map(e)
    }
}

