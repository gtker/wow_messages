use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/quest_common.wowm:174`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/quest_common.wowm#L174):
/// ```text
/// struct QuestItemReward {
///     u32 item;
///     u32 item_count;
/// }
/// ```
pub struct QuestItemReward {
    pub item: u32,
    pub item_count: u32,
}

impl QuestItemReward {
    pub(crate) fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // item: u32
        w.write_all(&self.item.to_le_bytes())?;

        // item_count: u32
        w.write_all(&self.item_count.to_le_bytes())?;

        Ok(())
    }
}

impl QuestItemReward {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> std::result::Result<Self, std::io::Error> {
        // item: u32
        let item = crate::util::read_u32_le(&mut r)?;

        // item_count: u32
        let item_count = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            item,
            item_count,
        })
    }

}

