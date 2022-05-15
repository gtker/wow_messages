use std::convert::{TryFrom, TryInto};
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

impl QuestDetailsEmote {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // emote: u32
        let emote = crate::util::read_u32_le(r)?;

        // emote_delay_in_msecs: u32
        let emote_delay_in_msecs = crate::util::read_u32_le(r)?;

        Ok(Self {
            emote,
            emote_delay_in_msecs,
        })
    }

    pub(crate) fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // emote: u32
        w.write_all(&self.emote.to_le_bytes())?;

        // emote_delay_in_msecs: u32
        w.write_all(&self.emote_delay_in_msecs.to_le_bytes())?;

        Ok(())
    }

    pub(crate) async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // emote: u32
        let emote = crate::util::tokio_read_u32_le(r).await?;

        // emote_delay_in_msecs: u32
        let emote_delay_in_msecs = crate::util::tokio_read_u32_le(r).await?;

        Ok(Self {
            emote,
            emote_delay_in_msecs,
        })
    }

    pub(crate) async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // emote: u32
        w.write_all(&self.emote.to_le_bytes()).await?;

        // emote_delay_in_msecs: u32
        w.write_all(&self.emote_delay_in_msecs.to_le_bytes()).await?;

        Ok(())
    }

    pub(crate) async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // emote: u32
        let emote = crate::util::astd_read_u32_le(r).await?;

        // emote_delay_in_msecs: u32
        let emote_delay_in_msecs = crate::util::astd_read_u32_le(r).await?;

        Ok(Self {
            emote,
            emote_delay_in_msecs,
        })
    }

    pub(crate) async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // emote: u32
        w.write_all(&self.emote.to_le_bytes()).await?;

        // emote_delay_in_msecs: u32
        w.write_all(&self.emote_delay_in_msecs.to_le_bytes()).await?;

        Ok(())
    }

}

impl QuestDetailsEmote {
    pub(crate) fn size() -> usize {
        0
        + 4 // emote: u32
        + 4 // emote_delay_in_msecs: u32
    }
}

