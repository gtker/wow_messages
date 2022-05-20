use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{WhoPlayer, WhoPlayerError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_WHO {
    pub online_players: u32,
    pub players: Vec<WhoPlayer>,
}

impl ServerMessageWrite for SMSG_WHO {}

impl SMSG_WHO {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(2259152797704);
        // listed_players: u32
        w.write_all(&(self.players.len() as u32).to_le_bytes())?;

        // online_players: u32
        w.write_all(&self.online_players.to_le_bytes())?;

        // players: WhoPlayer[listed_players]
        for i in self.players.iter() {
            w.write_all(&(i.as_bytes()?))?;
        }

        Ok(w)
    }
}

impl MessageBody for SMSG_WHO {
    const OPCODE: u16 = 0x0063;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_WHOError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // listed_players: u32
        let listed_players = crate::util::read_u32_le(r)?;

        // online_players: u32
        let online_players = crate::util::read_u32_le(r)?;

        // players: WhoPlayer[listed_players]
        let mut players = Vec::with_capacity(listed_players as usize);
        for i in 0..listed_players {
            players.push(WhoPlayer::read(r)?);
        }

        Ok(Self {
            online_players,
            players,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let inner = self.as_bytes()?;
        w.write_all(&inner)
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
            // listed_players: u32
            let listed_players = crate::util::tokio_read_u32_le(r).await?;

            // online_players: u32
            let online_players = crate::util::tokio_read_u32_le(r).await?;

            // players: WhoPlayer[listed_players]
            let mut players = Vec::with_capacity(listed_players as usize);
            for i in 0..listed_players {
                players.push(WhoPlayer::tokio_read(r).await?);
            }

            Ok(Self {
                online_players,
                players,
            })
        })
    }

    #[cfg(feature = "tokio")]
    fn tokio_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let inner = self.as_bytes()?;
            w.write_all(&inner).await
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
            // listed_players: u32
            let listed_players = crate::util::astd_read_u32_le(r).await?;

            // online_players: u32
            let online_players = crate::util::astd_read_u32_le(r).await?;

            // players: WhoPlayer[listed_players]
            let mut players = Vec::with_capacity(listed_players as usize);
            for i in 0..listed_players {
                players.push(WhoPlayer::astd_read(r).await?);
            }

            Ok(Self {
                online_players,
                players,
            })
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let inner = self.as_bytes()?;
            w.write_all(&inner).await
        })
    }

}

impl SMSG_WHO {
    pub fn size(&self) -> usize {
        0
        + 4 // listed_players: u32
        + 4 // online_players: u32
        + self.players.iter().fold(0, |acc, x| acc + x.size()) // players: WhoPlayer[listed_players]
    }
}

#[derive(Debug)]
pub enum SMSG_WHOError {
    Io(std::io::Error),
    WhoPlayer(WhoPlayerError),
}

impl std::error::Error for SMSG_WHOError {}
impl std::fmt::Display for SMSG_WHOError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::WhoPlayer(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_WHOError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<WhoPlayerError> for SMSG_WHOError {
    fn from(e: WhoPlayerError) -> Self {
        Self::WhoPlayer(e)
    }
}

