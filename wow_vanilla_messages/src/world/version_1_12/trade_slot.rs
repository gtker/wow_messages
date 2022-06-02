use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/trade/smsg_trade_status_extended.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/trade/smsg_trade_status_extended.wowm#L3):
/// ```text
/// struct TradeSlot {
///     u8 trade_slot_number;
///     u32 item_id;
///     u32 display_id;
///     u32 stack_count;
///     u32 is_wrapped;
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
pub struct TradeSlot {
    /// cmangos/vmangos/mangoszero: sets to index of array
    ///
    pub trade_slot_number: u8,
    pub item_id: u32,
    pub display_id: u32,
    pub stack_count: u32,
    pub is_wrapped: u32,
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
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // trade_slot_number: u8
        w.write_all(&self.trade_slot_number.to_le_bytes())?;

        // item_id: u32
        w.write_all(&self.item_id.to_le_bytes())?;

        // display_id: u32
        w.write_all(&self.display_id.to_le_bytes())?;

        // stack_count: u32
        w.write_all(&self.stack_count.to_le_bytes())?;

        // is_wrapped: u32
        w.write_all(&self.is_wrapped.to_le_bytes())?;

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
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // trade_slot_number: u8
        let trade_slot_number = crate::util::read_u8_le(r)?;

        // item_id: u32
        let item_id = crate::util::read_u32_le(r)?;

        // display_id: u32
        let display_id = crate::util::read_u32_le(r)?;

        // stack_count: u32
        let stack_count = crate::util::read_u32_le(r)?;

        // is_wrapped: u32
        let is_wrapped = crate::util::read_u32_le(r)?;

        // gift_wrapper: Guid
        let gift_wrapper = Guid::read(r)?;

        // enchantment: u32
        let enchantment = crate::util::read_u32_le(r)?;

        // item_creator: Guid
        let item_creator = Guid::read(r)?;

        // spell_charges: u32
        let spell_charges = crate::util::read_u32_le(r)?;

        // item_suffix_factor: u32
        let item_suffix_factor = crate::util::read_u32_le(r)?;

        // item_random_properties_id: u32
        let item_random_properties_id = crate::util::read_u32_le(r)?;

        // lock_id: u32
        let lock_id = crate::util::read_u32_le(r)?;

        // max_durability: u32
        let max_durability = crate::util::read_u32_le(r)?;

        // durability: u32
        let durability = crate::util::read_u32_le(r)?;

        Ok(Self {
            trade_slot_number,
            item_id,
            display_id,
            stack_count,
            is_wrapped,
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

