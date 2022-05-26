use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{BattlefieldPortAction, BattlefieldPortActionError};
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
pub struct CMSG_BATTLEFIELD_PORT {
    pub map: Map,
    pub action: BattlefieldPortAction,
}

impl CMSG_BATTLEFIELD_PORT {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 5], std::io::Error> {
        let mut array_w = [0u8; 5];
        let mut w = array_w.as_mut_slice();
        // map: Map
        w.write_all(&(self.map.as_int() as u32).to_le_bytes())?;

        // action: BattlefieldPortAction
        w.write_all(&(self.action.as_int() as u8).to_le_bytes())?;

        Ok(array_w)
    }
}

impl ClientMessage for CMSG_BATTLEFIELD_PORT {
    fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(5);
        // map: Map
        w.write_all(&(self.map.as_int() as u32).to_le_bytes())?;

        // action: BattlefieldPortAction
        w.write_all(&(self.action.as_int() as u8).to_le_bytes())?;

        Ok(w)
    }
    const OPCODE: u16 = 0x02d5;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        5
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

            // action: BattlefieldPortAction
            let action: BattlefieldPortAction = crate::util::tokio_read_u8_le(r).await?.try_into()?;

            Ok(Self {
                map,
                action,
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

            // action: BattlefieldPortAction
            let action: BattlefieldPortAction = crate::util::astd_read_u8_le(r).await?.try_into()?;

            Ok(Self {
                map,
                action,
            })
        })
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

