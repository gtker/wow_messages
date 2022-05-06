use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{FactionFlag};
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct FactionInitializer {
    pub flag: FactionFlag,
    pub standing: u32,
}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl ReadableAndWritable for FactionInitializer {
    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // flag: FactionFlag
        let flag = FactionFlag::read(r)?;

        // standing: u32
        let standing = crate::util::read_u32_le(r)?;

        Ok(Self {
            flag,
            standing,
        })
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // flag: FactionFlag
        self.flag.write(w)?;

        // standing: u32
        w.write_all(&self.standing.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // flag: FactionFlag
        let flag = FactionFlag::tokio_read(r).await?;

        // standing: u32
        let standing = crate::util::tokio_read_u32_le(r).await?;

        Ok(Self {
            flag,
            standing,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // flag: FactionFlag
        self.flag.tokio_write(w).await?;

        // standing: u32
        w.write_all(&self.standing.to_le_bytes()).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // flag: FactionFlag
        let flag = FactionFlag::astd_read(r).await?;

        // standing: u32
        let standing = crate::util::astd_read_u32_le(r).await?;

        Ok(Self {
            flag,
            standing,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // flag: FactionFlag
        self.flag.astd_write(w).await?;

        // standing: u32
        w.write_all(&self.standing.to_le_bytes()).await?;

        Ok(())
    }

}

impl ConstantSized for FactionInitializer {}

impl MaximumPossibleSized for FactionInitializer {
    fn maximum_possible_size() -> usize {
        0
        + 1 // flag: FactionFlag
        + 4 // standing: u32
    }
}

