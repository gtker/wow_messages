use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{Map, MapError};
use crate::world::v1::v12::{RaidInstanceMessage, RaidInstanceMessageError};
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
pub struct SMSG_RAID_INSTANCE_MESSAGE {
    pub message_type: RaidInstanceMessage,
    pub map: Map,
    pub time_left: u32,
}

impl ServerMessageWrite for SMSG_RAID_INSTANCE_MESSAGE {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_RAID_INSTANCE_MESSAGE {
    const OPCODE: u16 = 0x02fa;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = SMSG_RAID_INSTANCE_MESSAGEError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // message_type: RaidInstanceMessage
        let message_type = RaidInstanceMessage::read(r)?;

        // map: Map
        let map = Map::read(r)?;

        // time_left: u32
        let time_left = crate::util::read_u32_le(r)?;

        Ok(Self {
            message_type,
            map,
            time_left,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // message_type: RaidInstanceMessage
        self.message_type.write(w)?;

        // map: Map
        self.map.write(w)?;

        // time_left: u32
        w.write_all(&self.time_left.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // message_type: RaidInstanceMessage
        let message_type = RaidInstanceMessage::tokio_read(r).await?;

        // map: Map
        let map = Map::tokio_read(r).await?;

        // time_left: u32
        let time_left = crate::util::tokio_read_u32_le(r).await?;

        Ok(Self {
            message_type,
            map,
            time_left,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // message_type: RaidInstanceMessage
        self.message_type.tokio_write(w).await?;

        // map: Map
        self.map.tokio_write(w).await?;

        // time_left: u32
        w.write_all(&self.time_left.to_le_bytes()).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // message_type: RaidInstanceMessage
        let message_type = RaidInstanceMessage::astd_read(r).await?;

        // map: Map
        let map = Map::astd_read(r).await?;

        // time_left: u32
        let time_left = crate::util::astd_read_u32_le(r).await?;

        Ok(Self {
            message_type,
            map,
            time_left,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // message_type: RaidInstanceMessage
        self.message_type.astd_write(w).await?;

        // map: Map
        self.map.astd_write(w).await?;

        // time_left: u32
        w.write_all(&self.time_left.to_le_bytes()).await?;

        Ok(())
    }
}

impl ConstantSized for SMSG_RAID_INSTANCE_MESSAGE {}

impl MaximumPossibleSized for SMSG_RAID_INSTANCE_MESSAGE {
    fn maximum_possible_size() -> usize {
        RaidInstanceMessage::size() // message_type: RaidInstanceMessage
        + Map::size() // map: Map
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

