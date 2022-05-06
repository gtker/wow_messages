use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{Emote, EmoteError};
use crate::{ServerMessageWrite, MessageBody};
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
pub struct SMSG_EMOTE {
    pub emote: Emote,
    pub guid: Guid,
}

impl ServerMessageWrite for SMSG_EMOTE {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_EMOTE {
    const OPCODE: u16 = 0x0103;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = SMSG_EMOTEError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // emote: Emote
        let emote = Emote::read(r)?;

        // guid: Guid
        let guid = Guid::read(r)?;

        Ok(Self {
            emote,
            guid,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // emote: Emote
        self.emote.write(w)?;

        // guid: Guid
        self.guid.write(w)?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // emote: Emote
        let emote = Emote::tokio_read(r).await?;

        // guid: Guid
        let guid = Guid::tokio_read(r).await?;

        Ok(Self {
            emote,
            guid,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // emote: Emote
        self.emote.tokio_write(w).await?;

        // guid: Guid
        self.guid.tokio_write(w).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // emote: Emote
        let emote = Emote::astd_read(r).await?;

        // guid: Guid
        let guid = Guid::astd_read(r).await?;

        Ok(Self {
            emote,
            guid,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // emote: Emote
        self.emote.astd_write(w).await?;

        // guid: Guid
        self.guid.astd_write(w).await?;

        Ok(())
    }

}

impl ConstantSized for SMSG_EMOTE {}

impl MaximumPossibleSized for SMSG_EMOTE {
    fn maximum_possible_size() -> usize {
        0
        + 4 // emote: Emote
        + 8 // guid: Guid
    }
}

#[derive(Debug)]
pub enum SMSG_EMOTEError {
    Io(std::io::Error),
    Emote(EmoteError),
}

impl std::error::Error for SMSG_EMOTEError {}
impl std::fmt::Display for SMSG_EMOTEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::Emote(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_EMOTEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<EmoteError> for SMSG_EMOTEError {
    fn from(e: EmoteError) -> Self {
        Self::Emote(e)
    }
}

