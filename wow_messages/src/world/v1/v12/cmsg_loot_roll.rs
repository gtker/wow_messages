use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{RollVote, RollVoteError};
use crate::world::helper::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/cmsg_loot_roll.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/cmsg_loot_roll.wowm#L3):
/// ```text
/// cmsg CMSG_LOOT_ROLL = 0x2A0 {
///     u64 item_guid;
///     u32 item_slot;
///     RollVote vote;
/// }
/// ```
pub struct CMSG_LOOT_ROLL {
    pub item_guid: u64,
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
impl WorldMessageBody for CMSG_LOOT_ROLL {
    type Error = CMSG_LOOT_ROLLError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // item_guid: u64
        let item_guid = crate::util::read_u64_le(r)?;

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
        // item_guid: u64
        w.write_all(&self.item_guid.to_le_bytes())?;

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
        8 // item_guid: u64
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

