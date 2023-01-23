use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/cmsg_questgiver_complete_quest.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/cmsg_questgiver_complete_quest.wowm#L3):
/// ```text
/// cmsg CMSG_QUESTGIVER_COMPLETE_QUEST = 0x018A {
///     Guid guid;
///     u32 quest_id;
/// }
/// ```
pub struct CMSG_QUESTGIVER_COMPLETE_QUEST {
    pub guid: Guid,
    pub quest_id: u32,
}

impl crate::Message for CMSG_QUESTGIVER_COMPLETE_QUEST {
    const OPCODE: u32 = 0x018a;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x018A, size: body_size as u32 });
        }

        // guid: Guid
        let guid = Guid::read(r)?;

        // quest_id: u32
        let quest_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            guid,
            quest_id,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_QUESTGIVER_COMPLETE_QUEST {}

#[cfg(feature = "tbc")]
impl crate::world::tbc::ClientMessage for CMSG_QUESTGIVER_COMPLETE_QUEST {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_QUESTGIVER_COMPLETE_QUEST {}

