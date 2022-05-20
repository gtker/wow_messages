use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct ItemStat {
    pub item_stat_type: u32,
    pub item_stat_value: u32,
}

impl ItemStat {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 8], std::io::Error> {
        let mut array_w = [0u8; 8];
        let mut w = array_w.as_mut_slice();
        // item_stat_type: u32
        w.write_all(&self.item_stat_type.to_le_bytes())?;

        // item_stat_value: u32
        w.write_all(&self.item_stat_value.to_le_bytes())?;

        Ok(array_w)
    }
}

impl ItemStat {
    #[cfg(feature = "sync")]
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // item_stat_type: u32
        let item_stat_type = crate::util::read_u32_le(r)?;

        // item_stat_value: u32
        let item_stat_value = crate::util::read_u32_le(r)?;

        Ok(Self {
            item_stat_type,
            item_stat_value,
        })
    }

    #[cfg(feature = "tokio")]
    pub(crate) async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // item_stat_type: u32
        let item_stat_type = crate::util::tokio_read_u32_le(r).await?;

        // item_stat_value: u32
        let item_stat_value = crate::util::tokio_read_u32_le(r).await?;

        Ok(Self {
            item_stat_type,
            item_stat_value,
        })
    }

    #[cfg(feature = "async-std")]
    pub(crate) async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // item_stat_type: u32
        let item_stat_type = crate::util::astd_read_u32_le(r).await?;

        // item_stat_value: u32
        let item_stat_value = crate::util::astd_read_u32_le(r).await?;

        Ok(Self {
            item_stat_type,
            item_stat_value,
        })
    }

}

impl ItemStat {
    pub(crate) fn size() -> usize {
        0
        + 4 // item_stat_type: u32
        + 4 // item_stat_value: u32
    }
}

