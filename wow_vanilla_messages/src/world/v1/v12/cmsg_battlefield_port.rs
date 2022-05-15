use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{BattlefieldPortAction, BattlefieldPortActionError};
use crate::world::v1::v12::{Map, MapError};
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, VariableSized};
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

impl MessageBody for CMSG_BATTLEFIELD_PORT {
    const OPCODE: u16 = 0x02d5;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = CMSG_BATTLEFIELD_PORTError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // map: Map
        let map: Map = crate::util::read_u32_le(r)?.try_into()?;

        // action: BattlefieldPortAction
        let action: BattlefieldPortAction = crate::util::read_u8_le(r)?.try_into()?;

        Ok(Self {
            map,
            action,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // map: Map
        crate::util::write_u32_le(w, self.map.as_int() as u32)?;

        // action: BattlefieldPortAction
        crate::util::write_u8_le(w, self.action.as_int() as u8)?;

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
            // map: Map
            let map: Map = crate::util::tokio_read_u32_le(r).await?.try_into()?;

            // action: BattlefieldPortAction
            let action: BattlefieldPortAction = crate::util::tokio_read_u8_le(r).await?.try_into()?;

            Ok(Self {
                map,
                action,
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
            // map: Map
            crate::util::tokio_write_u32_le(w, self.map.as_int() as u32).await?;

            // action: BattlefieldPortAction
            crate::util::tokio_write_u8_le(w, self.action.as_int() as u8).await?;

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
            // map: Map
            let map: Map = crate::util::astd_read_u32_le(r).await?.try_into()?;

            // action: BattlefieldPortAction
            let action: BattlefieldPortAction = crate::util::astd_read_u8_le(r).await?.try_into()?;

            Ok(Self {
                map,
                action,
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
            // map: Map
            crate::util::astd_write_u32_le(w, self.map.as_int() as u32).await?;

            // action: BattlefieldPortAction
            crate::util::astd_write_u8_le(w, self.action.as_int() as u8).await?;

            Ok(())
        })
    }

}

impl ConstantSized for CMSG_BATTLEFIELD_PORT {}

impl MaximumPossibleSized for CMSG_BATTLEFIELD_PORT {
    fn maximum_possible_size() -> usize {
        0
        + 4 // map: Map
        + 1 // action: BattlefieldPortAction
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

