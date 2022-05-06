use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct InitialSpell {
    pub spell_id: u16,
    pub unknown1: u16,
}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl ReadableAndWritable for InitialSpell {
    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // spell_id: u16
        let spell_id = crate::util::read_u16_le(r)?;

        // unknown1: u16
        let unknown1 = crate::util::read_u16_le(r)?;

        Ok(Self {
            spell_id,
            unknown1,
        })
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // spell_id: u16
        w.write_all(&self.spell_id.to_le_bytes())?;

        // unknown1: u16
        w.write_all(&self.unknown1.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // spell_id: u16
        let spell_id = crate::util::tokio_read_u16_le(r).await?;

        // unknown1: u16
        let unknown1 = crate::util::tokio_read_u16_le(r).await?;

        Ok(Self {
            spell_id,
            unknown1,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // spell_id: u16
        w.write_all(&self.spell_id.to_le_bytes()).await?;

        // unknown1: u16
        w.write_all(&self.unknown1.to_le_bytes()).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // spell_id: u16
        let spell_id = crate::util::astd_read_u16_le(r).await?;

        // unknown1: u16
        let unknown1 = crate::util::astd_read_u16_le(r).await?;

        Ok(Self {
            spell_id,
            unknown1,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // spell_id: u16
        w.write_all(&self.spell_id.to_le_bytes()).await?;

        // unknown1: u16
        w.write_all(&self.unknown1.to_le_bytes()).await?;

        Ok(())
    }

}

impl ConstantSized for InitialSpell {}

impl MaximumPossibleSized for InitialSpell {
    fn maximum_possible_size() -> usize {
        0
        + 2 // spell_id: u16
        + 2 // unknown1: u16
    }
}

