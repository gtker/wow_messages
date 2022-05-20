use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct Vector3d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3d {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(8000);
        // x: f32
        w.write_all(&self.x.to_le_bytes())?;

        // y: f32
        w.write_all(&self.y.to_le_bytes())?;

        // z: f32
        w.write_all(&self.z.to_le_bytes())?;

        Ok(w)
    }
}

impl Vector3d {
    #[cfg(feature = "sync")]
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

    #[cfg(feature = "tokio")]
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

    #[cfg(feature = "async-std")]
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

}

impl Vector3d {
    pub(crate) fn size() -> usize {
        0
        + 4 // x: f32
        + 4 // y: f32
        + 4 // z: f32
    }
}

