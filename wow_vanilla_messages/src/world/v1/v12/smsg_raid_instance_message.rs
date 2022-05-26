use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::Map;
use crate::world::v1::v12::RaidInstanceMessage;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_RAID_INSTANCE_MESSAGE {
    pub message_type: RaidInstanceMessage,
    pub map: Map,
    pub time_left: u32,
}

impl SMSG_RAID_INSTANCE_MESSAGE {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 12], std::io::Error> {
        let mut array_w = [0u8; 12];
        let mut w = array_w.as_mut_slice();
        // message_type: RaidInstanceMessage
        w.write_all(&(self.message_type.as_int() as u32).to_le_bytes())?;

        // map: Map
        w.write_all(&(self.map.as_int() as u32).to_le_bytes())?;

        // time_left: u32
        w.write_all(&self.time_left.to_le_bytes())?;

        Ok(array_w)
    }
}

impl ServerMessage for SMSG_RAID_INSTANCE_MESSAGE {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // message_type: RaidInstanceMessage
        w.write_all(&(self.message_type.as_int() as u32).to_le_bytes())?;

        // map: Map
        w.write_all(&(self.map.as_int() as u32).to_le_bytes())?;

        // time_left: u32
        w.write_all(&self.time_left.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x02fa;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        12
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

}

#[derive(Debug)]
pub enum SMSG_RAID_INSTANCE_MESSAGEError {
    Io(std::io::Error),
    Enum(crate::errors::EnumError),
}

impl std::error::Error for SMSG_RAID_INSTANCE_MESSAGEError {}
impl std::fmt::Display for SMSG_RAID_INSTANCE_MESSAGEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::Enum(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_RAID_INSTANCE_MESSAGEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<crate::errors::EnumError> for SMSG_RAID_INSTANCE_MESSAGEError {
    fn from(e: crate::errors::EnumError) -> Self {
        Self::Enum(e)
    }
}

