use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct Faction {
    pub reputation_list_id: u32,
    pub standing: u32,
}

impl ReadableAndWritable for Faction {
    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // reputation_list_id: u32
        let reputation_list_id = crate::util::read_u32_le(r)?;

        // standing: u32
        let standing = crate::util::read_u32_le(r)?;

        Ok(Self {
            reputation_list_id,
            standing,
        })
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // reputation_list_id: u32
        w.write_all(&self.reputation_list_id.to_le_bytes())?;

        // standing: u32
        w.write_all(&self.standing.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
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
            // reputation_list_id: u32
            let reputation_list_id = crate::util::tokio_read_u32_le(r).await?;

            // standing: u32
            let standing = crate::util::tokio_read_u32_le(r).await?;

            Ok(Self {
                reputation_list_id,
                standing,
            })
        })
    }

    #[cfg(feature = "async_tokio")]
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
            // reputation_list_id: u32
            w.write_all(&self.reputation_list_id.to_le_bytes()).await?;

            // standing: u32
            w.write_all(&self.standing.to_le_bytes()).await?;

            Ok(())
        })
    }

    #[cfg(feature = "async_std")]
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
            // reputation_list_id: u32
            let reputation_list_id = crate::util::astd_read_u32_le(r).await?;

            // standing: u32
            let standing = crate::util::astd_read_u32_le(r).await?;

            Ok(Self {
                reputation_list_id,
                standing,
            })
        })
    }

    #[cfg(feature = "async_std")]
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
            // reputation_list_id: u32
            w.write_all(&self.reputation_list_id.to_le_bytes()).await?;

            // standing: u32
            w.write_all(&self.standing.to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl ConstantSized for Faction {}

impl MaximumPossibleSized for Faction {
    fn maximum_possible_size() -> usize {
        0
        + 4 // reputation_list_id: u32
        + 4 // standing: u32
    }
}

