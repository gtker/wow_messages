use std::convert::{TryFrom, TryInto};
use crate::world::vanilla::QuestFailedReason;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_questgiver_quest_invalid.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_questgiver_quest_invalid.wowm#L3):
/// ```text
/// smsg SMSG_QUESTGIVER_QUEST_INVALID = 0x018F {
///     QuestFailedReason msg;
/// }
/// ```
pub struct SMSG_QUESTGIVER_QUEST_INVALID {
    pub msg: QuestFailedReason,
}

impl ServerMessage for SMSG_QUESTGIVER_QUEST_INVALID {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // msg: QuestFailedReason
        w.write_all(&(self.msg.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x018f;

    fn server_size(&self) -> u16 {
        8
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // msg: QuestFailedReason
        let msg: QuestFailedReason = crate::util::read_u32_le(r)?.try_into()?;

        Ok(Self {
            msg,
        })
    }

}

