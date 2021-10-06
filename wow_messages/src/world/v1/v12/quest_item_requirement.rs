use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/new5.wowm:595`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/new5.wowm):
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

impl ReadableAndWritable for QuestItemRequirement {
    type Error = std::io::Error;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
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

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // item: u32
        w.write_all(&self.item.to_le_bytes())?;

        // item_count: u32
        w.write_all(&self.item_count.to_le_bytes())?;

        // item_display_id: u32
        w.write_all(&self.item_display_id.to_le_bytes())?;

        Ok(())
    }

}

impl ConstantSized for QuestItemRequirement {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for QuestItemRequirement {
    fn maximum_possible_size() -> usize {
        4 // item: u32
        + 4 // item_count: u32
        + 4 // item_display_id: u32
    }
}

