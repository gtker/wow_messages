use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{BattlegroundEndStatus, BattlegroundEndStatusError};
use crate::world::v1::v12::{BattlegroundPlayer, BattlegroundPlayerError};
use crate::world::v1::v12::{BattlegroundWinner, BattlegroundWinnerError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use crate::AsyncReadWrite;
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct MSG_PVP_LOG_DATA_Server {
    pub status: MSG_PVP_LOG_DATA_ServerBattlegroundEndStatus,
    pub players: Vec<BattlegroundPlayer>,
}

impl ServerMessageWrite for MSG_PVP_LOG_DATA_Server {
    const OPCODE: u16 = 0x2e0;

    fn size_without_size_field(&self) -> u16 {
        self.size() as u16
    }

}

impl MessageBody for MSG_PVP_LOG_DATA_Server {
    type Error = MSG_PVP_LOG_DATA_ServerError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // status: BattlegroundEndStatus
        let status = BattlegroundEndStatus::read(r)?;

        let status_if = match status {
            BattlegroundEndStatus::NOT_ENDED => MSG_PVP_LOG_DATA_ServerBattlegroundEndStatus::NOT_ENDED,
            BattlegroundEndStatus::ENDED => {
                // winner: BattlegroundWinner
                let winner = BattlegroundWinner::read(r)?;

                MSG_PVP_LOG_DATA_ServerBattlegroundEndStatus::ENDED {
                    winner,
                }
            }
        };

        // amount_of_players: u32
        let amount_of_players = crate::util::read_u32_le(r)?;

        // players: BattlegroundPlayer[amount_of_players]
        let mut players = Vec::with_capacity(amount_of_players as usize);
        for i in 0..amount_of_players {
            players.push(BattlegroundPlayer::read(r)?);
        }

        Ok(Self {
            status: status_if,
            players,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // status: BattlegroundEndStatus
        self.status.write(w)?;

        match &self.status {
            MSG_PVP_LOG_DATA_ServerBattlegroundEndStatus::NOT_ENDED => {}
            MSG_PVP_LOG_DATA_ServerBattlegroundEndStatus::ENDED {
                winner,
            } => {
                // winner: BattlegroundWinner
                winner.write(w)?;

            }
        }

        // amount_of_players: u32
        w.write_all(&(self.players.len() as u32).to_le_bytes())?;

        // players: BattlegroundPlayer[amount_of_players]
        for i in self.players.iter() {
            i.write(w)?;
        }

        Ok(())
    }
}

impl VariableSized for MSG_PVP_LOG_DATA_Server {
    fn size(&self) -> usize {
        self.status.size() // status: BattlegroundEndStatus and subfields
        + 4 // amount_of_players: u32
        + self.players.iter().fold(0, |acc, x| acc + x.size()) // players: BattlegroundPlayer[amount_of_players]
    }
}

impl MaximumPossibleSized for MSG_PVP_LOG_DATA_Server {
    fn maximum_possible_size() -> usize {
        BattlegroundEndStatus::maximum_possible_size() // status: BattlegroundEndStatus
        + 4 // amount_of_players: u32
        + 4294967295 * BattlegroundPlayer::maximum_possible_size() // players: BattlegroundPlayer[amount_of_players]
    }
}

#[derive(Debug)]
pub enum MSG_PVP_LOG_DATA_ServerError {
    Io(std::io::Error),
    BattlegroundEndStatus(BattlegroundEndStatusError),
    BattlegroundPlayer(BattlegroundPlayerError),
    BattlegroundWinner(BattlegroundWinnerError),
}

impl std::error::Error for MSG_PVP_LOG_DATA_ServerError {}
impl std::fmt::Display for MSG_PVP_LOG_DATA_ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::BattlegroundEndStatus(i) => i.fmt(f),
            Self::BattlegroundPlayer(i) => i.fmt(f),
            Self::BattlegroundWinner(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for MSG_PVP_LOG_DATA_ServerError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<BattlegroundEndStatusError> for MSG_PVP_LOG_DATA_ServerError {
    fn from(e: BattlegroundEndStatusError) -> Self {
        Self::BattlegroundEndStatus(e)
    }
}

impl From<BattlegroundPlayerError> for MSG_PVP_LOG_DATA_ServerError {
    fn from(e: BattlegroundPlayerError) -> Self {
        Self::BattlegroundPlayer(e)
    }
}

impl From<BattlegroundWinnerError> for MSG_PVP_LOG_DATA_ServerError {
    fn from(e: BattlegroundWinnerError) -> Self {
        Self::BattlegroundWinner(e)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum MSG_PVP_LOG_DATA_ServerBattlegroundEndStatus {
    NOT_ENDED,
    ENDED {
        winner: BattlegroundWinner,
    },
}

impl From<&BattlegroundEndStatus> for MSG_PVP_LOG_DATA_ServerBattlegroundEndStatus {
    fn from(e: &BattlegroundEndStatus) -> Self {
        match &e {
            BattlegroundEndStatus::NOT_ENDED => Self::NOT_ENDED,
            BattlegroundEndStatus::ENDED => Self::ENDED {
                winner: Default::default(),
            },
        }
    }
}

impl From<&MSG_PVP_LOG_DATA_ServerBattlegroundEndStatus> for BattlegroundEndStatus {
    fn from(v: &MSG_PVP_LOG_DATA_ServerBattlegroundEndStatus) -> Self {
        match &v {
            MSG_PVP_LOG_DATA_ServerBattlegroundEndStatus::NOT_ENDED => Self::NOT_ENDED,
            MSG_PVP_LOG_DATA_ServerBattlegroundEndStatus::ENDED { .. } => Self::ENDED,
        }
    }
}

impl Default for MSG_PVP_LOG_DATA_ServerBattlegroundEndStatus {
    fn default() -> Self {
        // First enumerator without any fields
        Self::NOT_ENDED
    }
}

impl MSG_PVP_LOG_DATA_ServerBattlegroundEndStatus {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: BattlegroundEndStatus = self.into();
        a.write(w)?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: BattlegroundEndStatus = self.into();
        a.tokio_write(w).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: BattlegroundEndStatus = self.into();
        a.astd_write(w).await?;
        Ok(())
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: BattlegroundEndStatus = self.into();
        a.write_u16_le(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u16_le<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: BattlegroundEndStatus = self.into();
        a.tokio_write_u16_le(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u16_le<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: BattlegroundEndStatus = self.into();
        a.astd_write_u16_le(w).await
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: BattlegroundEndStatus = self.into();
        a.write_u16_be(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u16_be<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: BattlegroundEndStatus = self.into();
        a.tokio_write_u16_be(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u16_be<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: BattlegroundEndStatus = self.into();
        a.astd_write_u16_be(w).await
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: BattlegroundEndStatus = self.into();
        a.write_u32_le(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u32_le<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: BattlegroundEndStatus = self.into();
        a.tokio_write_u32_le(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u32_le<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: BattlegroundEndStatus = self.into();
        a.astd_write_u32_le(w).await
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: BattlegroundEndStatus = self.into();
        a.write_u32_be(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u32_be<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: BattlegroundEndStatus = self.into();
        a.tokio_write_u32_be(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u32_be<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: BattlegroundEndStatus = self.into();
        a.astd_write_u32_be(w).await
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: BattlegroundEndStatus = self.into();
        a.write_u64_le(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u64_le<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: BattlegroundEndStatus = self.into();
        a.tokio_write_u64_le(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u64_le<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: BattlegroundEndStatus = self.into();
        a.astd_write_u64_le(w).await
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: BattlegroundEndStatus = self.into();
        a.write_u64_be(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u64_be<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: BattlegroundEndStatus = self.into();
        a.tokio_write_u64_be(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u64_be<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: BattlegroundEndStatus = self.into();
        a.astd_write_u64_be(w).await
    }

}

impl VariableSized for MSG_PVP_LOG_DATA_ServerBattlegroundEndStatus {
    fn size(&self) -> usize {
        match self {
            Self::NOT_ENDED =>  {
                1
            }
            Self::ENDED  {
                winner,
            } => {
                1
                + BattlegroundWinner::size() // winner: BattlegroundWinner
            }
        }
    }
}

impl MaximumPossibleSized for MSG_PVP_LOG_DATA_ServerBattlegroundEndStatus {
    fn maximum_possible_size() -> usize {
        65536 // maximum possible u16 size. TODO value.
    }
}

