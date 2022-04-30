use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{BattlefieldPortAction, BattlefieldPortActionError};
use crate::world::v1::v12::{Map, MapError};
use crate::{ClientMessageWrite, MessageBody};
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
pub struct CMSG_BATTLEFIELD_PORT {
    pub map: Map,
    pub action: BattlefieldPortAction,
}

impl ClientMessageWrite for CMSG_BATTLEFIELD_PORT {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for CMSG_BATTLEFIELD_PORT {
    const OPCODE: u16 = 0x02d5;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = CMSG_BATTLEFIELD_PORTError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // map: Map
        let map = Map::read(r)?;

        // action: BattlefieldPortAction
        let action = BattlefieldPortAction::read(r)?;

        Ok(Self {
            map,
            action,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // map: Map
        self.map.write(w)?;

        // action: BattlefieldPortAction
        self.action.write(w)?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // map: Map
        let map = Map::tokio_read(r).await?;

        // action: BattlefieldPortAction
        let action = BattlefieldPortAction::tokio_read(r).await?;

        Ok(Self {
            map,
            action,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // map: Map
        self.map.tokio_write(w).await?;

        // action: BattlefieldPortAction
        self.action.tokio_write(w).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // map: Map
        let map = Map::astd_read(r).await?;

        // action: BattlefieldPortAction
        let action = BattlefieldPortAction::astd_read(r).await?;

        Ok(Self {
            map,
            action,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // map: Map
        self.map.astd_write(w).await?;

        // action: BattlefieldPortAction
        self.action.astd_write(w).await?;

        Ok(())
    }

}

impl ConstantSized for CMSG_BATTLEFIELD_PORT {}

impl MaximumPossibleSized for CMSG_BATTLEFIELD_PORT {
    fn maximum_possible_size() -> usize {
        Map::size() // map: Map
        + BattlefieldPortAction::size() // action: BattlefieldPortAction
    }
}

#[derive(Debug)]
pub enum CMSG_BATTLEFIELD_PORTError {
    Io(std::io::Error),
    BattlefieldPortAction(BattlefieldPortActionError),
    Map(MapError),
}

impl std::error::Error for CMSG_BATTLEFIELD_PORTError {}
impl std::fmt::Display for CMSG_BATTLEFIELD_PORTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::BattlefieldPortAction(i) => i.fmt(f),
            Self::Map(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_BATTLEFIELD_PORTError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<BattlefieldPortActionError> for CMSG_BATTLEFIELD_PORTError {
    fn from(e: BattlefieldPortActionError) -> Self {
        Self::BattlefieldPortAction(e)
    }
}

impl From<MapError> for CMSG_BATTLEFIELD_PORTError {
    fn from(e: MapError) -> Self {
        Self::Map(e)
    }
}

