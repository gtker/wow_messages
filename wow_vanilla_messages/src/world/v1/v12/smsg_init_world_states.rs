use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{Area, AreaError};
use crate::world::v1::v12::{Map, MapError};
use crate::world::v1::v12::WorldState;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_INIT_WORLD_STATES {
    pub map: Map,
    pub area: Area,
    pub states: Vec<WorldState>,
}

impl SMSG_INIT_WORLD_STATES {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
        // map: Map
        w.write_all(&(self.map.as_int() as u32).to_le_bytes())?;

        // area: Area
        w.write_all(&(self.area.as_int() as u32).to_le_bytes())?;

        // amount_of_states: u16
        w.write_all(&(self.states.len() as u16).to_le_bytes())?;

        // states: WorldState[amount_of_states]
        for i in self.states.iter() {
            w.write_all(&(i.as_bytes()?))?;
        }

        Ok(w)
    }
}

impl ServerMessage for SMSG_INIT_WORLD_STATES {
    fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
        // map: Map
        w.write_all(&(self.map.as_int() as u32).to_le_bytes())?;

        // area: Area
        w.write_all(&(self.area.as_int() as u32).to_le_bytes())?;

        // amount_of_states: u16
        w.write_all(&(self.states.len() as u16).to_le_bytes())?;

        // states: WorldState[amount_of_states]
        for i in self.states.iter() {
            w.write_all(&(i.as_bytes()?))?;
        }

        Ok(w)
    }
    const OPCODE: u16 = 0x02c2;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_INIT_WORLD_STATESError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // map: Map
        let map: Map = crate::util::read_u32_le(r)?.try_into()?;

        // area: Area
        let area: Area = crate::util::read_u32_le(r)?.try_into()?;

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

    #[cfg(feature = "tokio")]
    fn tokio_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // map: Map
            let map: Map = crate::util::tokio_read_u32_le(r).await?.try_into()?;

            // area: Area
            let area: Area = crate::util::tokio_read_u32_le(r).await?.try_into()?;

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
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // map: Map
            let map: Map = crate::util::astd_read_u32_le(r).await?.try_into()?;

            // area: Area
            let area: Area = crate::util::astd_read_u32_le(r).await?.try_into()?;

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
        })
    }

}

impl SMSG_INIT_WORLD_STATES {
    pub fn size(&self) -> usize {
        0
        + 4 // map: Map
        + 4 // area: Area
        + 2 // amount_of_states: u16
        + self.states.len() * 8 // states: WorldState[amount_of_states]
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

