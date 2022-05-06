use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct TransportInfo {
    pub guid: Guid,
    pub position_x: f32,
    pub position_y: f32,
    pub position_z: f32,
    pub orientation: f32,
    pub timestamp: u32,
}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl ReadableAndWritable for TransportInfo {
    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // guid: PackedGuid
        let guid = Guid::read_packed(r)?;

        // position_x: f32
        let position_x = crate::util::read_f32_le(r)?;
        // position_y: f32
        let position_y = crate::util::read_f32_le(r)?;
        // position_z: f32
        let position_z = crate::util::read_f32_le(r)?;
        // orientation: f32
        let orientation = crate::util::read_f32_le(r)?;
        // timestamp: u32
        let timestamp = crate::util::read_u32_le(r)?;

        Ok(Self {
            guid,
            position_x,
            position_y,
            position_z,
            orientation,
            timestamp,
        })
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: PackedGuid
        self.guid.write_packed(w)?;

        // position_x: f32
        w.write_all(&self.position_x.to_le_bytes())?;

        // position_y: f32
        w.write_all(&self.position_y.to_le_bytes())?;

        // position_z: f32
        w.write_all(&self.position_z.to_le_bytes())?;

        // orientation: f32
        w.write_all(&self.orientation.to_le_bytes())?;

        // timestamp: u32
        w.write_all(&self.timestamp.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // guid: PackedGuid
        let guid = Guid::tokio_read_packed(r).await?;

        // position_x: f32
        let position_x = crate::util::tokio_read_f32_le(r).await?;
        // position_y: f32
        let position_y = crate::util::tokio_read_f32_le(r).await?;
        // position_z: f32
        let position_z = crate::util::tokio_read_f32_le(r).await?;
        // orientation: f32
        let orientation = crate::util::tokio_read_f32_le(r).await?;
        // timestamp: u32
        let timestamp = crate::util::tokio_read_u32_le(r).await?;

        Ok(Self {
            guid,
            position_x,
            position_y,
            position_z,
            orientation,
            timestamp,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: PackedGuid
        self.guid.tokio_write_packed(w).await?;

        // position_x: f32
        w.write_all(&self.position_x.to_le_bytes()).await?;

        // position_y: f32
        w.write_all(&self.position_y.to_le_bytes()).await?;

        // position_z: f32
        w.write_all(&self.position_z.to_le_bytes()).await?;

        // orientation: f32
        w.write_all(&self.orientation.to_le_bytes()).await?;

        // timestamp: u32
        w.write_all(&self.timestamp.to_le_bytes()).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // guid: PackedGuid
        let guid = Guid::astd_read_packed(r).await?;

        // position_x: f32
        let position_x = crate::util::astd_read_f32_le(r).await?;
        // position_y: f32
        let position_y = crate::util::astd_read_f32_le(r).await?;
        // position_z: f32
        let position_z = crate::util::astd_read_f32_le(r).await?;
        // orientation: f32
        let orientation = crate::util::astd_read_f32_le(r).await?;
        // timestamp: u32
        let timestamp = crate::util::astd_read_u32_le(r).await?;

        Ok(Self {
            guid,
            position_x,
            position_y,
            position_z,
            orientation,
            timestamp,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: PackedGuid
        self.guid.astd_write_packed(w).await?;

        // position_x: f32
        w.write_all(&self.position_x.to_le_bytes()).await?;

        // position_y: f32
        w.write_all(&self.position_y.to_le_bytes()).await?;

        // position_z: f32
        w.write_all(&self.position_z.to_le_bytes()).await?;

        // orientation: f32
        w.write_all(&self.orientation.to_le_bytes()).await?;

        // timestamp: u32
        w.write_all(&self.timestamp.to_le_bytes()).await?;

        Ok(())
    }

}

impl VariableSized for TransportInfo {
    fn size(&self) -> usize {
        0
        + self.guid.size() // guid: Guid
        + 4 // position_x: f32
        + 4 // position_y: f32
        + 4 // position_z: f32
        + 4 // orientation: f32
        + 4 // timestamp: u32
    }
}

impl MaximumPossibleSized for TransportInfo {
    fn maximum_possible_size() -> usize {
        0
        + 9 // guid: Guid
        + 4 // position_x: f32
        + 4 // position_y: f32
        + 4 // position_z: f32
        + 4 // orientation: f32
        + 4 // timestamp: u32
    }
}

