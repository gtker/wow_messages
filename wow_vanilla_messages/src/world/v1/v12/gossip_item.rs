use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct GossipItem {
    pub id: u32,
    pub item_icon: u8,
    pub coded: u8,
}

impl GossipItem {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 6], std::io::Error> {
        let mut array_w = [0u8; 6];
        let mut w = array_w.as_mut_slice();
        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // item_icon: u8
        w.write_all(&self.item_icon.to_le_bytes())?;

        // coded: u8
        w.write_all(&self.coded.to_le_bytes())?;

        Ok(array_w)
    }
}

impl GossipItem {
    #[cfg(feature = "sync")]
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // id: u32
        let id = crate::util::read_u32_le(r)?;

        // item_icon: u8
        let item_icon = crate::util::read_u8_le(r)?;

        // coded: u8
        let coded = crate::util::read_u8_le(r)?;

        Ok(Self {
            id,
            item_icon,
            coded,
        })
    }

    #[cfg(feature = "tokio")]
    pub(crate) async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // id: u32
        let id = crate::util::tokio_read_u32_le(r).await?;

        // item_icon: u8
        let item_icon = crate::util::tokio_read_u8_le(r).await?;

        // coded: u8
        let coded = crate::util::tokio_read_u8_le(r).await?;

        Ok(Self {
            id,
            item_icon,
            coded,
        })
    }

    #[cfg(feature = "async-std")]
    pub(crate) async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // id: u32
        let id = crate::util::astd_read_u32_le(r).await?;

        // item_icon: u8
        let item_icon = crate::util::astd_read_u8_le(r).await?;

        // coded: u8
        let coded = crate::util::astd_read_u8_le(r).await?;

        Ok(Self {
            id,
            item_icon,
            coded,
        })
    }

}

