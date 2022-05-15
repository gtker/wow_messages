use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{BattlegroundEndStatus, BattlegroundEndStatusError};
use crate::world::v1::v12::{BattlegroundPlayer, BattlegroundPlayerError};
use crate::world::v1::v12::{BattlegroundWinner, BattlegroundWinnerError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct MSG_PVP_LOG_DATA_Server {
    pub status: MSG_PVP_LOG_DATA_ServerBattlegroundEndStatus,
    pub players: Vec<BattlegroundPlayer>,
}

impl ServerMessageWrite for MSG_PVP_LOG_DATA_Server {}

impl MessageBody for MSG_PVP_LOG_DATA_Server {
    const OPCODE: u16 = 0x02e0;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = MSG_PVP_LOG_DATA_ServerError;

    #[cfg(feature = "sync")]
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

    #[cfg(feature = "sync")]
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
            // status: BattlegroundEndStatus
            let status = BattlegroundEndStatus::tokio_read(r).await?;

            let status_if = match status {
                BattlegroundEndStatus::NOT_ENDED => MSG_PVP_LOG_DATA_ServerBattlegroundEndStatus::NOT_ENDED,
                BattlegroundEndStatus::ENDED => {
                    // winner: BattlegroundWinner
                    let winner = BattlegroundWinner::tokio_read(r).await?;

                    MSG_PVP_LOG_DATA_ServerBattlegroundEndStatus::ENDED {
                        winner,
                    }
                }
            };

            // amount_of_players: u32
            let amount_of_players = crate::util::tokio_read_u32_le(r).await?;

            // players: BattlegroundPlayer[amount_of_players]
            let mut players = Vec::with_capacity(amount_of_players as usize);
            for i in 0..amount_of_players {
                players.push(BattlegroundPlayer::tokio_read(r).await?);
            }

            Ok(Self {
                status: status_if,
                players,
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
            // status: BattlegroundEndStatus
            self.status.tokio_write(w).await?;

            match &self.status {
                MSG_PVP_LOG_DATA_ServerBattlegroundEndStatus::NOT_ENDED => {}
                MSG_PVP_LOG_DATA_ServerBattlegroundEndStatus::ENDED {
                    winner,
                } => {
                    // winner: BattlegroundWinner
                    winner.tokio_write(w).await?;

                }
            }

            // amount_of_players: u32
            w.write_all(&(self.players.len() as u32).to_le_bytes()).await?;

            // players: BattlegroundPlayer[amount_of_players]
            for i in self.players.iter() {
                i.tokio_write(w).await?;
            }

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
            // status: BattlegroundEndStatus
            let status = BattlegroundEndStatus::astd_read(r).await?;

            let status_if = match status {
                BattlegroundEndStatus::NOT_ENDED => MSG_PVP_LOG_DATA_ServerBattlegroundEndStatus::NOT_ENDED,
                BattlegroundEndStatus::ENDED => {
                    // winner: BattlegroundWinner
                    let winner = BattlegroundWinner::astd_read(r).await?;

                    MSG_PVP_LOG_DATA_ServerBattlegroundEndStatus::ENDED {
                        winner,
                    }
                }
            };

            // amount_of_players: u32
            let amount_of_players = crate::util::astd_read_u32_le(r).await?;

            // players: BattlegroundPlayer[amount_of_players]
            let mut players = Vec::with_capacity(amount_of_players as usize);
            for i in 0..amount_of_players {
                players.push(BattlegroundPlayer::astd_read(r).await?);
            }

            Ok(Self {
                status: status_if,
                players,
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
            // status: BattlegroundEndStatus
            self.status.astd_write(w).await?;

            match &self.status {
                MSG_PVP_LOG_DATA_ServerBattlegroundEndStatus::NOT_ENDED => {}
                MSG_PVP_LOG_DATA_ServerBattlegroundEndStatus::ENDED {
                    winner,
                } => {
                    // winner: BattlegroundWinner
                    winner.astd_write(w).await?;

                }
            }

            // amount_of_players: u32
            w.write_all(&(self.players.len() as u32).to_le_bytes()).await?;

            // players: BattlegroundPlayer[amount_of_players]
            for i in self.players.iter() {
                i.astd_write(w).await?;
            }

            Ok(())
        })
    }

}

impl VariableSized for MSG_PVP_LOG_DATA_Server {
    fn size(&self) -> usize {
        0
        + self.status.size() // status: MSG_PVP_LOG_DATA_ServerBattlegroundEndStatus
        + 4 // amount_of_players: u32
        + self.players.iter().fold(0, |acc, x| acc + x.size()) // players: BattlegroundPlayer[amount_of_players]
    }
}

impl MaximumPossibleSized for MSG_PVP_LOG_DATA_Server {
    fn maximum_possible_size() -> usize {
        65535 // Capped at u16::MAX due to size field.
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

impl Default for MSG_PVP_LOG_DATA_ServerBattlegroundEndStatus {
    fn default() -> Self {
        // First enumerator without any fields
        Self::NOT_ENDED
    }
}

impl MSG_PVP_LOG_DATA_ServerBattlegroundEndStatus {
    #[cfg(feature = "sync")]
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_int().to_le_bytes())?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_int().to_le_bytes()).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_int().to_le_bytes()).await?;
        Ok(())
    }

    pub(crate) fn as_int(&self) -> u8 {
        match self {
            Self::NOT_ENDED => 0,
            Self::ENDED{ .. } => 1,
        }
    }

}

impl VariableSized for MSG_PVP_LOG_DATA_ServerBattlegroundEndStatus {
    fn size(&self) -> usize {
        match self {
            Self::NOT_ENDED => {
                1
            }
            Self::ENDED {
                winner,
            } => {
                1
                + 1 // winner: BattlegroundWinner
            }
        }
    }
}

impl MaximumPossibleSized for MSG_PVP_LOG_DATA_ServerBattlegroundEndStatus {
    fn maximum_possible_size() -> usize {
        2
    }
}

