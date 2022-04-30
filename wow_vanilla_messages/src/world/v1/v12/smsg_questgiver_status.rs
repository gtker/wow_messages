use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{QuestGiverStatus, QuestGiverStatusError};
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
#[derive(Copy)]
pub struct SMSG_QUESTGIVER_STATUS {
    pub guid: Guid,
    pub status: QuestGiverStatus,
}

impl ServerMessageWrite for SMSG_QUESTGIVER_STATUS {
    const OPCODE: u16 = 0x183;

    fn size_without_size_field(&self) -> u16 {
        Self::size() as u16
    }

}

impl MessageBody for SMSG_QUESTGIVER_STATUS {
    type Error = SMSG_QUESTGIVER_STATUSError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // status: QuestGiverStatus
        let status = QuestGiverStatus::read_u32_le(r)?;

        Ok(Self {
            guid,
            status,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // status: QuestGiverStatus
        self.status.write_u32_le(w)?;

        Ok(())
    }
}

impl ConstantSized for SMSG_QUESTGIVER_STATUS {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_QUESTGIVER_STATUS {
    fn maximum_possible_size() -> usize {
        8 // guid: Guid
        + 4 // status: QuestGiverStatus upcasted to u32
    }
}

#[derive(Debug)]
pub enum SMSG_QUESTGIVER_STATUSError {
    Io(std::io::Error),
    QuestGiverStatus(QuestGiverStatusError),
}

impl std::error::Error for SMSG_QUESTGIVER_STATUSError {}
impl std::fmt::Display for SMSG_QUESTGIVER_STATUSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::QuestGiverStatus(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_QUESTGIVER_STATUSError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<QuestGiverStatusError> for SMSG_QUESTGIVER_STATUSError {
    fn from(e: QuestGiverStatusError) -> Self {
        Self::QuestGiverStatus(e)
    }
}

