use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::QuestGiverStatus;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_questgiver_status_multiple.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_questgiver_status_multiple.wowm#L1):
/// ```text
/// struct QuestGiverStatusReport {
///     Guid npc;
///     QuestGiverStatus dialog_status;
/// }
/// ```
pub struct QuestGiverStatusReport {
    pub npc: Guid,
    pub dialog_status: QuestGiverStatus,
}

impl QuestGiverStatusReport {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // npc: Guid
        w.write_all(&self.npc.guid().to_le_bytes())?;

        // dialog_status: QuestGiverStatus
        w.write_all(&(self.dialog_status.as_int().to_le_bytes()))?;

        Ok(())
    }
}

impl QuestGiverStatusReport {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // npc: Guid
        let npc = crate::util::read_guid(&mut r)?;

        // dialog_status: QuestGiverStatus
        let dialog_status = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            npc,
            dialog_status,
        })
    }

}

