use std::io::{Read, Write};

use crate::wrath::LootSlotType;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/smsg_loot_response.wowm:73`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/smsg_loot_response.wowm#L73):
/// ```text
/// struct LootItem {
///     u8 index;
///     Item item;
///     u32 count;
///     u32 display_id;
///     u32 random_suffix;
///     i32 random_property;
///     LootSlotType ty;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct LootItem {
    pub index: u8,
    pub item: u32,
    pub count: u32,
    pub display_id: u32,
    pub random_suffix: u32,
    pub random_property: i32,
    pub ty: LootSlotType,
}
impl LootItem {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // index: u8
        w.write_all(&self.index.to_le_bytes())?;

        // item: Item
        w.write_all(&self.item.to_le_bytes())?;

        // count: u32
        w.write_all(&self.count.to_le_bytes())?;

        // display_id: u32
        w.write_all(&self.display_id.to_le_bytes())?;

        // random_suffix: u32
        w.write_all(&self.random_suffix.to_le_bytes())?;

        // random_property: i32
        w.write_all(&self.random_property.to_le_bytes())?;

        // ty: LootSlotType
        w.write_all(&(self.ty.as_int().to_le_bytes()))?;

        Ok(())
    }
}

impl LootItem {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // index: u8
        let index = crate::util::read_u8_le(&mut r)?;

        // item: Item
        let item = crate::util::read_u32_le(&mut r)?;

        // count: u32
        let count = crate::util::read_u32_le(&mut r)?;

        // display_id: u32
        let display_id = crate::util::read_u32_le(&mut r)?;

        // random_suffix: u32
        let random_suffix = crate::util::read_u32_le(&mut r)?;

        // random_property: i32
        let random_property = crate::util::read_i32_le(&mut r)?;

        // ty: LootSlotType
        let ty = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            index,
            item,
            count,
            display_id,
            random_suffix,
            random_property,
            ty,
        })
    }

}
