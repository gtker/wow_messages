use crate:: {
};
use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_quest_force_remove.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_quest_force_remove.wowm#L1):
/// ```text
/// smsg SMSG_QUEST_FORCE_REMOVE = 0x021E {
///     u32 quest_id;
/// }
/// ```
pub struct SMSG_QUEST_FORCE_REMOVE {
    pub quest_id: u32,
}

impl crate::Message for SMSG_QUEST_FORCE_REMOVE {
    const OPCODE: u32 = 0x021e;

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
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x021E, size: body_size as u32 });
        }

        // quest_id: u32
        let quest_id = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            quest_id,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_QUEST_FORCE_REMOVE {}

