use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use crate::AsyncReadWrite;
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};

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

impl ReadableAndWritable for AuctionListItem {
    type Error = std::io::Error;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
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

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
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
        self.item_owner.write(w)?;

        // start_bid: u32
        w.write_all(&self.start_bid.to_le_bytes())?;

        // minimum_bid: u32
        w.write_all(&self.minimum_bid.to_le_bytes())?;

        // buyout_amount: u32
        w.write_all(&self.buyout_amount.to_le_bytes())?;

        // time_left_in_msecs: u32
        w.write_all(&self.time_left_in_msecs.to_le_bytes())?;

        // highest_bidder: Guid
        self.highest_bidder.write(w)?;

        // highest_bid: u32
        w.write_all(&self.highest_bid.to_le_bytes())?;

        Ok(())
    }

}

#[cfg(any(feature = "async_tokio", feature = "async_std"))]
#[async_trait]
impl AsyncReadWrite for AuctionListItem {
    type Error = std::io::Error;

    #[cfg(feature = "async_tokio")]
    async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, Self::Error> {
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

    #[cfg(feature = "async_tokio")]
    async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // id: u32
        w.write_all(&self.id.to_le_bytes()).await?;

        // item_entry: u32
        w.write_all(&self.item_entry.to_le_bytes()).await?;

        // item_enchantment: u32
        w.write_all(&self.item_enchantment.to_le_bytes()).await?;

        // item_random_property_id: u32
        w.write_all(&self.item_random_property_id.to_le_bytes()).await?;

        // item_suffix_factor: u32
        w.write_all(&self.item_suffix_factor.to_le_bytes()).await?;

        // item_count: u32
        w.write_all(&self.item_count.to_le_bytes()).await?;

        // item_charges: u32
        w.write_all(&self.item_charges.to_le_bytes()).await?;

        // item_owner: Guid
        self.item_owner.tokio_write(w).await?;

        // start_bid: u32
        w.write_all(&self.start_bid.to_le_bytes()).await?;

        // minimum_bid: u32
        w.write_all(&self.minimum_bid.to_le_bytes()).await?;

        // buyout_amount: u32
        w.write_all(&self.buyout_amount.to_le_bytes()).await?;

        // time_left_in_msecs: u32
        w.write_all(&self.time_left_in_msecs.to_le_bytes()).await?;

        // highest_bidder: Guid
        self.highest_bidder.tokio_write(w).await?;

        // highest_bid: u32
        w.write_all(&self.highest_bid.to_le_bytes()).await?;

        Ok(())
    }

}

impl ConstantSized for AuctionListItem {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for AuctionListItem {
    fn maximum_possible_size() -> usize {
        4 // id: u32
        + 4 // item_entry: u32
        + 4 // item_enchantment: u32
        + 4 // item_random_property_id: u32
        + 4 // item_suffix_factor: u32
        + 4 // item_count: u32
        + 4 // item_charges: u32
        + 8 // item_owner: Guid
        + 4 // start_bid: u32
        + 4 // minimum_bid: u32
        + 4 // buyout_amount: u32
        + 4 // time_left_in_msecs: u32
        + 8 // highest_bidder: Guid
        + 4 // highest_bid: u32
    }
}

