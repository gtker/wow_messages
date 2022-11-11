use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_quest_query_response.wowm:12`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_quest_query_response.wowm#L12):
/// ```text
/// struct QuestItemRequirement {
///     u32 item;
///     u32 count;
/// }
/// ```
pub struct QuestItemRequirement {
    pub item: u32,
    pub count: u32,
}

impl QuestItemRequirement {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // item: u32
        w.write_all(&self.item.to_le_bytes())?;

        // count: u32
        w.write_all(&self.count.to_le_bytes())?;

        Ok(())
    }
}

impl QuestItemRequirement {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // item: u32
        let item = crate::util::read_u32_le(r)?;

        // count: u32
        let count = crate::util::read_u32_le(r)?;

        Ok(Self {
            item,
            count,
        })
    }

}

