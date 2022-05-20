use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{FactionFlag};
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

impl FactionInitializer {
    #[cfg(feature = "sync")]
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // flag: FactionFlag
        let flag = FactionFlag::new(crate::util::read_u8_le(r)?);

        // standing: u32
        let standing = crate::util::read_u32_le(r)?;

        Ok(Self {
            flag,
            standing,
        })
    }

    #[cfg(feature = "sync")]
    pub(crate) fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // flag: FactionFlag
        w.write_all(&(self.flag.as_int() as u8).to_le_bytes())?;

        // standing: u32
        w.write_all(&self.standing.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub(crate) async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // flag: FactionFlag
        let flag = FactionFlag::new(crate::util::tokio_read_u8_le(r).await?);

        // standing: u32
        let standing = crate::util::tokio_read_u32_le(r).await?;

        Ok(Self {
            flag,
            standing,
        })
    }

    #[cfg(feature = "async_tokio")]
    pub(crate) async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // flag: FactionFlag
        w.write_all(&(self.flag.as_int() as u8).to_le_bytes()).await?;

        // standing: u32
        w.write_all(&self.standing.to_le_bytes()).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub(crate) async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // flag: FactionFlag
        let flag = FactionFlag::new(crate::util::astd_read_u8_le(r).await?);

        // standing: u32
        let standing = crate::util::astd_read_u32_le(r).await?;

        Ok(Self {
            flag,
            standing,
        })
    }

    #[cfg(feature = "async_std")]
    pub(crate) async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // flag: FactionFlag
        w.write_all(&(self.flag.as_int() as u8).to_le_bytes()).await?;

        // standing: u32
        w.write_all(&self.standing.to_le_bytes()).await?;

        Ok(())
    }

}

impl FactionInitializer {
    pub(crate) fn size() -> usize {
        0
        + 1 // flag: FactionFlag
        + 4 // standing: u32
    }
}

