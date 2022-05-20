use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct ForcedReaction {
    pub faction_id: u32,
    pub reputation_rank: u32,
}

impl ForcedReaction {
    #[cfg(feature = "sync")]
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // faction_id: u32
        let faction_id = crate::util::read_u32_le(r)?;

        // reputation_rank: u32
        let reputation_rank = crate::util::read_u32_le(r)?;

        Ok(Self {
            faction_id,
            reputation_rank,
        })
    }

    #[cfg(feature = "sync")]
    pub(crate) fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // faction_id: u32
        w.write_all(&self.faction_id.to_le_bytes())?;

        // reputation_rank: u32
        w.write_all(&self.reputation_rank.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "tokio")]
    pub(crate) async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // faction_id: u32
        let faction_id = crate::util::tokio_read_u32_le(r).await?;

        // reputation_rank: u32
        let reputation_rank = crate::util::tokio_read_u32_le(r).await?;

        Ok(Self {
            faction_id,
            reputation_rank,
        })
    }

    #[cfg(feature = "tokio")]
    pub(crate) async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // faction_id: u32
        w.write_all(&self.faction_id.to_le_bytes()).await?;

        // reputation_rank: u32
        w.write_all(&self.reputation_rank.to_le_bytes()).await?;

        Ok(())
    }

    #[cfg(feature = "async-std")]
    pub(crate) async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // faction_id: u32
        let faction_id = crate::util::astd_read_u32_le(r).await?;

        // reputation_rank: u32
        let reputation_rank = crate::util::astd_read_u32_le(r).await?;

        Ok(Self {
            faction_id,
            reputation_rank,
        })
    }

    #[cfg(feature = "async-std")]
    pub(crate) async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // faction_id: u32
        w.write_all(&self.faction_id.to_le_bytes()).await?;

        // reputation_rank: u32
        w.write_all(&self.reputation_rank.to_le_bytes()).await?;

        Ok(())
    }

}

impl ForcedReaction {
    pub(crate) fn size() -> usize {
        0
        + 4 // faction_id: u32
        + 4 // reputation_rank: u32
    }
}

