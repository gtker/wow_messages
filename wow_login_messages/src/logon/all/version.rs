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
pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
    pub build: u16,
}

impl ReadableAndWritable for Version {
    type Error = std::io::Error;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
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

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
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

}

#[cfg(any(feature = "async_tokio", feature = "async_std"))]
#[async_trait]
impl AsyncReadWrite for Version {
    type Error = std::io::Error;
    #[cfg(feature = "async_tokio")]
    async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> Result<Self, Self::Error> {
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
    #[cfg(feature = "async_tokio")]
    async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> Result<(), std::io::Error> {
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
impl ConstantSized for Version {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for Version {
    fn maximum_possible_size() -> usize {
        1 // major: u8
        + 1 // minor: u8
        + 1 // patch: u8
        + 2 // build: u16
    }
}

