use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::QuestFailedReason;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::Write;

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

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        4
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // msg: QuestFailedReason
        let msg: QuestFailedReason = crate::util::read_u32_le(r)?.try_into()?;

        Ok(Self {
            msg,
        })
    }

}

