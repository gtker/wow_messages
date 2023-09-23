use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::AuctionEnchantment;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/auction/auction_common.wowm:49`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/auction/auction_common.wowm#L49):
/// ```text
/// struct AuctionListItem {
///     u32 id;
///     u32 item;
///     AuctionEnchantment[7] enchantments;
///     u32 item_random_property_id;
///     u32 item_suffix_factor;
///     u32 item_count;
///     u32 item_charges;
///     u32 item_flags;
///     Guid item_owner;
///     u32 start_bid;
///     u32 minimum_bid;
///     u32 buyout_amount;
///     Milliseconds time_left;
///     Guid highest_bidder;
///     u32 highest_bid;
/// }
/// ```
pub struct AuctionListItem {
    pub id: u32,
    pub item: u32,
    pub enchantments: [AuctionEnchantment; 7],
    pub item_random_property_id: u32,
    pub item_suffix_factor: u32,
    pub item_count: u32,
    pub item_charges: u32,
    /// mangosone: item flags (dynamic?) (0x04 no lockId?)
    pub item_flags: u32,
    pub item_owner: Guid,
    pub start_bid: u32,
    pub minimum_bid: u32,
    pub buyout_amount: u32,
    pub time_left: Duration,
    pub highest_bidder: Guid,
    pub highest_bid: u32,
}

impl AuctionListItem {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // item: u32
        w.write_all(&self.item.to_le_bytes())?;

        // enchantments: AuctionEnchantment[7]
        for i in self.enchantments.iter() {
            i.write_into_vec(&mut w)?;
        }

        // item_random_property_id: u32
        w.write_all(&self.item_random_property_id.to_le_bytes())?;

        // item_suffix_factor: u32
        w.write_all(&self.item_suffix_factor.to_le_bytes())?;

        // item_count: u32
        w.write_all(&self.item_count.to_le_bytes())?;

        // item_charges: u32
        w.write_all(&self.item_charges.to_le_bytes())?;

        // item_flags: u32
        w.write_all(&self.item_flags.to_le_bytes())?;

        // item_owner: Guid
        w.write_all(&self.item_owner.guid().to_le_bytes())?;

        // start_bid: u32
        w.write_all(&self.start_bid.to_le_bytes())?;

        // minimum_bid: u32
        w.write_all(&self.minimum_bid.to_le_bytes())?;

        // buyout_amount: u32
        w.write_all(&self.buyout_amount.to_le_bytes())?;

        // time_left: Milliseconds
        w.write_all((self.time_left.as_millis() as u32).to_le_bytes().as_slice())?;

        // highest_bidder: Guid
        w.write_all(&self.highest_bidder.guid().to_le_bytes())?;

        // highest_bid: u32
        w.write_all(&self.highest_bid.to_le_bytes())?;

        Ok(())
    }
}

impl AuctionListItem {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, std::io::Error> {
        // id: u32
        let id = crate::util::read_u32_le(&mut r)?;

        // item: u32
        let item = crate::util::read_u32_le(&mut r)?;

        // enchantments: AuctionEnchantment[7]
        let enchantments = {
            let mut enchantments = [AuctionEnchantment::default(); 7];
            for i in enchantments.iter_mut() {
                *i = AuctionEnchantment::read(&mut r)?;
            }
            enchantments
        };

        // item_random_property_id: u32
        let item_random_property_id = crate::util::read_u32_le(&mut r)?;

        // item_suffix_factor: u32
        let item_suffix_factor = crate::util::read_u32_le(&mut r)?;

        // item_count: u32
        let item_count = crate::util::read_u32_le(&mut r)?;

        // item_charges: u32
        let item_charges = crate::util::read_u32_le(&mut r)?;

        // item_flags: u32
        let item_flags = crate::util::read_u32_le(&mut r)?;

        // item_owner: Guid
        let item_owner = crate::util::read_guid(&mut r)?;

        // start_bid: u32
        let start_bid = crate::util::read_u32_le(&mut r)?;

        // minimum_bid: u32
        let minimum_bid = crate::util::read_u32_le(&mut r)?;

        // buyout_amount: u32
        let buyout_amount = crate::util::read_u32_le(&mut r)?;

        // time_left: Milliseconds
        let time_left = Duration::from_millis(crate::util::read_u32_le(&mut r)?.into());

        // highest_bidder: Guid
        let highest_bidder = crate::util::read_guid(&mut r)?;

        // highest_bid: u32
        let highest_bid = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            id,
            item,
            enchantments,
            item_random_property_id,
            item_suffix_factor,
            item_count,
            item_charges,
            item_flags,
            item_owner,
            start_bid,
            minimum_bid,
            buyout_amount,
            time_left,
            highest_bidder,
            highest_bid,
        })
    }

}

