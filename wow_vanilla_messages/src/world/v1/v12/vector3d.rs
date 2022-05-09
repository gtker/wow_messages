use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct Vector3d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl ReadableAndWritable for Vector3d {
    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // x: f32
        let x = crate::util::read_f32_le(r)?;
        // y: f32
        let y = crate::util::read_f32_le(r)?;
        // z: f32
        let z = crate::util::read_f32_le(r)?;
        Ok(Self {
            x,
            y,
            z,
        })
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // x: f32
        w.write_all(&self.x.to_le_bytes())?;

        // y: f32
        w.write_all(&self.y.to_le_bytes())?;

        // z: f32
        w.write_all(&self.z.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
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
            // x: f32
            let x = crate::util::tokio_read_f32_le(r).await?;
            // y: f32
            let y = crate::util::tokio_read_f32_le(r).await?;
            // z: f32
            let z = crate::util::tokio_read_f32_le(r).await?;
            Ok(Self {
                x,
                y,
                z,
            })
        })
    }

    #[cfg(feature = "async_tokio")]
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
            // x: f32
            w.write_all(&self.x.to_le_bytes()).await?;

            // y: f32
            w.write_all(&self.y.to_le_bytes()).await?;

            // z: f32
            w.write_all(&self.z.to_le_bytes()).await?;

            Ok(())
        })
    }

    #[cfg(feature = "async_std")]
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
            // x: f32
            let x = crate::util::astd_read_f32_le(r).await?;
            // y: f32
            let y = crate::util::astd_read_f32_le(r).await?;
            // z: f32
            let z = crate::util::astd_read_f32_le(r).await?;
            Ok(Self {
                x,
                y,
                z,
            })
        })
    }

    #[cfg(feature = "async_std")]
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
            // x: f32
            w.write_all(&self.x.to_le_bytes()).await?;

            // y: f32
            w.write_all(&self.y.to_le_bytes()).await?;

            // z: f32
            w.write_all(&self.z.to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl ConstantSized for Vector3d {}

impl MaximumPossibleSized for Vector3d {
    fn maximum_possible_size() -> usize {
        0
        + 4 // x: f32
        + 4 // y: f32
        + 4 // z: f32
    }
}

