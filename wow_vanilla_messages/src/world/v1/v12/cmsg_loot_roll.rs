use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{RollVote, RollVoteError};
use crate::{WorldClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use crate::AsyncReadWrite;
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMSG_LOOT_ROLL {
    pub item_guid: Guid,
    pub item_slot: u32,
    pub vote: RollVote,
}

impl WorldClientMessageWrite for CMSG_LOOT_ROLL {
    const OPCODE: u32 = 0x2a0;

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
impl MessageBody for CMSG_LOOT_ROLL {
    type Error = CMSG_LOOT_ROLLError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // item_guid: Guid
        let item_guid = Guid::read(r)?;

        // item_slot: u32
        let item_slot = crate::util::read_u32_le(r)?;

        // vote: RollVote
        let vote = RollVote::read(r)?;

        Ok(Self {
            item_guid,
            item_slot,
            vote,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // item_guid: Guid
        self.item_guid.write(w)?;

        // item_slot: u32
        w.write_all(&self.item_slot.to_le_bytes())?;

        // vote: RollVote
        self.vote.write(w)?;

        Ok(())
    }
}

impl ConstantSized for CMSG_LOOT_ROLL {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMSG_LOOT_ROLL {
    fn maximum_possible_size() -> usize {
        8 // item_guid: Guid
        + 4 // item_slot: u32
        + RollVote::size() // vote: RollVote
    }
}

#[derive(Debug)]
pub enum CMSG_LOOT_ROLLError {
    Io(std::io::Error),
    RollVote(RollVoteError),
}

impl std::error::Error for CMSG_LOOT_ROLLError {}
impl std::fmt::Display for CMSG_LOOT_ROLLError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::RollVote(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_LOOT_ROLLError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<RollVoteError> for CMSG_LOOT_ROLLError {
    fn from(e: RollVoteError) -> Self {
        Self::RollVote(e)
    }
}

