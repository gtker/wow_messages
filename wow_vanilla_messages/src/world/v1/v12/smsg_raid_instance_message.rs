use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{Map, MapError};
use crate::world::v1::v12::{RaidInstanceMessage, RaidInstanceMessageError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_RAID_INSTANCE_MESSAGE {
    pub message_type: RaidInstanceMessage,
    pub map: Map,
    pub time_left: u32,
}

impl ServerMessageWrite for SMSG_RAID_INSTANCE_MESSAGE {}

impl MessageBody for SMSG_RAID_INSTANCE_MESSAGE {
    const OPCODE: u16 = 0x02fa;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = SMSG_RAID_INSTANCE_MESSAGEError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // message_type: RaidInstanceMessage
        let message_type: RaidInstanceMessage = crate::util::read_u32_le(r)?.try_into()?;

        // map: Map
        let map: Map = crate::util::read_u32_le(r)?.try_into()?;

        // time_left: u32
        let time_left = crate::util::read_u32_le(r)?;

        Ok(Self {
            message_type,
            map,
            time_left,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // message_type: RaidInstanceMessage
        w.write_all(&(self.message_type.as_int() as u32).to_le_bytes())?;

        // map: Map
        w.write_all(&(self.map.as_int() as u32).to_le_bytes())?;

        // time_left: u32
        w.write_all(&self.time_left.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
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
            // message_type: RaidInstanceMessage
            let message_type: RaidInstanceMessage = crate::util::tokio_read_u32_le(r).await?.try_into()?;

            // map: Map
            let map: Map = crate::util::tokio_read_u32_le(r).await?.try_into()?;

            // time_left: u32
            let time_left = crate::util::tokio_read_u32_le(r).await?;

            Ok(Self {
                message_type,
                map,
                time_left,
            })
        })
    }

    #[cfg(feature = "async_tokio")]
    fn tokio_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // message_type: RaidInstanceMessage
            w.write_all(&(self.message_type.as_int() as u32).to_le_bytes()).await?;

            // map: Map
            w.write_all(&(self.map.as_int() as u32).to_le_bytes()).await?;

            // time_left: u32
            w.write_all(&self.time_left.to_le_bytes()).await?;

            Ok(())
        })
    }

    #[cfg(feature = "async_std")]
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
            // message_type: RaidInstanceMessage
            let message_type: RaidInstanceMessage = crate::util::astd_read_u32_le(r).await?.try_into()?;

            // map: Map
            let map: Map = crate::util::astd_read_u32_le(r).await?.try_into()?;

            // time_left: u32
            let time_left = crate::util::astd_read_u32_le(r).await?;

            Ok(Self {
                message_type,
                map,
                time_left,
            })
        })
    }

    #[cfg(feature = "async_std")]
    fn astd_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // message_type: RaidInstanceMessage
            w.write_all(&(self.message_type.as_int() as u32).to_le_bytes()).await?;

            // map: Map
            w.write_all(&(self.map.as_int() as u32).to_le_bytes()).await?;

            // time_left: u32
            w.write_all(&self.time_left.to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl SMSG_RAID_INSTANCE_MESSAGE {
    pub(crate) fn size() -> usize {
        0
        + 4 // message_type: RaidInstanceMessage
        + 4 // map: Map
        + 4 // time_left: u32
    }
}

#[derive(Debug)]
pub enum SMSG_RAID_INSTANCE_MESSAGEError {
    Io(std::io::Error),
    Map(MapError),
    RaidInstanceMessage(RaidInstanceMessageError),
}

impl std::error::Error for SMSG_RAID_INSTANCE_MESSAGEError {}
impl std::fmt::Display for SMSG_RAID_INSTANCE_MESSAGEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::Map(i) => i.fmt(f),
            Self::RaidInstanceMessage(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_RAID_INSTANCE_MESSAGEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<MapError> for SMSG_RAID_INSTANCE_MESSAGEError {
    fn from(e: MapError) -> Self {
        Self::Map(e)
    }
}

impl From<RaidInstanceMessageError> for SMSG_RAID_INSTANCE_MESSAGEError {
    fn from(e: RaidInstanceMessageError) -> Self {
        Self::RaidInstanceMessage(e)
    }
}

