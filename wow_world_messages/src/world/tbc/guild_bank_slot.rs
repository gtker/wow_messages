use std::io::{Read, Write};

use crate::tbc::{
    GuildBankSocket, VariableItemRandomProperty,
};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild_bank/smsg_guild_bank_list.wowm:29`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild_bank/smsg_guild_bank_list.wowm#L29):
/// ```text
/// struct GuildBankSlot {
///     u8 slot;
///     Item item;
///     VariableItemRandomProperty item_random_property_id;
///     u8 amount_of_items;
///     u32 enchant;
///     u8 charges;
///     u8 amount_of_sockets;
///     GuildBankSocket[amount_of_sockets] sockets;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct GuildBankSlot {
    pub slot: u8,
    pub item: u32,
    pub item_random_property_id: VariableItemRandomProperty,
    pub amount_of_items: u8,
    pub enchant: u32,
    pub charges: u8,
    pub sockets: Vec<GuildBankSocket>,
}

impl GuildBankSlot {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // slot: u8
        w.write_all(&self.slot.to_le_bytes())?;

        // item: Item
        w.write_all(&self.item.to_le_bytes())?;

        // item_random_property_id: VariableItemRandomProperty
        self.item_random_property_id.write_into_vec(&mut w)?;

        // amount_of_items: u8
        w.write_all(&self.amount_of_items.to_le_bytes())?;

        // enchant: u32
        w.write_all(&self.enchant.to_le_bytes())?;

        // charges: u8
        w.write_all(&self.charges.to_le_bytes())?;

        // amount_of_sockets: u8
        w.write_all(&(self.sockets.len() as u8).to_le_bytes())?;

        // sockets: GuildBankSocket[amount_of_sockets]
        for i in self.sockets.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }
}

impl GuildBankSlot {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, std::io::Error> {
        // slot: u8
        let slot = crate::util::read_u8_le(&mut r)?;

        // item: Item
        let item = crate::util::read_u32_le(&mut r)?;

        // item_random_property_id: VariableItemRandomProperty
        let item_random_property_id = VariableItemRandomProperty::read(&mut r)?;

        // amount_of_items: u8
        let amount_of_items = crate::util::read_u8_le(&mut r)?;

        // enchant: u32
        let enchant = crate::util::read_u32_le(&mut r)?;

        // charges: u8
        let charges = crate::util::read_u8_le(&mut r)?;

        // amount_of_sockets: u8
        let amount_of_sockets = crate::util::read_u8_le(&mut r)?;

        // sockets: GuildBankSocket[amount_of_sockets]
        let sockets = {
            let mut sockets = Vec::with_capacity(amount_of_sockets as usize);
            for _ in 0..amount_of_sockets {
                sockets.push(GuildBankSocket::read(&mut r)?);
            }
            sockets
        };

        Ok(Self {
            slot,
            item,
            item_random_property_id,
            amount_of_items,
            enchant,
            charges,
            sockets,
        })
    }

}

impl GuildBankSlot {
    pub(crate) fn size(&self) -> usize {
        1 // slot: u8
        + 4 // item: Item
        + self.item_random_property_id.size() // item_random_property_id: VariableItemRandomProperty
        + 1 // amount_of_items: u8
        + 4 // enchant: u32
        + 1 // charges: u8
        + 1 // amount_of_sockets: u8
        + self.sockets.len() * 5 // sockets: GuildBankSocket[amount_of_sockets]
    }
}

