use std::convert::{TryFrom, TryInto};
use crate::helper::Guid;
use crate::world::v1::v12::{QuestPartyMessage, QuestPartyMessageError};
use crate::{WorldClientMessageWrite, WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/msg_quest_push_result.wowm:35`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/msg_quest_push_result.wowm#L35):
/// ```text
/// msg MSG_QUEST_PUSH_RESULT = 0x276 {
///     Guid guid;
///     QuestPartyMessage message;
/// }
/// ```
pub struct MSG_QUEST_PUSH_RESULT {
    pub guid: Guid,
    pub message: QuestPartyMessage,
}

impl WorldClientMessageWrite for MSG_QUEST_PUSH_RESULT {
    const OPCODE: u32 = 0x276;

    fn write_unencrypted_client<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        crate::util::write_u16_be(w, (Self::size() + 4) as u16)?;
        crate::util::write_u32_le(w, <Self as WorldClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_client<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        e.write_encrypted_client_header(w, (Self::size() + 4) as u16, <Self as WorldClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldServerMessageWrite for MSG_QUEST_PUSH_RESULT {
    const OPCODE: u16 = 0x276;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (Self::size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (Self::size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for MSG_QUEST_PUSH_RESULT {
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

