use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct QuestItemRequirement {
    pub item: u32,
    pub item_count: u32,
    pub item_display_id: u32,
}

impl ReadableAndWritable for QuestItemRequirement {
    type Error = std::io::Error;

    #[cfg(feature = "sync")]
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

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // item: u32
        w.write_all(&self.item.to_le_bytes())?;

        // item_count: u32
        w.write_all(&self.item_count.to_le_bytes())?;

        // item_display_id: u32
        w.write_all(&self.item_display_id.to_le_bytes())?;

        Ok(())
    }

    fn tokio_read<'life0, 'async_trait, R>(
        r: &'life0 mut R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
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
        })
    }

    fn tokio_write<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // item: u32
            w.write_all(&self.item.to_le_bytes()).await?;

            // item_count: u32
            w.write_all(&self.item_count.to_le_bytes()).await?;

            // item_display_id: u32
            w.write_all(&self.item_display_id.to_le_bytes()).await?;

            Ok(())
        })
    }

    fn astd_read<'life0, 'async_trait, R>(
        r: &'life0 mut R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // item: u32
            let item = crate::util::astd_read_u32_le(r).await?;

            // item_count: u32
            let item_count = crate::util::astd_read_u32_le(r).await?;

            // item_display_id: u32
            let item_display_id = crate::util::astd_read_u32_le(r).await?;

            Ok(Self {
                item,
                item_count,
                item_display_id,
            })
        })
    }

    fn astd_write<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // item: u32
            w.write_all(&self.item.to_le_bytes()).await?;

            // item_count: u32
            w.write_all(&self.item_count.to_le_bytes()).await?;

            // item_display_id: u32
            w.write_all(&self.item_display_id.to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl ConstantSized for QuestItemRequirement {}

impl MaximumPossibleSized for QuestItemRequirement {
    fn maximum_possible_size() -> usize {
        0
        + 4 // item: u32
        + 4 // item_count: u32
        + 4 // item_display_id: u32
    }
}

