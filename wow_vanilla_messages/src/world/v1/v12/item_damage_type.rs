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

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
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

    #[cfg(feature = "async_tokio")]
    async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, Self::Error> {
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
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // damage_minimum: u32
        w.write_all(&self.damage_minimum.to_le_bytes()).await?;

        // damage_maximum: u32
        w.write_all(&self.damage_maximum.to_le_bytes()).await?;

        // damage_type: u32
        w.write_all(&self.damage_type.to_le_bytes()).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, Self::Error> {
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
    }

    #[cfg(feature = "async_std")]
    async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // damage_minimum: u32
        w.write_all(&self.damage_minimum.to_le_bytes()).await?;

        // damage_maximum: u32
        w.write_all(&self.damage_maximum.to_le_bytes()).await?;

        // damage_type: u32
        w.write_all(&self.damage_type.to_le_bytes()).await?;

        Ok(())
    }

}

impl ConstantSized for ItemDamageType {}

impl MaximumPossibleSized for ItemDamageType {
    fn maximum_possible_size() -> usize {
        4 // damage_minimum: u32
        + 4 // damage_maximum: u32
        + 4 // damage_type: u32
    }
}

