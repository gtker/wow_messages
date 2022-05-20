use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct ListInventoryItem {
    pub item_stack_count: u32,
    pub item_id: u32,
    pub item_display_id: u32,
    pub max_items: u32,
    pub price: u32,
    pub max_durability: u32,
    pub durability: u32,
}

impl ListInventoryItem {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(8000);
        // item_stack_count: u32
        w.write_all(&self.item_stack_count.to_le_bytes())?;

        // item_id: u32
        w.write_all(&self.item_id.to_le_bytes())?;

        // item_display_id: u32
        w.write_all(&self.item_display_id.to_le_bytes())?;

        // max_items: u32
        w.write_all(&self.max_items.to_le_bytes())?;

        // price: u32
        w.write_all(&self.price.to_le_bytes())?;

        // max_durability: u32
        w.write_all(&self.max_durability.to_le_bytes())?;

        // durability: u32
        w.write_all(&self.durability.to_le_bytes())?;

        Ok(w)
    }
}

impl ListInventoryItem {
    #[cfg(feature = "sync")]
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // item_stack_count: u32
        let item_stack_count = crate::util::read_u32_le(r)?;

        // item_id: u32
        let item_id = crate::util::read_u32_le(r)?;

        // item_display_id: u32
        let item_display_id = crate::util::read_u32_le(r)?;

        // max_items: u32
        let max_items = crate::util::read_u32_le(r)?;

        // price: u32
        let price = crate::util::read_u32_le(r)?;

        // max_durability: u32
        let max_durability = crate::util::read_u32_le(r)?;

        // durability: u32
        let durability = crate::util::read_u32_le(r)?;

        Ok(Self {
            item_stack_count,
            item_id,
            item_display_id,
            max_items,
            price,
            max_durability,
            durability,
        })
    }

    #[cfg(feature = "tokio")]
    pub(crate) async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // item_stack_count: u32
        let item_stack_count = crate::util::tokio_read_u32_le(r).await?;

        // item_id: u32
        let item_id = crate::util::tokio_read_u32_le(r).await?;

        // item_display_id: u32
        let item_display_id = crate::util::tokio_read_u32_le(r).await?;

        // max_items: u32
        let max_items = crate::util::tokio_read_u32_le(r).await?;

        // price: u32
        let price = crate::util::tokio_read_u32_le(r).await?;

        // max_durability: u32
        let max_durability = crate::util::tokio_read_u32_le(r).await?;

        // durability: u32
        let durability = crate::util::tokio_read_u32_le(r).await?;

        Ok(Self {
            item_stack_count,
            item_id,
            item_display_id,
            max_items,
            price,
            max_durability,
            durability,
        })
    }

    #[cfg(feature = "async-std")]
    pub(crate) async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // item_stack_count: u32
        let item_stack_count = crate::util::astd_read_u32_le(r).await?;

        // item_id: u32
        let item_id = crate::util::astd_read_u32_le(r).await?;

        // item_display_id: u32
        let item_display_id = crate::util::astd_read_u32_le(r).await?;

        // max_items: u32
        let max_items = crate::util::astd_read_u32_le(r).await?;

        // price: u32
        let price = crate::util::astd_read_u32_le(r).await?;

        // max_durability: u32
        let max_durability = crate::util::astd_read_u32_le(r).await?;

        // durability: u32
        let durability = crate::util::astd_read_u32_le(r).await?;

        Ok(Self {
            item_stack_count,
            item_id,
            item_display_id,
            max_items,
            price,
            max_durability,
            durability,
        })
    }

}

impl ListInventoryItem {
    pub(crate) fn size() -> usize {
        0
        + 4 // item_stack_count: u32
        + 4 // item_id: u32
        + 4 // item_display_id: u32
        + 4 // max_items: u32
        + 4 // price: u32
        + 4 // max_durability: u32
        + 4 // durability: u32
    }
}

