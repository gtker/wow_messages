use std::convert::{TryFrom, TryInto};
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
pub struct SMSG_UPDATE_LAST_INSTANCE {
    pub map: Map,
}

impl ServerMessageWrite for SMSG_UPDATE_LAST_INSTANCE {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_UPDATE_LAST_INSTANCE {
    const OPCODE: u16 = 0x0320;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = SMSG_UPDATE_LAST_INSTANCEError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // map: Map
        let map = Map::read(r)?;

        Ok(Self {
            map,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // map: Map
        self.map.write(w)?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // map: Map
        let map = Map::tokio_read(r).await?;

        Ok(Self {
            map,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // map: Map
        self.map.tokio_write(w).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // map: Map
        let map = Map::astd_read(r).await?;

        Ok(Self {
            map,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // map: Map
        self.map.astd_write(w).await?;

        Ok(())
    }

}

impl ConstantSized for SMSG_UPDATE_LAST_INSTANCE {}

impl MaximumPossibleSized for SMSG_UPDATE_LAST_INSTANCE {
    fn maximum_possible_size() -> usize {
        Map::size() // map: Map
    }
}

#[derive(Debug)]
pub enum SMSG_UPDATE_LAST_INSTANCEError {
    Io(std::io::Error),
    Map(MapError),
}

impl std::error::Error for SMSG_UPDATE_LAST_INSTANCEError {}
impl std::fmt::Display for SMSG_UPDATE_LAST_INSTANCEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::Map(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_UPDATE_LAST_INSTANCEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<MapError> for SMSG_UPDATE_LAST_INSTANCEError {
    fn from(e: MapError) -> Self {
        Self::Map(e)
    }
}

