use std::io::{Read, Write};

use crate::Guid;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/trade/smsg_trade_status_extended.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/trade/smsg_trade_status_extended.wowm#L1):
/// ```text
/// struct TradeSlot {
///     u8 trade_slot_number;
///     Item item;
///     u32 display_id;
///     u32 stack_count;
///     Bool32 wrapped;
///     Guid gift_wrapper;
///     u32 enchantment;
///     Guid item_creator;
///     u32 spell_charges;
///     u32 item_suffix_factor;
///     u32 item_random_properties_id;
///     u32 lock_id;
///     u32 max_durability;
///     u32 durability;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct TradeSlot {
    /// cmangos/vmangos/mangoszero: sets to index of array
    pub trade_slot_number: u8,
    pub item: u32,
    pub display_id: u32,
    pub stack_count: u32,
    pub wrapped: bool,
    pub gift_wrapper: Guid,
    pub enchantment: u32,
    pub item_creator: Guid,
    pub spell_charges: u32,
    pub item_suffix_factor: u32,
    pub item_random_properties_id: u32,
    pub lock_id: u32,
    pub max_durability: u32,
    pub durability: u32,
}

impl TradeSlot {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // trade_slot_number: u8
        w.write_all(&self.trade_slot_number.to_le_bytes())?;

        // item: Item
        w.write_all(&self.item.to_le_bytes())?;

        // display_id: u32
        w.write_all(&self.display_id.to_le_bytes())?;

        // stack_count: u32
        w.write_all(&self.stack_count.to_le_bytes())?;

        // wrapped: Bool32
        w.write_all(u32::from(self.wrapped).to_le_bytes().as_slice())?;

        // gift_wrapper: Guid
        w.write_all(&self.gift_wrapper.guid().to_le_bytes())?;

        // enchantment: u32
        w.write_all(&self.enchantment.to_le_bytes())?;

        // item_creator: Guid
        w.write_all(&self.item_creator.guid().to_le_bytes())?;

        // spell_charges: u32
        w.write_all(&self.spell_charges.to_le_bytes())?;

        // item_suffix_factor: u32
        w.write_all(&self.item_suffix_factor.to_le_bytes())?;

        // item_random_properties_id: u32
        w.write_all(&self.item_random_properties_id.to_le_bytes())?;

        // lock_id: u32
        w.write_all(&self.lock_id.to_le_bytes())?;

        // max_durability: u32
        w.write_all(&self.max_durability.to_le_bytes())?;

        // durability: u32
        w.write_all(&self.durability.to_le_bytes())?;

        Ok(())
    }
}

impl TradeSlot {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, std::io::Error> {
        // trade_slot_number: u8
        let trade_slot_number = crate::util::read_u8_le(&mut r)?;

        // item: Item
        let item = crate::util::read_u32_le(&mut r)?;

        // display_id: u32
        let display_id = crate::util::read_u32_le(&mut r)?;

        // stack_count: u32
        let stack_count = crate::util::read_u32_le(&mut r)?;

        // wrapped: Bool32
        let wrapped = crate::util::read_bool_u32(&mut r)?;

        // gift_wrapper: Guid
        let gift_wrapper = crate::util::read_guid(&mut r)?;

        // enchantment: u32
        let enchantment = crate::util::read_u32_le(&mut r)?;

        // item_creator: Guid
        let item_creator = crate::util::read_guid(&mut r)?;

        // spell_charges: u32
        let spell_charges = crate::util::read_u32_le(&mut r)?;

        // item_suffix_factor: u32
        let item_suffix_factor = crate::util::read_u32_le(&mut r)?;

        // item_random_properties_id: u32
        let item_random_properties_id = crate::util::read_u32_le(&mut r)?;

        // lock_id: u32
        let lock_id = crate::util::read_u32_le(&mut r)?;

        // max_durability: u32
        let max_durability = crate::util::read_u32_le(&mut r)?;

        // durability: u32
        let durability = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            trade_slot_number,
            item,
            display_id,
            stack_count,
            wrapped,
            gift_wrapper,
            enchantment,
            item_creator,
            spell_charges,
            item_suffix_factor,
            item_random_properties_id,
            lock_id,
            max_durability,
            durability,
        })
    }

}

