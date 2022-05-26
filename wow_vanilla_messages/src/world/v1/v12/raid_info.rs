use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::Map;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct RaidInfo {
    pub map: Map,
    pub reset_time: u32,
    pub instance_id: u32,
}

impl RaidInfo {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 12], std::io::Error> {
        let mut array_w = [0u8; 12];
        let mut w = array_w.as_mut_slice();
        // map: Map
        w.write_all(&(self.map.as_int() as u32).to_le_bytes())?;

        // reset_time: u32
        w.write_all(&self.reset_time.to_le_bytes())?;

        // instance_id: u32
        w.write_all(&self.instance_id.to_le_bytes())?;

        Ok(array_w)
    }
}

impl RaidInfo {
    #[cfg(feature = "sync")]
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
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

    #[cfg(feature = "tokio")]
    pub(crate) async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
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

    #[cfg(feature = "async-std")]
    pub(crate) async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
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

}

