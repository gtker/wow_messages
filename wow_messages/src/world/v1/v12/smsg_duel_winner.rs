use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{DuelWinnerReason, DuelWinnerReasonError};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/new_all.wowm:2082`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/new_all.wowm#L2082):
/// ```text
/// smsg SMSG_DUEL_WINNER = 0x16B {
///     DuelWinnerReason reason;
///     CString opponent_name;
///     CString initiator_name;
/// }
/// ```
pub struct SMSG_DUEL_WINNER {
    pub reason: DuelWinnerReason,
    pub opponent_name: String,
    pub initiator_name: String,
}

impl WorldServerMessageWrite for SMSG_DUEL_WINNER {
    const OPCODE: u16 = 0x16b;

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
impl WorldMessageBody for SMSG_DUEL_WINNER {
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

