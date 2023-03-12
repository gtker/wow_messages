use std::io::{Read, Write};

use crate::tbc::QuestFailedReason;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_questgiver_questfailed.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_questgiver_questfailed.wowm#L3):
/// ```text
/// smsg SMSG_QUESTGIVER_QUEST_FAILED = 0x0192 {
///     u32 quest_id;
///     QuestFailedReason reason;
/// }
/// ```
pub struct SMSG_QUESTGIVER_QUEST_FAILED {
    pub quest_id: u32,
    pub reason: QuestFailedReason,
}

impl crate::Message for SMSG_QUESTGIVER_QUEST_FAILED {
    const OPCODE: u32 = 0x0192;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        // reason: QuestFailedReason
        w.write_all(&u32::from(self.reason.as_int()).to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0192, size: body_size as u32 });
        }

        // quest_id: u32
        let quest_id = crate::util::read_u32_le(&mut r)?;

        // reason: QuestFailedReason
        let reason: QuestFailedReason = crate::util::read_u32_le(&mut r)?.try_into()?;

        Ok(Self {
            quest_id,
            reason,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_QUESTGIVER_QUEST_FAILED {}

