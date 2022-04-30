use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{Emote, EmoteError};
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMSG_TEXT_EMOTE {
    pub text_emote: u32,
    pub emote: Emote,
    pub guid: Guid,
}

impl ClientMessageWrite for CMSG_TEXT_EMOTE {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for CMSG_TEXT_EMOTE {
    const OPCODE: u16 = 0x0104;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = CMSG_TEXT_EMOTEError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // text_emote: u32
        let text_emote = crate::util::read_u32_le(r)?;

        // emote: Emote
        let emote = Emote::read(r)?;

        // guid: Guid
        let guid = Guid::read(r)?;

        Ok(Self {
            text_emote,
            emote,
            guid,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // text_emote: u32
        w.write_all(&self.text_emote.to_le_bytes())?;

        // emote: Emote
        self.emote.write(w)?;

        // guid: Guid
        self.guid.write(w)?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // text_emote: u32
        let text_emote = crate::util::tokio_read_u32_le(r).await?;

        // emote: Emote
        let emote = Emote::tokio_read(r).await?;

        // guid: Guid
        let guid = Guid::tokio_read(r).await?;

        Ok(Self {
            text_emote,
            emote,
            guid,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // text_emote: u32
        w.write_all(&self.text_emote.to_le_bytes()).await?;

        // emote: Emote
        self.emote.tokio_write(w).await?;

        // guid: Guid
        self.guid.tokio_write(w).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // text_emote: u32
        let text_emote = crate::util::astd_read_u32_le(r).await?;

        // emote: Emote
        let emote = Emote::astd_read(r).await?;

        // guid: Guid
        let guid = Guid::astd_read(r).await?;

        Ok(Self {
            text_emote,
            emote,
            guid,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // text_emote: u32
        w.write_all(&self.text_emote.to_le_bytes()).await?;

        // emote: Emote
        self.emote.astd_write(w).await?;

        // guid: Guid
        self.guid.astd_write(w).await?;

        Ok(())
    }
}

impl ConstantSized for CMSG_TEXT_EMOTE {}

impl MaximumPossibleSized for CMSG_TEXT_EMOTE {
    fn maximum_possible_size() -> usize {
        4 // text_emote: u32
        + Emote::size() // emote: Emote
        + 8 // guid: Guid
    }
}

#[derive(Debug)]
pub enum CMSG_TEXT_EMOTEError {
    Io(std::io::Error),
    Emote(EmoteError),
}

impl std::error::Error for CMSG_TEXT_EMOTEError {}
impl std::fmt::Display for CMSG_TEXT_EMOTEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::Emote(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_TEXT_EMOTEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<EmoteError> for CMSG_TEXT_EMOTEError {
    fn from(e: EmoteError) -> Self {
        Self::Emote(e)
    }
}

