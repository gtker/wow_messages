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
pub struct ItemDamageType {
    pub damage_minimum: u32,
    pub damage_maximum: u32,
    pub damage_type: u32,
}

impl ReadableAndWritable for ItemDamageType {
    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // damage_minimum: u32
        let damage_minimum = crate::util::read_u32_le(r)?;

        // damage_maximum: u32
        let damage_maximum = crate::util::read_u32_le(r)?;

        // damage_type: u32
        let damage_type = crate::util::read_u32_le(r)?;

        Ok(Self {
            damage_minimum,
            damage_maximum,
            damage_type,
        })
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // damage_minimum: u32
        w.write_all(&self.damage_minimum.to_le_bytes())?;

        // damage_maximum: u32
        w.write_all(&self.damage_maximum.to_le_bytes())?;

        // damage_type: u32
        w.write_all(&self.damage_type.to_le_bytes())?;

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
            // damage_minimum: u32
            let damage_minimum = crate::util::tokio_read_u32_le(r).await?;

            // damage_maximum: u32
            let damage_maximum = crate::util::tokio_read_u32_le(r).await?;

            // damage_type: u32
            let damage_type = crate::util::tokio_read_u32_le(r).await?;

            Ok(Self {
                damage_minimum,
                damage_maximum,
                damage_type,
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
            // damage_minimum: u32
            w.write_all(&self.damage_minimum.to_le_bytes()).await?;

            // damage_maximum: u32
            w.write_all(&self.damage_maximum.to_le_bytes()).await?;

            // damage_type: u32
            w.write_all(&self.damage_type.to_le_bytes()).await?;

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
            // damage_minimum: u32
            let damage_minimum = crate::util::astd_read_u32_le(r).await?;

            // damage_maximum: u32
            let damage_maximum = crate::util::astd_read_u32_le(r).await?;

            // damage_type: u32
            let damage_type = crate::util::astd_read_u32_le(r).await?;

            Ok(Self {
                damage_minimum,
                damage_maximum,
                damage_type,
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
            // damage_minimum: u32
            w.write_all(&self.damage_minimum.to_le_bytes()).await?;

            // damage_maximum: u32
            w.write_all(&self.damage_maximum.to_le_bytes()).await?;

            // damage_type: u32
            w.write_all(&self.damage_type.to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl ConstantSized for ItemDamageType {}

impl MaximumPossibleSized for ItemDamageType {
    fn maximum_possible_size() -> usize {
        0
        + 4 // damage_minimum: u32
        + 4 // damage_maximum: u32
        + 4 // damage_type: u32
    }
}

