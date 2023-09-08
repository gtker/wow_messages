use std::io::{Read, Write};

use crate::shared::gold_vanilla_tbc_wrath::Gold;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_list_inventory.wowm:15`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_list_inventory.wowm#L15):
/// ```text
/// struct ListInventoryItem {
///     u32 item_stack_count;
///     u32 item;
///     u32 item_display_id;
///     u32 max_items;
///     Gold price;
///     u32 max_durability;
///     u32 durability;
///     u32 extended_cost;
/// }
/// ```
pub struct ListInventoryItem {
    pub item_stack_count: u32,
    pub item: u32,
    pub item_display_id: u32,
    /// cmangos: 0 for infinity item amount, although they send 0xFFFFFFFF in that case
    pub max_items: u32,
    pub price: Gold,
    pub max_durability: u32,
    pub durability: u32,
    pub extended_cost: u32,
}

impl ListInventoryItem {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // item_stack_count: u32
        w.write_all(&self.item_stack_count.to_le_bytes())?;

        // item: u32
        w.write_all(&self.item.to_le_bytes())?;

        // item_display_id: u32
        w.write_all(&self.item_display_id.to_le_bytes())?;

        // max_items: u32
        w.write_all(&self.max_items.to_le_bytes())?;

        // price: Gold
        w.write_all((self.price.as_int()).to_le_bytes().as_slice())?;

        // max_durability: u32
        w.write_all(&self.max_durability.to_le_bytes())?;

        // durability: u32
        w.write_all(&self.durability.to_le_bytes())?;

        // extended_cost: u32
        w.write_all(&self.extended_cost.to_le_bytes())?;

        Ok(())
    }
}

impl ListInventoryItem {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, std::io::Error> {
        // item_stack_count: u32
        let item_stack_count = crate::util::read_u32_le(&mut r)?;

        // item: u32
        let item = crate::util::read_u32_le(&mut r)?;

        // item_display_id: u32
        let item_display_id = crate::util::read_u32_le(&mut r)?;

        // max_items: u32
        let max_items = crate::util::read_u32_le(&mut r)?;

        // price: Gold
        let price = Gold::new(crate::util::read_u32_le(&mut r)?);

        // max_durability: u32
        let max_durability = crate::util::read_u32_le(&mut r)?;

        // durability: u32
        let durability = crate::util::read_u32_le(&mut r)?;

        // extended_cost: u32
        let extended_cost = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            item_stack_count,
            item,
            item_display_id,
            max_items,
            price,
            max_durability,
            durability,
            extended_cost,
        })
    }

}

