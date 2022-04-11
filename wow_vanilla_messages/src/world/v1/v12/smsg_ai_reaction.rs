use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{AiReaction, AiReactionError};
use crate::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_AI_REACTION {
    pub guid: Guid,
    pub reaction: AiReaction,
}

impl WorldServerMessageWrite for SMSG_AI_REACTION {
    const OPCODE: u16 = 0x13c;

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
impl WorldMessageBody for SMSG_AI_REACTION {
    type Error = SMSG_AI_REACTIONError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // reaction: AiReaction
        let reaction = AiReaction::read(r)?;

        Ok(Self {
            guid,
            reaction,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // reaction: AiReaction
        self.reaction.write(w)?;

        Ok(())
    }
}

impl ConstantSized for SMSG_AI_REACTION {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_AI_REACTION {
    fn maximum_possible_size() -> usize {
        8 // guid: Guid
        + AiReaction::size() // reaction: AiReaction
    }
}

#[derive(Debug)]
pub enum SMSG_AI_REACTIONError {
    Io(std::io::Error),
    AiReaction(AiReactionError),
}

impl std::error::Error for SMSG_AI_REACTIONError {}
impl std::fmt::Display for SMSG_AI_REACTIONError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::AiReaction(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_AI_REACTIONError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<AiReactionError> for SMSG_AI_REACTIONError {
    fn from(e: AiReactionError) -> Self {
        Self::AiReaction(e)
    }
}

