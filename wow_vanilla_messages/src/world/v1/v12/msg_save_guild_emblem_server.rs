use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{GuildEmblemResult, GuildEmblemResultError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct MSG_SAVE_GUILD_EMBLEM_Server {
    pub result: GuildEmblemResult,
}

impl ServerMessageWrite for MSG_SAVE_GUILD_EMBLEM_Server {}

impl MessageBody for MSG_SAVE_GUILD_EMBLEM_Server {
    const OPCODE: u16 = 0x01f1;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = MSG_SAVE_GUILD_EMBLEM_ServerError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // result: GuildEmblemResult
        let result = GuildEmblemResult::read(r)?;

        Ok(Self {
            result,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // result: GuildEmblemResult
        self.result.write(w)?;

        Ok(())
    }

    fn tokio_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // result: GuildEmblemResult
            let result = GuildEmblemResult::tokio_read(r).await?;

            Ok(Self {
                result,
            })
        })
    }

    fn tokio_write_body<'life0, 'life1, 'async_trait, W>(
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
            // result: GuildEmblemResult
            self.result.tokio_write(w).await?;

            Ok(())
        })
    }

    fn astd_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // result: GuildEmblemResult
            let result = GuildEmblemResult::astd_read(r).await?;

            Ok(Self {
                result,
            })
        })
    }

    fn astd_write_body<'life0, 'life1, 'async_trait, W>(
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
            // result: GuildEmblemResult
            self.result.astd_write(w).await?;

            Ok(())
        })
    }

}

impl ConstantSized for MSG_SAVE_GUILD_EMBLEM_Server {}

impl MaximumPossibleSized for MSG_SAVE_GUILD_EMBLEM_Server {
    fn maximum_possible_size() -> usize {
        0
        + 4 // result: GuildEmblemResult
    }
}

#[derive(Debug)]
pub enum MSG_SAVE_GUILD_EMBLEM_ServerError {
    Io(std::io::Error),
    GuildEmblemResult(GuildEmblemResultError),
}

impl std::error::Error for MSG_SAVE_GUILD_EMBLEM_ServerError {}
impl std::fmt::Display for MSG_SAVE_GUILD_EMBLEM_ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::GuildEmblemResult(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for MSG_SAVE_GUILD_EMBLEM_ServerError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<GuildEmblemResultError> for MSG_SAVE_GUILD_EMBLEM_ServerError {
    fn from(e: GuildEmblemResultError) -> Self {
        Self::GuildEmblemResult(e)
    }
}

