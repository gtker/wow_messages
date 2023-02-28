use crate::Guid;
use crate::vanilla::QuestGiverStatus;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_questgiver_status.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_questgiver_status.wowm#L3):
/// ```text
/// smsg SMSG_QUESTGIVER_STATUS = 0x0183 {
///     Guid guid;
///     (u32)QuestGiverStatus status;
/// }
/// ```
pub struct SMSG_QUESTGIVER_STATUS {
    pub guid: Guid,
    pub status: QuestGiverStatus,
}

impl crate::Message for SMSG_QUESTGIVER_STATUS {
    const OPCODE: u32 = 0x0183;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // status: QuestGiverStatus
        w.write_all(&u32::from(self.status.as_int()).to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0183, size: body_size as u32 });
        }

        // guid: Guid
        let guid = Guid::read(&mut r)?;

        // status: QuestGiverStatus
        let status: QuestGiverStatus = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

        Ok(Self {
            guid,
            status,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_QUESTGIVER_STATUS {}

