use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{RollVote, RollVoteError};
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
pub struct SMSG_LOOT_ROLL {
    pub creature_guid: Guid,
    pub loot_slot: u32,
    pub item_guid: Guid,
    pub item_id: u32,
    pub item_random_suffix: u32,
    pub item_random_property_id: u32,
    pub roll_number: u8,
    pub vote: RollVote,
}

impl ServerMessageWrite for SMSG_LOOT_ROLL {}

impl MessageBody for SMSG_LOOT_ROLL {
    const OPCODE: u16 = 0x02a2;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = SMSG_LOOT_ROLLError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // creature_guid: Guid
        let creature_guid = Guid::read(r)?;

        // loot_slot: u32
        let loot_slot = crate::util::read_u32_le(r)?;

        // item_guid: Guid
        let item_guid = Guid::read(r)?;

        // item_id: u32
        let item_id = crate::util::read_u32_le(r)?;

        // item_random_suffix: u32
        let item_random_suffix = crate::util::read_u32_le(r)?;

        // item_random_property_id: u32
        let item_random_property_id = crate::util::read_u32_le(r)?;

        // roll_number: u8
        let roll_number = crate::util::read_u8_le(r)?;

        // vote: RollVote
        let vote = RollVote::read(r)?;

        Ok(Self {
            creature_guid,
            loot_slot,
            item_guid,
            item_id,
            item_random_suffix,
            item_random_property_id,
            roll_number,
            vote,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // creature_guid: Guid
        self.creature_guid.write(w)?;

        // loot_slot: u32
        w.write_all(&self.loot_slot.to_le_bytes())?;

        // item_guid: Guid
        self.item_guid.write(w)?;

        // item_id: u32
        w.write_all(&self.item_id.to_le_bytes())?;

        // item_random_suffix: u32
        w.write_all(&self.item_random_suffix.to_le_bytes())?;

        // item_random_property_id: u32
        w.write_all(&self.item_random_property_id.to_le_bytes())?;

        // roll_number: u8
        w.write_all(&self.roll_number.to_le_bytes())?;

        // vote: RollVote
        self.vote.write(w)?;

        Ok(())
    }
}

impl ConstantSized for SMSG_LOOT_ROLL {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_LOOT_ROLL {
    fn maximum_possible_size() -> usize {
        8 // creature_guid: Guid
        + 4 // loot_slot: u32
        + 8 // item_guid: Guid
        + 4 // item_id: u32
        + 4 // item_random_suffix: u32
        + 4 // item_random_property_id: u32
        + 1 // roll_number: u8
        + RollVote::size() // vote: RollVote
    }
}

#[derive(Debug)]
pub enum SMSG_LOOT_ROLLError {
    Io(std::io::Error),
    RollVote(RollVoteError),
}

impl std::error::Error for SMSG_LOOT_ROLLError {}
impl std::fmt::Display for SMSG_LOOT_ROLLError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::RollVote(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_LOOT_ROLLError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<RollVoteError> for SMSG_LOOT_ROLLError {
    fn from(e: RollVoteError) -> Self {
        Self::RollVote(e)
    }
}

