use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct ItemStat {
    pub item_stat_type: u32,
    pub item_stat_value: u32,
}

impl ReadableAndWritable for ItemStat {
    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // item_stat_type: u32
        let item_stat_type = crate::util::read_u32_le(r)?;

        // item_stat_value: u32
        let item_stat_value = crate::util::read_u32_le(r)?;

        Ok(Self {
            item_stat_type,
            item_stat_value,
        })
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // item_stat_type: u32
        w.write_all(&self.item_stat_type.to_le_bytes())?;

        // item_stat_value: u32
        w.write_all(&self.item_stat_value.to_le_bytes())?;

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
            // item_stat_type: u32
            let item_stat_type = crate::util::tokio_read_u32_le(r).await?;

            // item_stat_value: u32
            let item_stat_value = crate::util::tokio_read_u32_le(r).await?;

            Ok(Self {
                item_stat_type,
                item_stat_value,
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
            // item_stat_type: u32
            w.write_all(&self.item_stat_type.to_le_bytes()).await?;

            // item_stat_value: u32
            w.write_all(&self.item_stat_value.to_le_bytes()).await?;

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
            // item_stat_type: u32
            let item_stat_type = crate::util::astd_read_u32_le(r).await?;

            // item_stat_value: u32
            let item_stat_value = crate::util::astd_read_u32_le(r).await?;

            Ok(Self {
                item_stat_type,
                item_stat_value,
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
            // item_stat_type: u32
            w.write_all(&self.item_stat_type.to_le_bytes()).await?;

            // item_stat_value: u32
            w.write_all(&self.item_stat_value.to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl ConstantSized for ItemStat {}

impl MaximumPossibleSized for ItemStat {
    fn maximum_possible_size() -> usize {
        0
        + 4 // item_stat_type: u32
        + 4 // item_stat_value: u32
    }
}

