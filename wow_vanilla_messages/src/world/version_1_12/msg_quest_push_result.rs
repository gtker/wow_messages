use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::version_1_12::QuestPartyMessage;
use crate::{ClientMessage, ServerMessage};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct MSG_QUEST_PUSH_RESULT {
    pub guid: Guid,
    pub message: QuestPartyMessage,
}

impl ClientMessage for MSG_QUEST_PUSH_RESULT {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // message: QuestPartyMessage
        w.write_all(&(self.message.as_int() as u8).to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0276;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        9
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // message: QuestPartyMessage
        let message: QuestPartyMessage = crate::util::read_u8_le(r)?.try_into()?;

        Ok(Self {
            guid,
            message,
        })
    }

}

impl ServerMessage for MSG_QUEST_PUSH_RESULT {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // message: QuestPartyMessage
        w.write_all(&(self.message.as_int() as u8).to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0276;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        9
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // message: QuestPartyMessage
        let message: QuestPartyMessage = crate::util::read_u8_le(r)?.try_into()?;

        Ok(Self {
            guid,
            message,
        })
    }

}

