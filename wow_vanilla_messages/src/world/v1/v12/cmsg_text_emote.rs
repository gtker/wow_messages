use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{Emote, EmoteError};
use crate::{WorldClientMessageWrite, WorldMessageBody};
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
pub struct CMSG_TEXT_EMOTE {
    pub text_emote: u32,
    pub emote: Emote,
    pub guid: Guid,
}

impl WorldClientMessageWrite for CMSG_TEXT_EMOTE {
    const OPCODE: u32 = 0x104;

    fn write_unencrypted_client<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        crate::util::write_u16_be(w, (Self::size() + 4) as u16)?;
        crate::util::write_u32_le(w, <Self as WorldClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_client<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        e.write_encrypted_client_header(w, (Self::size() + 4) as u16, <Self as WorldClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for CMSG_TEXT_EMOTE {
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
}

impl ConstantSized for CMSG_TEXT_EMOTE {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

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

