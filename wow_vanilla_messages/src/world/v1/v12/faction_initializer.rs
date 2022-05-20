use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{FactionFlag};
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct FactionInitializer {
    pub flag: FactionFlag,
    pub standing: u32,
}

impl FactionInitializer {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(8000);
        // flag: FactionFlag
        w.write_all(&(self.flag.as_int() as u8).to_le_bytes())?;

        // standing: u32
        w.write_all(&self.standing.to_le_bytes())?;

        Ok(w)
    }

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

    #[cfg(feature = "tokio")]
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

    #[cfg(feature = "async-std")]
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

}

impl FactionInitializer {
    pub(crate) fn size() -> usize {
        0
        + 1 // flag: FactionFlag
        + 4 // standing: u32
    }
}

