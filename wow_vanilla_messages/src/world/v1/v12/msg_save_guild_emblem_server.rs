use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{GuildEmblemResult, GuildEmblemResultError};
use crate::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use crate::AsyncReadWrite;
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct MSG_SAVE_GUILD_EMBLEM_Server {
    pub result: GuildEmblemResult,
}

impl WorldServerMessageWrite for MSG_SAVE_GUILD_EMBLEM_Server {
    const OPCODE: u16 = 0x1f1;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (Self::size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (Self::size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for MSG_SAVE_GUILD_EMBLEM_Server {
    type Error = MSG_SAVE_GUILD_EMBLEM_ServerError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // result: GuildEmblemResult
        let result = GuildEmblemResult::read(r)?;

        Ok(Self {
            result,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // result: GuildEmblemResult
        self.result.write(w)?;

        Ok(())
    }
}

impl ConstantSized for MSG_SAVE_GUILD_EMBLEM_Server {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for MSG_SAVE_GUILD_EMBLEM_Server {
    fn maximum_possible_size() -> usize {
        GuildEmblemResult::size() // result: GuildEmblemResult
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

