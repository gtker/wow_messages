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
pub struct QuestObjective {
    pub creature_id: u32,
    pub kill_count: u32,
    pub required_item_id: u32,
    pub required_item_count: u32,
}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl ReadableAndWritable for QuestObjective {
    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // creature_id: u32
        let creature_id = crate::util::read_u32_le(r)?;

        // kill_count: u32
        let kill_count = crate::util::read_u32_le(r)?;

        // required_item_id: u32
        let required_item_id = crate::util::read_u32_le(r)?;

        // required_item_count: u32
        let required_item_count = crate::util::read_u32_le(r)?;

        Ok(Self {
            creature_id,
            kill_count,
            required_item_id,
            required_item_count,
        })
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // creature_id: u32
        w.write_all(&self.creature_id.to_le_bytes())?;

        // kill_count: u32
        w.write_all(&self.kill_count.to_le_bytes())?;

        // required_item_id: u32
        w.write_all(&self.required_item_id.to_le_bytes())?;

        // required_item_count: u32
        w.write_all(&self.required_item_count.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // creature_id: u32
        let creature_id = crate::util::tokio_read_u32_le(r).await?;

        // kill_count: u32
        let kill_count = crate::util::tokio_read_u32_le(r).await?;

        // required_item_id: u32
        let required_item_id = crate::util::tokio_read_u32_le(r).await?;

        // required_item_count: u32
        let required_item_count = crate::util::tokio_read_u32_le(r).await?;

        Ok(Self {
            creature_id,
            kill_count,
            required_item_id,
            required_item_count,
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
            // creature_id: u32
            w.write_all(&self.creature_id.to_le_bytes()).await?;

            // kill_count: u32
            w.write_all(&self.kill_count.to_le_bytes()).await?;

            // required_item_id: u32
            w.write_all(&self.required_item_id.to_le_bytes()).await?;

            // required_item_count: u32
            w.write_all(&self.required_item_count.to_le_bytes()).await?;

            Ok(())
        })
    }
    #[cfg(feature = "async_std")]
    async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // creature_id: u32
        let creature_id = crate::util::astd_read_u32_le(r).await?;

        // kill_count: u32
        let kill_count = crate::util::astd_read_u32_le(r).await?;

        // required_item_id: u32
        let required_item_id = crate::util::astd_read_u32_le(r).await?;

        // required_item_count: u32
        let required_item_count = crate::util::astd_read_u32_le(r).await?;

        Ok(Self {
            creature_id,
            kill_count,
            required_item_id,
            required_item_count,
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
            // creature_id: u32
            w.write_all(&self.creature_id.to_le_bytes()).await?;

            // kill_count: u32
            w.write_all(&self.kill_count.to_le_bytes()).await?;

            // required_item_id: u32
            w.write_all(&self.required_item_id.to_le_bytes()).await?;

            // required_item_count: u32
            w.write_all(&self.required_item_count.to_le_bytes()).await?;

            Ok(())
        })
    }
}

impl ConstantSized for QuestObjective {}

impl MaximumPossibleSized for QuestObjective {
    fn maximum_possible_size() -> usize {
        0
        + 4 // creature_id: u32
        + 4 // kill_count: u32
        + 4 // required_item_id: u32
        + 4 // required_item_count: u32
    }
}

