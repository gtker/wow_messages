use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{DuelWinnerReason, DuelWinnerReasonError};
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
pub struct SMSG_DUEL_WINNER {
    pub reason: DuelWinnerReason,
    pub opponent_name: String,
    pub initiator_name: String,
}

impl ServerMessageWrite for SMSG_DUEL_WINNER {
    const OPCODE: u16 = 0x16b;

    fn size_without_size_field(&self) -> u16 {
        self.size() as u16
    }

}

impl MessageBody for SMSG_DUEL_WINNER {
    type Error = SMSG_DUEL_WINNERError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // reason: DuelWinnerReason
        let reason = DuelWinnerReason::read(r)?;

        // opponent_name: CString
        let opponent_name = crate::util::read_c_string_to_vec(r)?;
        let opponent_name = String::from_utf8(opponent_name)?;

        // initiator_name: CString
        let initiator_name = crate::util::read_c_string_to_vec(r)?;
        let initiator_name = String::from_utf8(initiator_name)?;

        Ok(Self {
            reason,
            opponent_name,
            initiator_name,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // reason: DuelWinnerReason
        self.reason.write(w)?;

        // opponent_name: CString
        w.write_all(self.opponent_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // initiator_name: CString
        w.write_all(self.initiator_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
}

impl VariableSized for SMSG_DUEL_WINNER {
    fn size(&self) -> usize {
        DuelWinnerReason::size() // reason: DuelWinnerReason
        + self.opponent_name.len() + 1 // opponent_name: CString and Null Terminator
        + self.initiator_name.len() + 1 // initiator_name: CString and Null Terminator
    }
}

impl MaximumPossibleSized for SMSG_DUEL_WINNER {
    fn maximum_possible_size() -> usize {
        DuelWinnerReason::maximum_possible_size() // reason: DuelWinnerReason
        + 256 // opponent_name: CString
        + 256 // initiator_name: CString
    }
}

#[derive(Debug)]
pub enum SMSG_DUEL_WINNERError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
    DuelWinnerReason(DuelWinnerReasonError),
}

impl std::error::Error for SMSG_DUEL_WINNERError {}
impl std::fmt::Display for SMSG_DUEL_WINNERError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
            Self::DuelWinnerReason(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_DUEL_WINNERError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_DUEL_WINNERError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

impl From<DuelWinnerReasonError> for SMSG_DUEL_WINNERError {
    fn from(e: DuelWinnerReasonError) -> Self {
        Self::DuelWinnerReason(e)
    }
}

