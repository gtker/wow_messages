use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use crate::AsyncReadWrite;
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct ForcedReaction {
    pub faction_id: u32,
    pub reputation_rank: u32,
}

impl ReadableAndWritable for ForcedReaction {
    type Error = std::io::Error;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // faction_id: u32
        let faction_id = crate::util::read_u32_le(r)?;

        // reputation_rank: u32
        let reputation_rank = crate::util::read_u32_le(r)?;

        Ok(Self {
            faction_id,
            reputation_rank,
        })
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // faction_id: u32
        w.write_all(&self.faction_id.to_le_bytes())?;

        // reputation_rank: u32
        w.write_all(&self.reputation_rank.to_le_bytes())?;

        Ok(())
    }

}

#[cfg(any(feature = "async_tokio", feature = "async_std"))]
#[async_trait]
impl AsyncReadWrite for ForcedReaction {
    type Error = std::io::Error;

    #[cfg(feature = "async_tokio")]
    async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // faction_id: u32
        let faction_id = crate::util::tokio_read_u32_le(r).await?;

        // reputation_rank: u32
        let reputation_rank = crate::util::tokio_read_u32_le(r).await?;

        Ok(Self {
            faction_id,
            reputation_rank,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // faction_id: u32
        w.write_all(&self.faction_id.to_le_bytes()).await?;

        // reputation_rank: u32
        w.write_all(&self.reputation_rank.to_le_bytes()).await?;

        Ok(())
    }

}

impl ConstantSized for ForcedReaction {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for ForcedReaction {
    fn maximum_possible_size() -> usize {
        4 // faction_id: u32
        + 4 // reputation_rank: u32
    }
}

