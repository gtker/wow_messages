use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::BattlegroundEndStatus;
use crate::world::v1::v12::BattlegroundPlayer;
use crate::world::v1::v12::BattlegroundWinner;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct MSG_PVP_LOG_DATA_Server {
    pub status: MSG_PVP_LOG_DATA_ServerBattlegroundEndStatus,
    pub players: Vec<BattlegroundPlayer>,
}

impl ServerMessage for MSG_PVP_LOG_DATA_Server {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // status: BattlegroundEndStatus
        w.write_all(&(self.status.as_int() as u8).to_le_bytes())?;

        match &self.status {
            MSG_PVP_LOG_DATA_ServerBattlegroundEndStatus::NOT_ENDED => {}
            MSG_PVP_LOG_DATA_ServerBattlegroundEndStatus::ENDED {
                winner,
            } => {
                // winner: BattlegroundWinner
                w.write_all(&(winner.as_int() as u8).to_le_bytes())?;

            }
        }

        // amount_of_players: u32
        w.write_all(&(self.players.len() as u32).to_le_bytes())?;

        // players: BattlegroundPlayer[amount_of_players]
        for i in self.players.iter() {
            i.write_into_vec(w)?;
        }

        Ok(())
    }
    const OPCODE: u16 = 0x02e0;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = crate::errors::ParseError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // status: BattlegroundEndStatus
        let status: BattlegroundEndStatus = crate::util::read_u8_le(r)?.try_into()?;

        let status_if = match status {
            BattlegroundEndStatus::NOT_ENDED => MSG_PVP_LOG_DATA_ServerBattlegroundEndStatus::NOT_ENDED,
            BattlegroundEndStatus::ENDED => {
                // winner: BattlegroundWinner
                let winner: BattlegroundWinner = crate::util::read_u8_le(r)?.try_into()?;

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

}

impl MSG_PVP_LOG_DATA_Server {
    pub(crate) fn size(&self) -> usize {
        0
        + self.status.size() // status: MSG_PVP_LOG_DATA_ServerBattlegroundEndStatus
        + 4 // amount_of_players: u32
        + self.players.iter().fold(0, |acc, x| acc + x.size()) // players: BattlegroundPlayer[amount_of_players]
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
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::NOT_ENDED => 0,
            Self::ENDED { .. } => 1,
        }
    }

}

impl MSG_PVP_LOG_DATA_ServerBattlegroundEndStatus {
    pub(crate) fn size(&self) -> usize {
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

