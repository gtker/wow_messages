use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_list_inventory.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_list_inventory.wowm#L3):
/// ```text
/// struct ListInventoryItem {
///     u32 item_stack_count;
///     u32 item_id;
///     u32 item_display_id;
///     u32 max_items;
///     u32 price;
///     u32 max_durability;
///     u32 durability;
/// }
/// ```
pub struct ListInventoryItem {
    pub item_stack_count: u32,
    pub item_id: u32,
    pub item_display_id: u32,
    /// # Comment
    ///
    /// cmangos: 0 for infinity item amount, although they send 0xFFFFFFFF in that case
    pub max_items: u32,
    pub price: u32,
    pub max_durability: u32,
    pub durability: u32,
}

impl ListInventoryItem {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // item_stack_count: u32
        w.write_all(&self.item_stack_count.to_le_bytes())?;

        // item_id: u32
        w.write_all(&self.item_id.to_le_bytes())?;

        // item_display_id: u32
        w.write_all(&self.item_display_id.to_le_bytes())?;

        // max_items: u32
        w.write_all(&self.max_items.to_le_bytes())?;

        // price: u32
        w.write_all(&self.price.to_le_bytes())?;

        // max_durability: u32
        w.write_all(&self.max_durability.to_le_bytes())?;

        // durability: u32
        w.write_all(&self.durability.to_le_bytes())?;

        Ok(())
    }
}

impl ListInventoryItem {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // item_stack_count: u32
        let item_stack_count = crate::util::read_u32_le(r)?;

        // item_id: u32
        let item_id = crate::util::read_u32_le(r)?;

        // item_display_id: u32
        let item_display_id = crate::util::read_u32_le(r)?;

        // max_items: u32
        let max_items = crate::util::read_u32_le(r)?;

        // price: u32
        let price = crate::util::read_u32_le(r)?;

        // max_durability: u32
        let max_durability = crate::util::read_u32_le(r)?;

        // durability: u32
        let durability = crate::util::read_u32_le(r)?;

        Ok(Self {
            item_stack_count,
            item_id,
            item_display_id,
            max_items,
            price,
            max_durability,
            durability,
        })
    }

}

