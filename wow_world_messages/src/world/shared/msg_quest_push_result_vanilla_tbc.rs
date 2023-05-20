use std::io::{Read, Write};

use crate::Guid;
use wow_world_base::shared::quest_party_message_vanilla_tbc::QuestPartyMessage;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/msg_quest_push_result.wowm:34`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/msg_quest_push_result.wowm#L34):
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

impl crate::private::Sealed for MSG_QUEST_PUSH_RESULT {}
impl crate::Message for MSG_QUEST_PUSH_RESULT {
    const OPCODE: u32 = 0x0276;

    fn size_without_header(&self) -> u32 {
        9
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // message: QuestPartyMessage
        w.write_all(&(self.message.as_int().to_le_bytes()))?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 9 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0276, size: body_size });
        }

        // guid: Guid
        let guid = Guid::read(&mut r)?;

        // message: QuestPartyMessage
        let message: QuestPartyMessage = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            guid,
            message,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for MSG_QUEST_PUSH_RESULT {}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for MSG_QUEST_PUSH_RESULT {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for MSG_QUEST_PUSH_RESULT {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for MSG_QUEST_PUSH_RESULT {}

