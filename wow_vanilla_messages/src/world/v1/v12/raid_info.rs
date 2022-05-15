use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{Map, MapError};
use crate::{ConstantSized, MaximumPossibleSized, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct RaidInfo {
    pub map: Map,
    pub reset_time: u32,
    pub instance_id: u32,
}

impl RaidInfo {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, RaidInfoError> {
        // map: Map
        let map: Map = crate::util::read_u32_le(r)?.try_into()?;

        // reset_time: u32
        let reset_time = crate::util::read_u32_le(r)?;

        // instance_id: u32
        let instance_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            map,
            reset_time,
            instance_id,
        })
    }

    pub(crate) fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // map: Map
        crate::util::write_u32_le(w, self.map.as_int() as u32)?;

        // reset_time: u32
        w.write_all(&self.reset_time.to_le_bytes())?;

        // instance_id: u32
        w.write_all(&self.instance_id.to_le_bytes())?;

        Ok(())
    }

    pub(crate) async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, RaidInfoError> {
        // map: Map
        let map: Map = crate::util::tokio_read_u32_le(r).await?.try_into()?;

        // reset_time: u32
        let reset_time = crate::util::tokio_read_u32_le(r).await?;

        // instance_id: u32
        let instance_id = crate::util::tokio_read_u32_le(r).await?;

        Ok(Self {
            map,
            reset_time,
            instance_id,
        })
    }

    pub(crate) async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // map: Map
        crate::util::tokio_write_u32_le(w, self.map.as_int() as u32).await?;

        // reset_time: u32
        w.write_all(&self.reset_time.to_le_bytes()).await?;

        // instance_id: u32
        w.write_all(&self.instance_id.to_le_bytes()).await?;

        Ok(())
    }

    pub(crate) async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, RaidInfoError> {
        // map: Map
        let map: Map = crate::util::astd_read_u32_le(r).await?.try_into()?;

        // reset_time: u32
        let reset_time = crate::util::astd_read_u32_le(r).await?;

        // instance_id: u32
        let instance_id = crate::util::astd_read_u32_le(r).await?;

        Ok(Self {
            map,
            reset_time,
            instance_id,
        })
    }

    pub(crate) async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // map: Map
        crate::util::astd_write_u32_le(w, self.map.as_int() as u32).await?;

        // reset_time: u32
        w.write_all(&self.reset_time.to_le_bytes()).await?;

        // instance_id: u32
        w.write_all(&self.instance_id.to_le_bytes()).await?;

        Ok(())
    }

}

impl ConstantSized for RaidInfo {}

impl MaximumPossibleSized for RaidInfo {
    fn maximum_possible_size() -> usize {
        0
        + 4 // map: Map
        + 4 // reset_time: u32
        + 4 // instance_id: u32
    }
}

#[derive(Debug)]
pub enum RaidInfoError {
    Io(std::io::Error),
    Map(MapError),
}

impl std::error::Error for RaidInfoError {}
impl std::fmt::Display for RaidInfoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::Map(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for RaidInfoError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<MapError> for RaidInfoError {
    fn from(e: MapError) -> Self {
        Self::Map(e)
    }
}

