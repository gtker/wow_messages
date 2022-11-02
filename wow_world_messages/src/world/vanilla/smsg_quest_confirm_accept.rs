use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_quest_confirm_accept.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_quest_confirm_accept.wowm#L3):
/// ```text
/// smsg SMSG_QUEST_CONFIRM_ACCEPT = 0x019C {
///     u32 quest_id;
///     CString quest_title;
///     Guid guid;
/// }
/// ```
pub struct SMSG_QUEST_CONFIRM_ACCEPT {
    pub quest_id: u32,
    pub quest_title: String,
    pub guid: Guid,
}

impl crate::Message for SMSG_QUEST_CONFIRM_ACCEPT {
    const OPCODE: u32 = 0x019c;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        // quest_title: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.quest_title.as_bytes().iter().rev().next(), Some(&0_u8), "String `quest_title` must not be null-terminated.");
        w.write_all(self.quest_title.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        assert_eq!(self.size() as usize, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // quest_id: u32
        let quest_id = crate::util::read_u32_le(r)?;

        // quest_title: CString
        let quest_title = crate::util::read_c_string_to_vec(r)?;
        let quest_title = String::from_utf8(quest_title)?;

        // guid: Guid
        let guid = Guid::read(r)?;

        Ok(Self {
            quest_id,
            quest_title,
            guid,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_QUEST_CONFIRM_ACCEPT {}

impl SMSG_QUEST_CONFIRM_ACCEPT {
    pub(crate) fn size(&self) -> usize {
        4 // quest_id: u32
        + self.quest_title.len() + 1 // quest_title: CString
        + 8 // guid: Guid
    }
}

