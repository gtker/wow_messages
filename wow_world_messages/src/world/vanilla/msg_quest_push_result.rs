use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::vanilla::QuestPartyMessage;
use crate::world::vanilla::{ClientMessage, ServerMessage};
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/msg_quest_push_result.wowm:35`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/msg_quest_push_result.wowm#L35):
/// ```text
/// msg MSG_QUEST_PUSH_RESULT = 0x0276 {
///     Guid guid;
///     QuestPartyMessage message;
/// }
/// ```
pub struct MSG_QUEST_PUSH_RESULT {
    pub guid: Guid,
    pub message: QuestPartyMessage,
}

impl crate::Message for MSG_QUEST_PUSH_RESULT {
    const OPCODE: u32 = 0x0276;

    fn size_without_header(&self) -> u32 {
        9
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // message: QuestPartyMessage
        w.write_all(&(self.message.as_int() as u8).to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 9 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

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
impl ClientMessage for MSG_QUEST_PUSH_RESULT {}

impl ServerMessage for MSG_QUEST_PUSH_RESULT {}

