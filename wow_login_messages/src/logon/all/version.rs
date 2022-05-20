use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async-std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
    pub build: u16,
}

impl Version {
    #[cfg(feature = "sync")]
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // major: u8
        let major = crate::util::read_u8_le(r)?;

        // minor: u8
        let minor = crate::util::read_u8_le(r)?;

        // patch: u8
        let patch = crate::util::read_u8_le(r)?;

        // build: u16
        let build = crate::util::read_u16_le(r)?;

        Ok(Self {
            major,
            minor,
            patch,
            build,
        })
    }

    #[cfg(feature = "sync")]
    pub(crate) fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // major: u8
        w.write_all(&self.major.to_le_bytes())?;

        // minor: u8
        w.write_all(&self.minor.to_le_bytes())?;

        // patch: u8
        w.write_all(&self.patch.to_le_bytes())?;

        // build: u16
        w.write_all(&self.build.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "tokio")]
    pub(crate) async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // major: u8
        let major = crate::util::tokio_read_u8_le(r).await?;

        // minor: u8
        let minor = crate::util::tokio_read_u8_le(r).await?;

        // patch: u8
        let patch = crate::util::tokio_read_u8_le(r).await?;

        // build: u16
        let build = crate::util::tokio_read_u16_le(r).await?;

        Ok(Self {
            major,
            minor,
            patch,
            build,
        })
    }

    #[cfg(feature = "tokio")]
    pub(crate) async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // major: u8
        w.write_all(&self.major.to_le_bytes()).await?;

        // minor: u8
        w.write_all(&self.minor.to_le_bytes()).await?;

        // patch: u8
        w.write_all(&self.patch.to_le_bytes()).await?;

        // build: u16
        w.write_all(&self.build.to_le_bytes()).await?;

        Ok(())
    }

    #[cfg(feature = "async-std")]
    pub(crate) async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // major: u8
        let major = crate::util::astd_read_u8_le(r).await?;

        // minor: u8
        let minor = crate::util::astd_read_u8_le(r).await?;

        // patch: u8
        let patch = crate::util::astd_read_u8_le(r).await?;

        // build: u16
        let build = crate::util::astd_read_u16_le(r).await?;

        Ok(Self {
            major,
            minor,
            patch,
            build,
        })
    }

    #[cfg(feature = "async-std")]
    pub(crate) async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // major: u8
        w.write_all(&self.major.to_le_bytes()).await?;

        // minor: u8
        w.write_all(&self.minor.to_le_bytes()).await?;

        // patch: u8
        w.write_all(&self.patch.to_le_bytes()).await?;

        // build: u16
        w.write_all(&self.build.to_le_bytes()).await?;

        Ok(())
    }

}

impl Version {
    pub(crate) fn size() -> usize {
        0
        + 1 // major: u8
        + 1 // minor: u8
        + 1 // patch: u8
        + 2 // build: u16
    }
}

