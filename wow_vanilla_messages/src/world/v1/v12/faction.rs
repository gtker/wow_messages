use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct Faction {
    pub reputation_list_id: u32,
    pub standing: u32,
}

impl Faction {
    #[cfg(feature = "sync")]
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // reputation_list_id: u32
        let reputation_list_id = crate::util::read_u32_le(r)?;

        // standing: u32
        let standing = crate::util::read_u32_le(r)?;

        Ok(Self {
            reputation_list_id,
            standing,
        })
    }

    #[cfg(feature = "sync")]
    pub(crate) fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // reputation_list_id: u32
        w.write_all(&self.reputation_list_id.to_le_bytes())?;

        // standing: u32
        w.write_all(&self.standing.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "tokio")]
    pub(crate) async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // reputation_list_id: u32
        let reputation_list_id = crate::util::tokio_read_u32_le(r).await?;

        // standing: u32
        let standing = crate::util::tokio_read_u32_le(r).await?;

        Ok(Self {
            reputation_list_id,
            standing,
        })
    }

    #[cfg(feature = "tokio")]
    pub(crate) async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // reputation_list_id: u32
        w.write_all(&self.reputation_list_id.to_le_bytes()).await?;

        // standing: u32
        w.write_all(&self.standing.to_le_bytes()).await?;

        Ok(())
    }

    #[cfg(feature = "async-std")]
    pub(crate) async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // reputation_list_id: u32
        let reputation_list_id = crate::util::astd_read_u32_le(r).await?;

        // standing: u32
        let standing = crate::util::astd_read_u32_le(r).await?;

        Ok(Self {
            reputation_list_id,
            standing,
        })
    }

    #[cfg(feature = "async-std")]
    pub(crate) async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // reputation_list_id: u32
        w.write_all(&self.reputation_list_id.to_le_bytes()).await?;

        // standing: u32
        w.write_all(&self.standing.to_le_bytes()).await?;

        Ok(())
    }

}

impl Faction {
    pub(crate) fn size() -> usize {
        0
        + 4 // reputation_list_id: u32
        + 4 // standing: u32
    }
}

