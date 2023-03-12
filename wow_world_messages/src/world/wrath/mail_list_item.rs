use std::io::{Read, Write};
use crate::wrath::MailListItemEnchant;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/mail/smsg_mail_list_result.wowm:160`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/mail/smsg_mail_list_result.wowm#L160):
/// ```text
/// struct MailListItem {
///     u8 item_index;
///     u32 low_guid;
///     u32 item;
///     MailListItemEnchant[7] enchants;
///     u32 item_random_property_id;
///     u32 item_suffix_factor;
///     u8 item_amount;
///     u32 charges;
///     u32 max_durability;
///     u32 durability;
///     u8 unknown;
/// }
/// ```
pub struct MailListItem {
    pub item_index: u8,
    pub low_guid: u32,
    pub item: u32,
    pub enchants: [MailListItemEnchant; 7],
    pub item_random_property_id: u32,
    pub item_suffix_factor: u32,
    pub item_amount: u8,
    pub charges: u32,
    pub max_durability: u32,
    pub durability: u32,
    pub unknown: u8,
}

impl MailListItem {
    pub(crate) fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // item_index: u8
        w.write_all(&self.item_index.to_le_bytes())?;

        // low_guid: u32
        w.write_all(&self.low_guid.to_le_bytes())?;

        // item: u32
        w.write_all(&self.item.to_le_bytes())?;

        // enchants: MailListItemEnchant[7]
        for i in self.enchants.iter() {
            i.write_into_vec(&mut w)?;
        }

        // item_random_property_id: u32
        w.write_all(&self.item_random_property_id.to_le_bytes())?;

        // item_suffix_factor: u32
        w.write_all(&self.item_suffix_factor.to_le_bytes())?;

        // item_amount: u8
        w.write_all(&self.item_amount.to_le_bytes())?;

        // charges: u32
        w.write_all(&self.charges.to_le_bytes())?;

        // max_durability: u32
        w.write_all(&self.max_durability.to_le_bytes())?;

        // durability: u32
        w.write_all(&self.durability.to_le_bytes())?;

        // unknown: u8
        w.write_all(&self.unknown.to_le_bytes())?;

        Ok(())
    }
}

impl MailListItem {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> std::result::Result<Self, std::io::Error> {
        // item_index: u8
        let item_index = crate::util::read_u8_le(&mut r)?;

        // low_guid: u32
        let low_guid = crate::util::read_u32_le(&mut r)?;

        // item: u32
        let item = crate::util::read_u32_le(&mut r)?;

        // enchants: MailListItemEnchant[7]
        let enchants = {
            let mut enchants = [MailListItemEnchant::default(); 7];
            for i in enchants.iter_mut() {
                *i = MailListItemEnchant::read(&mut r)?;
            }
            enchants
        };

        // item_random_property_id: u32
        let item_random_property_id = crate::util::read_u32_le(&mut r)?;

        // item_suffix_factor: u32
        let item_suffix_factor = crate::util::read_u32_le(&mut r)?;

        // item_amount: u8
        let item_amount = crate::util::read_u8_le(&mut r)?;

        // charges: u32
        let charges = crate::util::read_u32_le(&mut r)?;

        // max_durability: u32
        let max_durability = crate::util::read_u32_le(&mut r)?;

        // durability: u32
        let durability = crate::util::read_u32_le(&mut r)?;

        // unknown: u8
        let unknown = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            item_index,
            low_guid,
            item,
            enchants,
            item_random_property_id,
            item_suffix_factor,
            item_amount,
            charges,
            max_durability,
            durability,
            unknown,
        })
    }

}

