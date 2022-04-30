use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{QuestPartyMessage, QuestPartyMessageError};
use crate::{ClientMessageWrite, ServerMessageWrite, MessageBody};
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
pub struct MSG_QUEST_PUSH_RESULT {
    pub guid: Guid,
    pub message: QuestPartyMessage,
}

impl ClientMessageWrite for MSG_QUEST_PUSH_RESULT {
    const OPCODE: u16 = 0x276;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

}

impl ServerMessageWrite for MSG_QUEST_PUSH_RESULT {
    const OPCODE: u16 = 0x276;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

}

impl MessageBody for MSG_QUEST_PUSH_RESULT {
    type Error = MSG_QUEST_PUSH_RESULTError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // message: QuestPartyMessage
        let message = QuestPartyMessage::read(r)?;

        Ok(Self {
            guid,
            message,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // message: QuestPartyMessage
        self.message.write(w)?;

        Ok(())
    }
}

impl ConstantSized for MSG_QUEST_PUSH_RESULT {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for MSG_QUEST_PUSH_RESULT {
    fn maximum_possible_size() -> usize {
        8 // guid: Guid
        + QuestPartyMessage::size() // message: QuestPartyMessage
    }
}

#[derive(Debug)]
pub enum MSG_QUEST_PUSH_RESULTError {
    Io(std::io::Error),
    QuestPartyMessage(QuestPartyMessageError),
}

impl std::error::Error for MSG_QUEST_PUSH_RESULTError {}
impl std::fmt::Display for MSG_QUEST_PUSH_RESULTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::QuestPartyMessage(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for MSG_QUEST_PUSH_RESULTError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<QuestPartyMessageError> for MSG_QUEST_PUSH_RESULTError {
    fn from(e: QuestPartyMessageError) -> Self {
        Self::QuestPartyMessage(e)
    }
}

