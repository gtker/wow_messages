use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_questgiver_request_item.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_questgiver_request_item.wowm#L1):
/// ```text
/// struct QuestItemRequirement {
///     Item item;
///     u32 item_count;
///     u32 item_display_id;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct QuestItemRequirement {
    pub item: u32,
    pub item_count: u32,
    pub item_display_id: u32,
}

impl QuestItemRequirement {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // item: Item
        w.write_all(&self.item.to_le_bytes())?;

        // item_count: u32
        w.write_all(&self.item_count.to_le_bytes())?;

        // item_display_id: u32
        w.write_all(&self.item_display_id.to_le_bytes())?;

        Ok(())
    }
}

impl QuestItemRequirement {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, std::io::Error> {
        // item: Item
        let item = crate::util::read_u32_le(&mut r)?;

        // item_count: u32
        let item_count = crate::util::read_u32_le(&mut r)?;

        // item_display_id: u32
        let item_display_id = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            item,
            item_count,
            item_display_id,
        })
    }

}

