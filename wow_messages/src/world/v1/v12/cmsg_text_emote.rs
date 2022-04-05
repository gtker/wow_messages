use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{Emote, EmoteError};
use crate::world::helper::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/cmsg_text_emote.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/cmsg_text_emote.wowm#L3):
/// ```text
/// cmsg CMSG_TEXT_EMOTE = 0x104 {
///     u32 text_emote;
///     Emote emote;
///     u64 guid;
/// }
/// ```
pub struct CMSG_TEXT_EMOTE {
    pub text_emote: u32,
    pub emote: Emote,
    pub guid: u64,
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

        // guid: u64
        let guid = crate::util::read_u64_le(r)?;

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

        // guid: u64
        w.write_all(&self.guid.to_le_bytes())?;

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
        + 8 // guid: u64
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

