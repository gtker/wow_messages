use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct WorldState {
    pub state: u32,
    pub value: u32,
}

impl WorldState {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // state: u32
        let state = crate::util::read_u32_le(r)?;

        // value: u32
        let value = crate::util::read_u32_le(r)?;

        Ok(Self {
            state,
            value,
        })
    }

    pub(crate) fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // state: u32
        w.write_all(&self.state.to_le_bytes())?;

        // value: u32
        w.write_all(&self.value.to_le_bytes())?;

        Ok(())
    }

    pub(crate) async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // state: u32
        let state = crate::util::tokio_read_u32_le(r).await?;

        // value: u32
        let value = crate::util::tokio_read_u32_le(r).await?;

        Ok(Self {
            state,
            value,
        })
    }

    pub(crate) async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // state: u32
        w.write_all(&self.state.to_le_bytes()).await?;

        // value: u32
        w.write_all(&self.value.to_le_bytes()).await?;

        Ok(())
    }

    pub(crate) async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // state: u32
        let state = crate::util::astd_read_u32_le(r).await?;

        // value: u32
        let value = crate::util::astd_read_u32_le(r).await?;

        Ok(Self {
            state,
            value,
        })
    }

    pub(crate) async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // state: u32
        w.write_all(&self.state.to_le_bytes()).await?;

        // value: u32
        w.write_all(&self.value.to_le_bytes()).await?;

        Ok(())
    }

}

impl ConstantSized for WorldState {}

impl MaximumPossibleSized for WorldState {
    fn maximum_possible_size() -> usize {
        0
        + 4 // state: u32
        + 4 // value: u32
    }
}

