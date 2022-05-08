use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct PetSpellCooldown {
    pub spell_id: u16,
    pub spell_category: u16,
    pub cooldown_in_msecs: u32,
    pub category_cooldown_in_msecs: u32,
}

impl ReadableAndWritable for PetSpellCooldown {
    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // spell_id: u16
        let spell_id = crate::util::read_u16_le(r)?;

        // spell_category: u16
        let spell_category = crate::util::read_u16_le(r)?;

        // cooldown_in_msecs: u32
        let cooldown_in_msecs = crate::util::read_u32_le(r)?;

        // category_cooldown_in_msecs: u32
        let category_cooldown_in_msecs = crate::util::read_u32_le(r)?;

        Ok(Self {
            spell_id,
            spell_category,
            cooldown_in_msecs,
            category_cooldown_in_msecs,
        })
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // spell_id: u16
        w.write_all(&self.spell_id.to_le_bytes())?;

        // spell_category: u16
        w.write_all(&self.spell_category.to_le_bytes())?;

        // cooldown_in_msecs: u32
        w.write_all(&self.cooldown_in_msecs.to_le_bytes())?;

        // category_cooldown_in_msecs: u32
        w.write_all(&self.category_cooldown_in_msecs.to_le_bytes())?;

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
            // spell_id: u16
            let spell_id = crate::util::tokio_read_u16_le(r).await?;

            // spell_category: u16
            let spell_category = crate::util::tokio_read_u16_le(r).await?;

            // cooldown_in_msecs: u32
            let cooldown_in_msecs = crate::util::tokio_read_u32_le(r).await?;

            // category_cooldown_in_msecs: u32
            let category_cooldown_in_msecs = crate::util::tokio_read_u32_le(r).await?;

            Ok(Self {
                spell_id,
                spell_category,
                cooldown_in_msecs,
                category_cooldown_in_msecs,
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
            // spell_id: u16
            w.write_all(&self.spell_id.to_le_bytes()).await?;

            // spell_category: u16
            w.write_all(&self.spell_category.to_le_bytes()).await?;

            // cooldown_in_msecs: u32
            w.write_all(&self.cooldown_in_msecs.to_le_bytes()).await?;

            // category_cooldown_in_msecs: u32
            w.write_all(&self.category_cooldown_in_msecs.to_le_bytes()).await?;

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
            // spell_id: u16
            let spell_id = crate::util::astd_read_u16_le(r).await?;

            // spell_category: u16
            let spell_category = crate::util::astd_read_u16_le(r).await?;

            // cooldown_in_msecs: u32
            let cooldown_in_msecs = crate::util::astd_read_u32_le(r).await?;

            // category_cooldown_in_msecs: u32
            let category_cooldown_in_msecs = crate::util::astd_read_u32_le(r).await?;

            Ok(Self {
                spell_id,
                spell_category,
                cooldown_in_msecs,
                category_cooldown_in_msecs,
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
            // spell_id: u16
            w.write_all(&self.spell_id.to_le_bytes()).await?;

            // spell_category: u16
            w.write_all(&self.spell_category.to_le_bytes()).await?;

            // cooldown_in_msecs: u32
            w.write_all(&self.cooldown_in_msecs.to_le_bytes()).await?;

            // category_cooldown_in_msecs: u32
            w.write_all(&self.category_cooldown_in_msecs.to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl ConstantSized for PetSpellCooldown {}

impl MaximumPossibleSized for PetSpellCooldown {
    fn maximum_possible_size() -> usize {
        0
        + 2 // spell_id: u16
        + 2 // spell_category: u16
        + 4 // cooldown_in_msecs: u32
        + 4 // category_cooldown_in_msecs: u32
    }
}

