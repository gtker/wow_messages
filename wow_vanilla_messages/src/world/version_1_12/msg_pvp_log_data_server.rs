use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::BattlegroundEndStatus;
use crate::world::version_1_12::BattlegroundPlayer;
use crate::world::version_1_12::BattlegroundWinner;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pvp/msg_pvp_log_data_server.wowm:29`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pvp/msg_pvp_log_data_server.wowm#L29):
/// ```text
/// smsg MSG_PVP_LOG_DATA_Server = 0x02E0 {
///     BattlegroundEndStatus status;
///     if (status == ENDED) {
///         BattlegroundWinner winner;
///     }
///     u32 amount_of_players;
///     BattlegroundPlayer[amount_of_players] players;
/// }
/// ```
pub struct MSG_PVP_LOG_DATA_Server {
    pub status: MSG_PVP_LOG_DATA_Server_BattlegroundEndStatus,
    pub players: Vec<BattlegroundPlayer>,
}

impl ServerMessage for MSG_PVP_LOG_DATA_Server {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // status: BattlegroundEndStatus
        w.write_all(&(self.status.as_int() as u8).to_le_bytes())?;

        match &self.status {
            MSG_PVP_LOG_DATA_Server_BattlegroundEndStatus::NOT_ENDED => {}
            MSG_PVP_LOG_DATA_Server_BattlegroundEndStatus::ENDED {
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

    fn server_size(&self) -> u16 {
        (self.size() + 4) as u16
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // status: BattlegroundEndStatus
        let status: BattlegroundEndStatus = crate::util::read_u8_le(r)?.try_into()?;

        let status_if = match status {
            BattlegroundEndStatus::NOT_ENDED => MSG_PVP_LOG_DATA_Server_BattlegroundEndStatus::NOT_ENDED,
            BattlegroundEndStatus::ENDED => {
                // winner: BattlegroundWinner
                let winner: BattlegroundWinner = crate::util::read_u8_le(r)?.try_into()?;

                MSG_PVP_LOG_DATA_Server_BattlegroundEndStatus::ENDED {
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
        self.status.size() // status: MSG_PVP_LOG_DATA_Server_BattlegroundEndStatus
        + 4 // amount_of_players: u32
        + self.players.iter().fold(0, |acc, x| acc + x.size()) // players: BattlegroundPlayer[amount_of_players]
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum MSG_PVP_LOG_DATA_Server_BattlegroundEndStatus {
    NOT_ENDED,
    ENDED {
        winner: BattlegroundWinner,
    },
}

impl Default for MSG_PVP_LOG_DATA_Server_BattlegroundEndStatus {
    fn default() -> Self {
        // First enumerator without any fields
        Self::NOT_ENDED
    }
}

impl MSG_PVP_LOG_DATA_Server_BattlegroundEndStatus {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::NOT_ENDED => 0,
            Self::ENDED { .. } => 1,
        }
    }

}

impl MSG_PVP_LOG_DATA_Server_BattlegroundEndStatus {
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

