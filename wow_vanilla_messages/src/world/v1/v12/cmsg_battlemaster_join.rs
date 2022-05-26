use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{Map, MapError};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMSG_BATTLEMASTER_JOIN {
    pub guid: Guid,
    pub map: Map,
    pub instance_id: u32,
    pub join_as_group: u8,
}

impl CMSG_BATTLEMASTER_JOIN {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 17], std::io::Error> {
        let mut array_w = [0u8; 17];
        let mut w = array_w.as_mut_slice();
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // map: Map
        w.write_all(&(self.map.as_int() as u32).to_le_bytes())?;

        // instance_id: u32
        w.write_all(&self.instance_id.to_le_bytes())?;

        // join_as_group: u8
        w.write_all(&self.join_as_group.to_le_bytes())?;

        Ok(array_w)
    }
}

impl ClientMessage for CMSG_BATTLEMASTER_JOIN {
    fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(17);
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // map: Map
        w.write_all(&(self.map.as_int() as u32).to_le_bytes())?;

        // instance_id: u32
        w.write_all(&self.instance_id.to_le_bytes())?;

        // join_as_group: u8
        w.write_all(&self.join_as_group.to_le_bytes())?;

        Ok(w)
    }
    const OPCODE: u16 = 0x02ee;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        17
    }

    type Error = CMSG_BATTLEMASTER_JOINError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // map: Map
        let map: Map = crate::util::read_u32_le(r)?.try_into()?;

        // instance_id: u32
        let instance_id = crate::util::read_u32_le(r)?;

        // join_as_group: u8
        let join_as_group = crate::util::read_u8_le(r)?;

        Ok(Self {
            guid,
            map,
            instance_id,
            join_as_group,
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
            // guid: Guid
            let guid = Guid::tokio_read(r).await?;

            // map: Map
            let map: Map = crate::util::tokio_read_u32_le(r).await?.try_into()?;

            // instance_id: u32
            let instance_id = crate::util::tokio_read_u32_le(r).await?;

            // join_as_group: u8
            let join_as_group = crate::util::tokio_read_u8_le(r).await?;

            Ok(Self {
                guid,
                map,
                instance_id,
                join_as_group,
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
            // guid: Guid
            let guid = Guid::astd_read(r).await?;

            // map: Map
            let map: Map = crate::util::astd_read_u32_le(r).await?.try_into()?;

            // instance_id: u32
            let instance_id = crate::util::astd_read_u32_le(r).await?;

            // join_as_group: u8
            let join_as_group = crate::util::astd_read_u8_le(r).await?;

            Ok(Self {
                guid,
                map,
                instance_id,
                join_as_group,
            })
        })
    }

}

#[derive(Debug)]
pub enum CMSG_BATTLEMASTER_JOINError {
    Io(std::io::Error),
    Map(MapError),
}

impl std::error::Error for CMSG_BATTLEMASTER_JOINError {}
impl std::fmt::Display for CMSG_BATTLEMASTER_JOINError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::Map(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_BATTLEMASTER_JOINError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<MapError> for CMSG_BATTLEMASTER_JOINError {
    fn from(e: MapError) -> Self {
        Self::Map(e)
    }
}

