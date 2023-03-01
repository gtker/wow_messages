use crate:: {
};
use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_questupdate_failed.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_questupdate_failed.wowm#L3):
/// ```text
/// smsg SMSG_QUESTUPDATE_FAILED = 0x0196 {
///     u32 quest_id;
/// }
/// ```
pub struct SMSG_QUESTUPDATE_FAILED {
    pub quest_id: u32,
}

impl crate::Message for SMSG_QUESTUPDATE_FAILED {
    const OPCODE: u32 = 0x0196;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0196, size: body_size as u32 });
        }

        // quest_id: u32
        let quest_id = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            quest_id,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_QUESTUPDATE_FAILED {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_QUESTUPDATE_FAILED {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_QUESTUPDATE_FAILED {}

