use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct QuestObjective {
    pub creature_id: u32,
    pub kill_count: u32,
    pub required_item_id: u32,
    pub required_item_count: u32,
}

impl QuestObjective {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 16], std::io::Error> {
        let mut array_w = [0u8; 16];
        let mut w = array_w.as_mut_slice();
        // creature_id: u32
        w.write_all(&self.creature_id.to_le_bytes())?;

        // kill_count: u32
        w.write_all(&self.kill_count.to_le_bytes())?;

        // required_item_id: u32
        w.write_all(&self.required_item_id.to_le_bytes())?;

        // required_item_count: u32
        w.write_all(&self.required_item_count.to_le_bytes())?;

        Ok(array_w)
    }
}

impl QuestObjective {
    #[cfg(feature = "sync")]
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
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

    #[cfg(feature = "tokio")]
    pub(crate) async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
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

    #[cfg(feature = "async-std")]
    pub(crate) async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
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

}

