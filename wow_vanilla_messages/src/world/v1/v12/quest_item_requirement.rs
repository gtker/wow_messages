use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use crate::AsyncReadWrite;
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct QuestItemRequirement {
    pub item: u32,
    pub item_count: u32,
    pub item_display_id: u32,
}

impl ReadableAndWritable for QuestItemRequirement {
    type Error = std::io::Error;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // item: u32
        let item = crate::util::read_u32_le(r)?;

        // item_count: u32
        let item_count = crate::util::read_u32_le(r)?;

        // item_display_id: u32
        let item_display_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            item,
            item_count,
            item_display_id,
        })
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // item: u32
        w.write_all(&self.item.to_le_bytes())?;

        // item_count: u32
        w.write_all(&self.item_count.to_le_bytes())?;

        // item_display_id: u32
        w.write_all(&self.item_display_id.to_le_bytes())?;

        Ok(())
    }

}

#[cfg(any(feature = "async_tokio", feature = "async_std"))]
#[async_trait]
impl AsyncReadWrite for QuestItemRequirement {
    type Error = std::io::Error;
    #[cfg(feature = "async_tokio")]
    async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> Result<Self, Self::Error> {
        // item: u32
        let item = crate::util::tokio_read_u32_le(r).await?;

        // item_count: u32
        let item_count = crate::util::tokio_read_u32_le(r).await?;

        // item_display_id: u32
        let item_display_id = crate::util::tokio_read_u32_le(r).await?;

        Ok(Self {
            item,
            item_count,
            item_display_id,
        })
    }
    #[cfg(feature = "async_tokio")]
    async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> Result<(), std::io::Error> {
        // item: u32
        w.write_all(&self.item.to_le_bytes()).await?;

        // item_count: u32
        w.write_all(&self.item_count.to_le_bytes()).await?;

        // item_display_id: u32
        w.write_all(&self.item_display_id.to_le_bytes()).await?;

        Ok(())
    }
}
impl ConstantSized for QuestItemRequirement {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for QuestItemRequirement {
    fn maximum_possible_size() -> usize {
        4 // item: u32
        + 4 // item_count: u32
        + 4 // item_display_id: u32
    }
}

