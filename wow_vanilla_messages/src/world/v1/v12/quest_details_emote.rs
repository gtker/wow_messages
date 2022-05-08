use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct QuestDetailsEmote {
    pub emote: u32,
    pub emote_delay_in_msecs: u32,
}

impl ReadableAndWritable for QuestDetailsEmote {
    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // emote: u32
        let emote = crate::util::read_u32_le(r)?;

        // emote_delay_in_msecs: u32
        let emote_delay_in_msecs = crate::util::read_u32_le(r)?;

        Ok(Self {
            emote,
            emote_delay_in_msecs,
        })
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // emote: u32
        w.write_all(&self.emote.to_le_bytes())?;

        // emote_delay_in_msecs: u32
        w.write_all(&self.emote_delay_in_msecs.to_le_bytes())?;

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
            // emote: u32
            let emote = crate::util::tokio_read_u32_le(r).await?;

            // emote_delay_in_msecs: u32
            let emote_delay_in_msecs = crate::util::tokio_read_u32_le(r).await?;

            Ok(Self {
                emote,
                emote_delay_in_msecs,
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
            // emote: u32
            w.write_all(&self.emote.to_le_bytes()).await?;

            // emote_delay_in_msecs: u32
            w.write_all(&self.emote_delay_in_msecs.to_le_bytes()).await?;

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
            // emote: u32
            let emote = crate::util::astd_read_u32_le(r).await?;

            // emote_delay_in_msecs: u32
            let emote_delay_in_msecs = crate::util::astd_read_u32_le(r).await?;

            Ok(Self {
                emote,
                emote_delay_in_msecs,
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
            // emote: u32
            w.write_all(&self.emote.to_le_bytes()).await?;

            // emote_delay_in_msecs: u32
            w.write_all(&self.emote_delay_in_msecs.to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl ConstantSized for QuestDetailsEmote {}

impl MaximumPossibleSized for QuestDetailsEmote {
    fn maximum_possible_size() -> usize {
        0
        + 4 // emote: u32
        + 4 // emote_delay_in_msecs: u32
    }
}

