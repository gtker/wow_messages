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
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(8000);
        // delay: u32
        w.write_all(&self.delay.to_le_bytes())?;

        // emote: u32
        w.write_all(&self.emote.to_le_bytes())?;

        Ok(w)
    }
}

impl NpcTextUpdateEmote {
    #[cfg(feature = "sync")]
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

impl NpcTextUpdateEmote {
    pub(crate) fn size() -> usize {
        0
        + 4 // delay: u32
        + 4 // emote: u32
    }
}

