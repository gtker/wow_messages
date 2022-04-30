use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{WhoPlayer, WhoPlayerError};
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
pub struct SMSG_WHO {
    pub online_players: u32,
    pub players: Vec<WhoPlayer>,
}

impl ServerMessageWrite for SMSG_WHO {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_WHO {
    const OPCODE: u16 = 0x0063;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_WHOError;

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

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // listed_players: u32
        w.write_all(&(self.players.len() as u32).to_le_bytes())?;

        // online_players: u32
        w.write_all(&self.online_players.to_le_bytes())?;

        // players: WhoPlayer[listed_players]
        for i in self.players.iter() {
            i.write(w)?;
        }

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
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
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // listed_players: u32
        w.write_all(&(self.players.len() as u32).to_le_bytes()).await?;

        // online_players: u32
        w.write_all(&self.online_players.to_le_bytes()).await?;

        // players: WhoPlayer[listed_players]
        for i in self.players.iter() {
            i.tokio_write(w).await?;
        }

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
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
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // listed_players: u32
        w.write_all(&(self.players.len() as u32).to_le_bytes()).await?;

        // online_players: u32
        w.write_all(&self.online_players.to_le_bytes()).await?;

        // players: WhoPlayer[listed_players]
        for i in self.players.iter() {
            i.astd_write(w).await?;
        }

        Ok(())
    }
}

impl VariableSized for SMSG_WHO {
    fn size(&self) -> usize {
        4 // listed_players: u32
        + 4 // online_players: u32
        + self.players.iter().fold(0, |acc, x| acc + x.size()) // players: WhoPlayer[listed_players]
    }
}

impl MaximumPossibleSized for SMSG_WHO {
    fn maximum_possible_size() -> usize {
        4 // listed_players: u32
        + 4 // online_players: u32
        + 4294967295 * WhoPlayer::maximum_possible_size() // players: WhoPlayer[listed_players]
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

