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
#[derive(Copy)]
pub struct ChannelMember {
    pub guid: Guid,
    pub member_flags: u8,
}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl ReadableAndWritable for ChannelMember {
    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // member_flags: u8
        let member_flags = crate::util::read_u8_le(r)?;

        Ok(Self {
            guid,
            member_flags,
        })
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // member_flags: u8
        w.write_all(&self.member_flags.to_le_bytes())?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::tokio_read(r).await?;

        // member_flags: u8
        let member_flags = crate::util::tokio_read_u8_le(r).await?;

        Ok(Self {
            guid,
            member_flags,
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
            // guid: Guid
            self.guid.tokio_write(w).await?;

            // member_flags: u8
            w.write_all(&self.member_flags.to_le_bytes()).await?;

            Ok(())
        })
    }
    #[cfg(feature = "async_std")]
    async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::astd_read(r).await?;

        // member_flags: u8
        let member_flags = crate::util::astd_read_u8_le(r).await?;

        Ok(Self {
            guid,
            member_flags,
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
            // guid: Guid
            self.guid.astd_write(w).await?;

            // member_flags: u8
            w.write_all(&self.member_flags.to_le_bytes()).await?;

            Ok(())
        })
    }
}

impl ConstantSized for ChannelMember {}

impl MaximumPossibleSized for ChannelMember {
    fn maximum_possible_size() -> usize {
        0
        + 8 // guid: Guid
        + 1 // member_flags: u8
    }
}

