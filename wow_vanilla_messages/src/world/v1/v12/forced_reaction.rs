use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct ForcedReaction {
    pub faction_id: u32,
    pub reputation_rank: u32,
}

impl ForcedReaction {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 8], std::io::Error> {
        let mut array_w = [0u8; 8];
        let mut w = array_w.as_mut_slice();
        // faction_id: u32
        w.write_all(&self.faction_id.to_le_bytes())?;

        // reputation_rank: u32
        w.write_all(&self.reputation_rank.to_le_bytes())?;

        Ok(array_w)
    }
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

}

impl ForcedReaction {
    pub(crate) fn size() -> usize {
        0
        + 4 // faction_id: u32
        + 4 // reputation_rank: u32
    }
}

