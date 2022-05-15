use std::convert::{TryFrom, TryInto};
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

impl Vector3d {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
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

    pub(crate) fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // x: f32
        w.write_all(&self.x.to_le_bytes())?;

        // y: f32
        w.write_all(&self.y.to_le_bytes())?;

        // z: f32
        w.write_all(&self.z.to_le_bytes())?;

        Ok(())
    }

    pub(crate) async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
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
    }

    pub(crate) async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // x: f32
        w.write_all(&self.x.to_le_bytes()).await?;

        // y: f32
        w.write_all(&self.y.to_le_bytes()).await?;

        // z: f32
        w.write_all(&self.z.to_le_bytes()).await?;

        Ok(())
    }

    pub(crate) async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
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
    }

    pub(crate) async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // x: f32
        w.write_all(&self.x.to_le_bytes()).await?;

        // y: f32
        w.write_all(&self.y.to_le_bytes()).await?;

        // z: f32
        w.write_all(&self.z.to_le_bytes()).await?;

        Ok(())
    }

}

impl Vector3d {
    pub(crate) fn size() -> usize {
        0
        + 4 // x: f32
        + 4 // y: f32
        + 4 // z: f32
    }
}

