use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/auction/auction_common.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/auction/auction_common.wowm#L1):
/// ```text
/// struct AuctionListItem {
///     u32 id;
///     u32 item;
///     u32 item_enchantment;
///     u32 item_random_property_id;
///     u32 item_suffix_factor;
///     u32 item_count;
///     u32 item_charges;
///     Guid item_owner;
///     u32 start_bid;
///     u32 minimum_bid;
///     u32 buyout_amount;
///     u32 time_left_in_msecs;
///     Guid highest_bidder;
///     u32 highest_bid;
/// }
/// ```
pub struct AuctionListItem {
    pub id: u32,
    pub item: u32,
    pub item_enchantment: u32,
    pub item_random_property_id: u32,
    pub item_suffix_factor: u32,
    pub item_count: u32,
    pub item_charges: u32,
    pub item_owner: Guid,
    pub start_bid: u32,
    pub minimum_bid: u32,
    pub buyout_amount: u32,
    pub time_left_in_msecs: u32,
    pub highest_bidder: Guid,
    pub highest_bid: u32,
}

impl AuctionListItem {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // item: u32
        w.write_all(&self.item.to_le_bytes())?;

        // item_enchantment: u32
        w.write_all(&self.item_enchantment.to_le_bytes())?;

        // item_random_property_id: u32
        w.write_all(&self.item_random_property_id.to_le_bytes())?;

        // item_suffix_factor: u32
        w.write_all(&self.item_suffix_factor.to_le_bytes())?;

        // item_count: u32
        w.write_all(&self.item_count.to_le_bytes())?;

        // item_charges: u32
        w.write_all(&self.item_charges.to_le_bytes())?;

        // item_owner: Guid
        w.write_all(&self.item_owner.guid().to_le_bytes())?;

        // start_bid: u32
        w.write_all(&self.start_bid.to_le_bytes())?;

        // minimum_bid: u32
        w.write_all(&self.minimum_bid.to_le_bytes())?;

        // buyout_amount: u32
        w.write_all(&self.buyout_amount.to_le_bytes())?;

        // time_left_in_msecs: u32
        w.write_all(&self.time_left_in_msecs.to_le_bytes())?;

        // highest_bidder: Guid
        w.write_all(&self.highest_bidder.guid().to_le_bytes())?;

        // highest_bid: u32
        w.write_all(&self.highest_bid.to_le_bytes())?;

        Ok(())
    }
}

impl AuctionListItem {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // id: u32
        let id = crate::util::read_u32_le(r)?;

        // item: u32
        let item = crate::util::read_u32_le(r)?;

        // item_enchantment: u32
        let item_enchantment = crate::util::read_u32_le(r)?;

        // item_random_property_id: u32
        let item_random_property_id = crate::util::read_u32_le(r)?;

        // item_suffix_factor: u32
        let item_suffix_factor = crate::util::read_u32_le(r)?;

        // item_count: u32
        let item_count = crate::util::read_u32_le(r)?;

        // item_charges: u32
        let item_charges = crate::util::read_u32_le(r)?;

        // item_owner: Guid
        let item_owner = Guid::read(r)?;

        // start_bid: u32
        let start_bid = crate::util::read_u32_le(r)?;

        // minimum_bid: u32
        let minimum_bid = crate::util::read_u32_le(r)?;

        // buyout_amount: u32
        let buyout_amount = crate::util::read_u32_le(r)?;

        // time_left_in_msecs: u32
        let time_left_in_msecs = crate::util::read_u32_le(r)?;

        // highest_bidder: Guid
        let highest_bidder = Guid::read(r)?;

        // highest_bid: u32
        let highest_bid = crate::util::read_u32_le(r)?;

        Ok(Self {
            id,
            item,
            item_enchantment,
            item_random_property_id,
            item_suffix_factor,
            item_count,
            item_charges,
            item_owner,
            start_bid,
            minimum_bid,
            buyout_amount,
            time_left_in_msecs,
            highest_bidder,
            highest_bid,
        })
    }

}

