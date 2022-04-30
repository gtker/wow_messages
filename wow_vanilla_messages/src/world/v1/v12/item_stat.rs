use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
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

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl ReadableAndWritable for ItemStat {
    type Error = std::io::Error;

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

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // item_stat_type: u32
        w.write_all(&self.item_stat_type.to_le_bytes())?;

        // item_stat_value: u32
        w.write_all(&self.item_stat_value.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // item_stat_type: u32
        let item_stat_type = crate::util::tokio_read_u32_le(r).await?;

        // item_stat_value: u32
        let item_stat_value = crate::util::tokio_read_u32_le(r).await?;

        Ok(Self {
            item_stat_type,
            item_stat_value,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // item_stat_type: u32
        w.write_all(&self.item_stat_type.to_le_bytes()).await?;

        // item_stat_value: u32
        w.write_all(&self.item_stat_value.to_le_bytes()).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // item_stat_type: u32
        let item_stat_type = crate::util::astd_read_u32_le(r).await?;

        // item_stat_value: u32
        let item_stat_value = crate::util::astd_read_u32_le(r).await?;

        Ok(Self {
            item_stat_type,
            item_stat_value,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // item_stat_type: u32
        w.write_all(&self.item_stat_type.to_le_bytes()).await?;

        // item_stat_value: u32
        w.write_all(&self.item_stat_value.to_le_bytes()).await?;

        Ok(())
    }

}

impl ConstantSized for ItemStat {}

impl MaximumPossibleSized for ItemStat {
    fn maximum_possible_size() -> usize {
        4 // item_stat_type: u32
        + 4 // item_stat_value: u32
    }
}

