use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct PetitionShowlist {
    pub index: u32,
    pub guild_charter_cost: u32,
    pub unknown1: u32,
    pub unknown2: u32,
}

impl PetitionShowlist {
    pub const CHARTER_ENTRY_VALUE: u32 = 0x16e7;

    pub const CHARTER_DISPLAY_ID_VALUE: u32 = 0x3f21;

}

impl PetitionShowlist {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 24], std::io::Error> {
        let mut array_w = [0u8; 24];
        let mut w = array_w.as_mut_slice();
        // index: u32
        w.write_all(&self.index.to_le_bytes())?;

        // charter_entry: u32
        w.write_all(&Self::CHARTER_ENTRY_VALUE.to_le_bytes())?;

        // charter_display_id: u32
        w.write_all(&Self::CHARTER_DISPLAY_ID_VALUE.to_le_bytes())?;

        // guild_charter_cost: u32
        w.write_all(&self.guild_charter_cost.to_le_bytes())?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // unknown2: u32
        w.write_all(&self.unknown2.to_le_bytes())?;

        Ok(array_w)
    }
}

impl PetitionShowlist {
    #[cfg(feature = "sync")]
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // index: u32
        let index = crate::util::read_u32_le(r)?;

        // charter_entry: u32
        let _charter_entry = crate::util::read_u32_le(r)?;
        // charter_entry is expected to always be 5863 (5863)

        // charter_display_id: u32
        let _charter_display_id = crate::util::read_u32_le(r)?;
        // charter_display_id is expected to always be 16161 (16161)

        // guild_charter_cost: u32
        let guild_charter_cost = crate::util::read_u32_le(r)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(r)?;

        // unknown2: u32
        let unknown2 = crate::util::read_u32_le(r)?;

        Ok(Self {
            index,
            guild_charter_cost,
            unknown1,
            unknown2,
        })
    }

    #[cfg(feature = "tokio")]
    pub(crate) async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // index: u32
        let index = crate::util::tokio_read_u32_le(r).await?;

        // charter_entry: u32
        let _charter_entry = crate::util::tokio_read_u32_le(r).await?;
        // charter_entry is expected to always be 5863 (5863)

        // charter_display_id: u32
        let _charter_display_id = crate::util::tokio_read_u32_le(r).await?;
        // charter_display_id is expected to always be 16161 (16161)

        // guild_charter_cost: u32
        let guild_charter_cost = crate::util::tokio_read_u32_le(r).await?;

        // unknown1: u32
        let unknown1 = crate::util::tokio_read_u32_le(r).await?;

        // unknown2: u32
        let unknown2 = crate::util::tokio_read_u32_le(r).await?;

        Ok(Self {
            index,
            guild_charter_cost,
            unknown1,
            unknown2,
        })
    }

    #[cfg(feature = "async-std")]
    pub(crate) async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // index: u32
        let index = crate::util::astd_read_u32_le(r).await?;

        // charter_entry: u32
        let _charter_entry = crate::util::astd_read_u32_le(r).await?;
        // charter_entry is expected to always be 5863 (5863)

        // charter_display_id: u32
        let _charter_display_id = crate::util::astd_read_u32_le(r).await?;
        // charter_display_id is expected to always be 16161 (16161)

        // guild_charter_cost: u32
        let guild_charter_cost = crate::util::astd_read_u32_le(r).await?;

        // unknown1: u32
        let unknown1 = crate::util::astd_read_u32_le(r).await?;

        // unknown2: u32
        let unknown2 = crate::util::astd_read_u32_le(r).await?;

        Ok(Self {
            index,
            guild_charter_cost,
            unknown1,
            unknown2,
        })
    }

}

impl PetitionShowlist {
    pub(crate) fn size() -> usize {
        0
        + 4 // index: u32
        + 4 // charter_entry: u32
        + 4 // charter_display_id: u32
        + 4 // guild_charter_cost: u32
        + 4 // unknown1: u32
        + 4 // unknown2: u32
    }
}

