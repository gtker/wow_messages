use std::convert::{TryFrom, TryInto};
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_questgiver_request_item.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_questgiver_request_item.wowm#L3):
/// ```text
/// struct QuestItemRequirement {
///     u32 item;
///     u32 item_count;
///     u32 item_display_id;
/// }
/// ```
pub struct QuestItemRequirement {
    pub item: u32,
    pub item_count: u32,
    pub item_display_id: u32,
}

impl QuestItemRequirement {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // item: u32
        w.write_all(&self.item.to_le_bytes())?;

        // item_count: u32
        w.write_all(&self.item_count.to_le_bytes())?;

        // item_display_id: u32
        w.write_all(&self.item_display_id.to_le_bytes())?;

        Ok(())
    }
}

impl QuestItemRequirement {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // item: u32
        let item = crate::util::read_u32_le(r)?;

        // item_count: u32
        let item_count = crate::util::read_u32_le(r)?;

        // item_display_id: u32
        let item_display_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            item,
            item_count,
            item_display_id,
        })
    }

}

