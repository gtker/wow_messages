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
pub struct WorldState {
    pub state: u32,
    pub value: u32,
}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl ReadableAndWritable for WorldState {
    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // state: u32
        let state = crate::util::read_u32_le(r)?;

        // value: u32
        let value = crate::util::read_u32_le(r)?;

        Ok(Self {
            state,
            value,
        })
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // state: u32
        w.write_all(&self.state.to_le_bytes())?;

        // value: u32
        w.write_all(&self.value.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // state: u32
        let state = crate::util::tokio_read_u32_le(r).await?;

        // value: u32
        let value = crate::util::tokio_read_u32_le(r).await?;

        Ok(Self {
            state,
            value,
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
            // state: u32
            w.write_all(&self.state.to_le_bytes()).await?;

            // value: u32
            w.write_all(&self.value.to_le_bytes()).await?;

            Ok(())
        })
    }
    #[cfg(feature = "async_std")]
    async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // state: u32
        let state = crate::util::astd_read_u32_le(r).await?;

        // value: u32
        let value = crate::util::astd_read_u32_le(r).await?;

        Ok(Self {
            state,
            value,
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
            // state: u32
            w.write_all(&self.state.to_le_bytes()).await?;

            // value: u32
            w.write_all(&self.value.to_le_bytes()).await?;

            Ok(())
        })
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

