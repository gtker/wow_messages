use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SpellCooldownStatus {
    pub id: u32,
    pub cooldown_time_in_msecs: u32,
}

impl ReadableAndWritable for SpellCooldownStatus {
    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // id: u32
        let id = crate::util::read_u32_le(r)?;

        // cooldown_time_in_msecs: u32
        let cooldown_time_in_msecs = crate::util::read_u32_le(r)?;

        Ok(Self {
            id,
            cooldown_time_in_msecs,
        })
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // cooldown_time_in_msecs: u32
        w.write_all(&self.cooldown_time_in_msecs.to_le_bytes())?;

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
            // id: u32
            let id = crate::util::tokio_read_u32_le(r).await?;

            // cooldown_time_in_msecs: u32
            let cooldown_time_in_msecs = crate::util::tokio_read_u32_le(r).await?;

            Ok(Self {
                id,
                cooldown_time_in_msecs,
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
            // id: u32
            w.write_all(&self.id.to_le_bytes()).await?;

            // cooldown_time_in_msecs: u32
            w.write_all(&self.cooldown_time_in_msecs.to_le_bytes()).await?;

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
            // id: u32
            let id = crate::util::astd_read_u32_le(r).await?;

            // cooldown_time_in_msecs: u32
            let cooldown_time_in_msecs = crate::util::astd_read_u32_le(r).await?;

            Ok(Self {
                id,
                cooldown_time_in_msecs,
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
            // id: u32
            w.write_all(&self.id.to_le_bytes()).await?;

            // cooldown_time_in_msecs: u32
            w.write_all(&self.cooldown_time_in_msecs.to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl ConstantSized for SpellCooldownStatus {}

impl MaximumPossibleSized for SpellCooldownStatus {
    fn maximum_possible_size() -> usize {
        0
        + 4 // id: u32
        + 4 // cooldown_time_in_msecs: u32
    }
}

