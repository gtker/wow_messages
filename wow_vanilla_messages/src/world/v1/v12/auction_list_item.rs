use std::convert::{TryFrom, TryInto};
use crate::Guid;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct AuctionListItem {
    pub id: u32,
    pub item_entry: u32,
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
    pub(crate) fn as_bytes(&self) -> Result<[u8; 64], std::io::Error> {
        let mut array_w = [0u8; 64];
        let mut w = array_w.as_mut_slice();
        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // item_entry: u32
        w.write_all(&self.item_entry.to_le_bytes())?;

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

        Ok(array_w)
    }
}

impl AuctionListItem {
    #[cfg(feature = "sync")]
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // id: u32
        let id = crate::util::read_u32_le(r)?;

        // item_entry: u32
        let item_entry = crate::util::read_u32_le(r)?;

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
            item_entry,
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

    #[cfg(feature = "tokio")]
    pub(crate) async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // id: u32
        let id = crate::util::tokio_read_u32_le(r).await?;

        // item_entry: u32
        let item_entry = crate::util::tokio_read_u32_le(r).await?;

        // item_enchantment: u32
        let item_enchantment = crate::util::tokio_read_u32_le(r).await?;

        // item_random_property_id: u32
        let item_random_property_id = crate::util::tokio_read_u32_le(r).await?;

        // item_suffix_factor: u32
        let item_suffix_factor = crate::util::tokio_read_u32_le(r).await?;

        // item_count: u32
        let item_count = crate::util::tokio_read_u32_le(r).await?;

        // item_charges: u32
        let item_charges = crate::util::tokio_read_u32_le(r).await?;

        // item_owner: Guid
        let item_owner = Guid::tokio_read(r).await?;

        // start_bid: u32
        let start_bid = crate::util::tokio_read_u32_le(r).await?;

        // minimum_bid: u32
        let minimum_bid = crate::util::tokio_read_u32_le(r).await?;

        // buyout_amount: u32
        let buyout_amount = crate::util::tokio_read_u32_le(r).await?;

        // time_left_in_msecs: u32
        let time_left_in_msecs = crate::util::tokio_read_u32_le(r).await?;

        // highest_bidder: Guid
        let highest_bidder = Guid::tokio_read(r).await?;

        // highest_bid: u32
        let highest_bid = crate::util::tokio_read_u32_le(r).await?;

        Ok(Self {
            id,
            item_entry,
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

    #[cfg(feature = "async-std")]
    pub(crate) async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // id: u32
        let id = crate::util::astd_read_u32_le(r).await?;

        // item_entry: u32
        let item_entry = crate::util::astd_read_u32_le(r).await?;

        // item_enchantment: u32
        let item_enchantment = crate::util::astd_read_u32_le(r).await?;

        // item_random_property_id: u32
        let item_random_property_id = crate::util::astd_read_u32_le(r).await?;

        // item_suffix_factor: u32
        let item_suffix_factor = crate::util::astd_read_u32_le(r).await?;

        // item_count: u32
        let item_count = crate::util::astd_read_u32_le(r).await?;

        // item_charges: u32
        let item_charges = crate::util::astd_read_u32_le(r).await?;

        // item_owner: Guid
        let item_owner = Guid::astd_read(r).await?;

        // start_bid: u32
        let start_bid = crate::util::astd_read_u32_le(r).await?;

        // minimum_bid: u32
        let minimum_bid = crate::util::astd_read_u32_le(r).await?;

        // buyout_amount: u32
        let buyout_amount = crate::util::astd_read_u32_le(r).await?;

        // time_left_in_msecs: u32
        let time_left_in_msecs = crate::util::astd_read_u32_le(r).await?;

        // highest_bidder: Guid
        let highest_bidder = Guid::astd_read(r).await?;

        // highest_bid: u32
        let highest_bid = crate::util::astd_read_u32_le(r).await?;

        Ok(Self {
            id,
            item_entry,
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

