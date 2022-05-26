use std::convert::{TryFrom, TryInto};
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct NpcTextUpdateEmote {
    pub delay: u32,
    pub emote: u32,
}

impl NpcTextUpdateEmote {
    pub(crate) fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // delay: u32
        w.write_all(&self.delay.to_le_bytes())?;

        // emote: u32
        w.write_all(&self.emote.to_le_bytes())?;

        Ok(())
    }
}

impl NpcTextUpdateEmote {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // delay: u32
        let delay = crate::util::read_u32_le(r)?;

        // emote: u32
        let emote = crate::util::read_u32_le(r)?;

        Ok(Self {
            delay,
            emote,
        })
    }

    #[cfg(feature = "tokio")]
    pub(crate) async fn tokio_read<R: AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // delay: u32
        let delay = crate::util::tokio_read_u32_le(r).await?;

        // emote: u32
        let emote = crate::util::tokio_read_u32_le(r).await?;

        Ok(Self {
            delay,
            emote,
        })
    }

    #[cfg(feature = "async-std")]
    pub(crate) async fn astd_read<R: ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // delay: u32
        let delay = crate::util::astd_read_u32_le(r).await?;

        // emote: u32
        let emote = crate::util::astd_read_u32_le(r).await?;

        Ok(Self {
            delay,
            emote,
        })
    }

}

