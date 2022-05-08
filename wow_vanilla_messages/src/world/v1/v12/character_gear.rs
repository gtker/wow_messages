use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{InventoryType, InventoryTypeError};
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CharacterGear {
    pub equipment_display_id: u32,
    pub inventory_type: InventoryType,
}

impl ReadableAndWritable for CharacterGear {
    type Error = CharacterGearError;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // equipment_display_id: u32
        let equipment_display_id = crate::util::read_u32_le(r)?;

        // inventory_type: InventoryType
        let inventory_type = InventoryType::read(r)?;

        Ok(Self {
            equipment_display_id,
            inventory_type,
        })
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // equipment_display_id: u32
        w.write_all(&self.equipment_display_id.to_le_bytes())?;

        // inventory_type: InventoryType
        self.inventory_type.write(w)?;

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
            // equipment_display_id: u32
            let equipment_display_id = crate::util::tokio_read_u32_le(r).await?;

            // inventory_type: InventoryType
            let inventory_type = InventoryType::tokio_read(r).await?;

            Ok(Self {
                equipment_display_id,
                inventory_type,
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
            // equipment_display_id: u32
            w.write_all(&self.equipment_display_id.to_le_bytes()).await?;

            // inventory_type: InventoryType
            self.inventory_type.tokio_write(w).await?;

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
            // equipment_display_id: u32
            let equipment_display_id = crate::util::astd_read_u32_le(r).await?;

            // inventory_type: InventoryType
            let inventory_type = InventoryType::astd_read(r).await?;

            Ok(Self {
                equipment_display_id,
                inventory_type,
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
            // equipment_display_id: u32
            w.write_all(&self.equipment_display_id.to_le_bytes()).await?;

            // inventory_type: InventoryType
            self.inventory_type.astd_write(w).await?;

            Ok(())
        })
    }

}

impl ConstantSized for CharacterGear {}

impl MaximumPossibleSized for CharacterGear {
    fn maximum_possible_size() -> usize {
        0
        + 4 // equipment_display_id: u32
        + 1 // inventory_type: InventoryType
    }
}

#[derive(Debug)]
pub enum CharacterGearError {
    Io(std::io::Error),
    InventoryType(InventoryTypeError),
}

impl std::error::Error for CharacterGearError {}
impl std::fmt::Display for CharacterGearError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::InventoryType(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CharacterGearError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<InventoryTypeError> for CharacterGearError {
    fn from(e: InventoryTypeError) -> Self {
        Self::InventoryType(e)
    }
}

