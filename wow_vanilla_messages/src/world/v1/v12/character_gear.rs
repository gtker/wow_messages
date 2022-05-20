use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{InventoryType, InventoryTypeError};
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CharacterGear {
    pub equipment_display_id: u32,
    pub inventory_type: InventoryType,
}

impl CharacterGear {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(8000);
        // equipment_display_id: u32
        w.write_all(&self.equipment_display_id.to_le_bytes())?;

        // inventory_type: InventoryType
        w.write_all(&(self.inventory_type.as_int() as u8).to_le_bytes())?;

        Ok(w)
    }

    #[cfg(feature = "sync")]
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, CharacterGearError> {
        // equipment_display_id: u32
        let equipment_display_id = crate::util::read_u32_le(r)?;

        // inventory_type: InventoryType
        let inventory_type: InventoryType = crate::util::read_u8_le(r)?.try_into()?;

        Ok(Self {
            equipment_display_id,
            inventory_type,
        })
    }

    #[cfg(feature = "tokio")]
    pub(crate) async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, CharacterGearError> {
        // equipment_display_id: u32
        let equipment_display_id = crate::util::tokio_read_u32_le(r).await?;

        // inventory_type: InventoryType
        let inventory_type: InventoryType = crate::util::tokio_read_u8_le(r).await?.try_into()?;

        Ok(Self {
            equipment_display_id,
            inventory_type,
        })
    }

    #[cfg(feature = "async-std")]
    pub(crate) async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, CharacterGearError> {
        // equipment_display_id: u32
        let equipment_display_id = crate::util::astd_read_u32_le(r).await?;

        // inventory_type: InventoryType
        let inventory_type: InventoryType = crate::util::astd_read_u8_le(r).await?.try_into()?;

        Ok(Self {
            equipment_display_id,
            inventory_type,
        })
    }

}

impl CharacterGear {
    pub(crate) fn size() -> usize {
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

