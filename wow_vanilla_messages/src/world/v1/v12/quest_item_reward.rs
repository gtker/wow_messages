use std::convert::{TryFrom, TryInto};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct QuestItemReward {
    pub item: u32,
    pub item_count: u32,
}

impl QuestItemReward {
    #[cfg(feature = "sync")]
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // item: u32
        let item = crate::util::read_u32_le(r)?;

        // item_count: u32
        let item_count = crate::util::read_u32_le(r)?;

        Ok(Self {
            item,
            item_count,
        })
    }

    #[cfg(feature = "sync")]
    pub(crate) fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // item: u32
        w.write_all(&self.item.to_le_bytes())?;

        // item_count: u32
        w.write_all(&self.item_count.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub(crate) async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // item: u32
        let item = crate::util::tokio_read_u32_le(r).await?;

        // item_count: u32
        let item_count = crate::util::tokio_read_u32_le(r).await?;

        Ok(Self {
            item,
            item_count,
        })
    }

    #[cfg(feature = "async_tokio")]
    pub(crate) async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // item: u32
        w.write_all(&self.item.to_le_bytes()).await?;

        // item_count: u32
        w.write_all(&self.item_count.to_le_bytes()).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub(crate) async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // item: u32
        let item = crate::util::astd_read_u32_le(r).await?;

        // item_count: u32
        let item_count = crate::util::astd_read_u32_le(r).await?;

        Ok(Self {
            item,
            item_count,
        })
    }

    #[cfg(feature = "async_std")]
    pub(crate) async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // item: u32
        w.write_all(&self.item.to_le_bytes()).await?;

        // item_count: u32
        w.write_all(&self.item_count.to_le_bytes()).await?;

        Ok(())
    }

}

impl QuestItemReward {
    pub(crate) fn size() -> usize {
        0
        + 4 // item: u32
        + 4 // item_count: u32
    }
}

