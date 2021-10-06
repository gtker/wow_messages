use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{BattlegroundEndStatus, BattlegroundEndStatusError};
use crate::world::v1::v12::{BattlegroundPlayer, BattlegroundPlayerError};
use crate::world::v1::v12::{BattlegroundWinner, BattlegroundWinnerError};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/add_messages.wowm:1442`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/add_messages.wowm):
/// ```text
/// smsg MSG_PVP_LOG_DATA_Server = 0x2E0 {
///     BattlegroundEndStatus status;
///     if (status == ENDED) {
///         BattlegroundWinner winner;
///     }
///     u32 amount_of_players;
///     BattlegroundPlayer[amount_of_players] players;
/// }
/// ```
pub struct MSG_PVP_LOG_DATA_Server {
    pub status: MSG_PVP_LOG_DATA_ServerBattlegroundEndStatus,
    pub amount_of_players: u32,
    pub players: Vec<BattlegroundPlayer>,
}

impl WorldServerMessageWrite for MSG_PVP_LOG_DATA_Server {
    const OPCODE: u16 = 0x2e0;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (self.size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (self.size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for MSG_PVP_LOG_DATA_Server {
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
            amount_of_players,
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

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: BattlegroundEndStatus = self.into();
        a.write_u16_le(w)
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: BattlegroundEndStatus = self.into();
        a.write_u16_be(w)
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: BattlegroundEndStatus = self.into();
        a.write_u32_le(w)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: BattlegroundEndStatus = self.into();
        a.write_u32_be(w)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: BattlegroundEndStatus = self.into();
        a.write_u64_le(w)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: BattlegroundEndStatus = self.into();
        a.write_u64_be(w)
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

