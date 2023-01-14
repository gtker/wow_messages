use std::convert::{TryFrom, TryInto};
use crate::world::wrath::GuildBankEnchant;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_guild_bank_list.wowm:29`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_guild_bank_list.wowm#L29):
/// ```text
/// struct GuildBankSlot {
///     u8 slot;
///     u32 item;
///     u32 unknown1;
///     u32 item_random_property_id;
///     u32 amount_of_items;
///     u32 unknown2;
///     u8 unknown3;
///     u8 amount_of_enchants;
///     GuildBankEnchant[amount_of_enchants] enchants;
/// }
/// ```
pub struct GuildBankSlot {
    pub slot: u8,
    pub item: u32,
    /// 3.3.0 (0x8000, 0x8020)
    ///
    pub unknown1: u32,
    pub item_random_property_id: u32,
    pub amount_of_items: u32,
    pub unknown2: u32,
    pub unknown3: u8,
    pub enchants: Vec<GuildBankEnchant>,
}

impl GuildBankSlot {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // slot: u8
        w.write_all(&self.slot.to_le_bytes())?;

        // item: u32
        w.write_all(&self.item.to_le_bytes())?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // item_random_property_id: u32
        w.write_all(&self.item_random_property_id.to_le_bytes())?;

        // amount_of_items: u32
        w.write_all(&self.amount_of_items.to_le_bytes())?;

        // unknown2: u32
        w.write_all(&self.unknown2.to_le_bytes())?;

        // unknown3: u8
        w.write_all(&self.unknown3.to_le_bytes())?;

        // amount_of_enchants: u8
        w.write_all(&(self.enchants.len() as u8).to_le_bytes())?;

        // enchants: GuildBankEnchant[amount_of_enchants]
        for i in self.enchants.iter() {
            i.write_into_vec(w)?;
        }

        Ok(())
    }
}

impl GuildBankSlot {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // slot: u8
        let slot = crate::util::read_u8_le(r)?;

        // item: u32
        let item = crate::util::read_u32_le(r)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(r)?;

        // item_random_property_id: u32
        let item_random_property_id = crate::util::read_u32_le(r)?;

        // amount_of_items: u32
        let amount_of_items = crate::util::read_u32_le(r)?;

        // unknown2: u32
        let unknown2 = crate::util::read_u32_le(r)?;

        // unknown3: u8
        let unknown3 = crate::util::read_u8_le(r)?;

        // amount_of_enchants: u8
        let amount_of_enchants = crate::util::read_u8_le(r)?;

        // enchants: GuildBankEnchant[amount_of_enchants]
        let mut enchants = Vec::with_capacity(amount_of_enchants as usize);
        for i in 0..amount_of_enchants {
            enchants.push(GuildBankEnchant::read(r)?);
        }

        Ok(Self {
            slot,
            item,
            unknown1,
            item_random_property_id,
            amount_of_items,
            unknown2,
            unknown3,
            enchants,
        })
    }

}

impl GuildBankSlot {
    pub(crate) fn size(&self) -> usize {
        1 // slot: u8
        + 4 // item: u32
        + 4 // unknown1: u32
        + 4 // item_random_property_id: u32
        + 4 // amount_of_items: u32
        + 4 // unknown2: u32
        + 1 // unknown3: u8
        + 1 // amount_of_enchants: u8
        + self.enchants.len() * 5 // enchants: GuildBankEnchant[amount_of_enchants]
    }
}

