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
pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
    pub build: u16,
}

impl ReadableAndWritable for Version {
    type Error = std::io::Error;

    #[cfg(feature = "sync")]
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

    #[cfg(feature = "sync")]
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

    fn tokio_read<'life0, 'async_trait, R>(
        r: &'life0 mut R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
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
        })
    }

    fn tokio_write<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // major: u8
            w.write_all(&self.major.to_le_bytes()).await?;

            // minor: u8
            w.write_all(&self.minor.to_le_bytes()).await?;

            // patch: u8
            w.write_all(&self.patch.to_le_bytes()).await?;

            // build: u16
            w.write_all(&self.build.to_le_bytes()).await?;

            Ok(())
        })
    }

    fn astd_read<'life0, 'async_trait, R>(
        r: &'life0 mut R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
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
        })
    }

    fn astd_write<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // major: u8
            w.write_all(&self.major.to_le_bytes()).await?;

            // minor: u8
            w.write_all(&self.minor.to_le_bytes()).await?;

            // patch: u8
            w.write_all(&self.patch.to_le_bytes()).await?;

            // build: u16
            w.write_all(&self.build.to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl ConstantSized for Version {}

impl MaximumPossibleSized for Version {
    fn maximum_possible_size() -> usize {
        0
        + 1 // major: u8
        + 1 // minor: u8
        + 1 // patch: u8
        + 2 // build: u16
    }
}

