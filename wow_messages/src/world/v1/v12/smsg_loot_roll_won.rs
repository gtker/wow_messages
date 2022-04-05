use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{RollVote, RollVoteError};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/new4.wowm:162`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/new4.wowm#L162):
/// ```text
/// smsg SMSG_LOOT_ROLL_WON = 0x29F {
///     u64 looted_target_guid;
///     u32 loot_slot;
///     u32 item_id;
///     u32 item_random_suffix;
///     u32 item_random_property_id;
///     u64 winning_player_guid;
///     u8 winning_roll;
///     RollVote vote;
/// }
/// ```
pub struct SMSG_LOOT_ROLL_WON {
    pub looted_target_guid: u64,
    pub loot_slot: u32,
    pub item_id: u32,
    pub item_random_suffix: u32,
    pub item_random_property_id: u32,
    pub winning_player_guid: u64,
    pub winning_roll: u8,
    pub vote: RollVote,
}

impl WorldServerMessageWrite for SMSG_LOOT_ROLL_WON {
    const OPCODE: u16 = 0x29f;

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
impl WorldMessageBody for SMSG_LOOT_ROLL_WON {
    type Error = SMSG_LOOT_ROLL_WONError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // looted_target_guid: u64
        let looted_target_guid = crate::util::read_u64_le(r)?;

        // loot_slot: u32
        let loot_slot = crate::util::read_u32_le(r)?;

        // item_id: u32
        let item_id = crate::util::read_u32_le(r)?;

        // item_random_suffix: u32
        let item_random_suffix = crate::util::read_u32_le(r)?;

        // item_random_property_id: u32
        let item_random_property_id = crate::util::read_u32_le(r)?;

        // winning_player_guid: u64
        let winning_player_guid = crate::util::read_u64_le(r)?;

        // winning_roll: u8
        let winning_roll = crate::util::read_u8_le(r)?;

        // vote: RollVote
        let vote = RollVote::read(r)?;

        Ok(Self {
            looted_target_guid,
            loot_slot,
            item_id,
            item_random_suffix,
            item_random_property_id,
            winning_player_guid,
            winning_roll,
            vote,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // looted_target_guid: u64
        w.write_all(&self.looted_target_guid.to_le_bytes())?;

        // loot_slot: u32
        w.write_all(&self.loot_slot.to_le_bytes())?;

        // item_id: u32
        w.write_all(&self.item_id.to_le_bytes())?;

        // item_random_suffix: u32
        w.write_all(&self.item_random_suffix.to_le_bytes())?;

        // item_random_property_id: u32
        w.write_all(&self.item_random_property_id.to_le_bytes())?;

        // winning_player_guid: u64
        w.write_all(&self.winning_player_guid.to_le_bytes())?;

        // winning_roll: u8
        w.write_all(&self.winning_roll.to_le_bytes())?;

        // vote: RollVote
        self.vote.write(w)?;

        Ok(())
    }
}

impl ConstantSized for SMSG_LOOT_ROLL_WON {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_LOOT_ROLL_WON {
    fn maximum_possible_size() -> usize {
        8 // looted_target_guid: u64
        + 4 // loot_slot: u32
        + 4 // item_id: u32
        + 4 // item_random_suffix: u32
        + 4 // item_random_property_id: u32
        + 8 // winning_player_guid: u64
        + 1 // winning_roll: u8
        + RollVote::size() // vote: RollVote
    }
}

#[derive(Debug)]
pub enum SMSG_LOOT_ROLL_WONError {
    Io(std::io::Error),
    RollVote(RollVoteError),
}

impl std::error::Error for SMSG_LOOT_ROLL_WONError {}
impl std::fmt::Display for SMSG_LOOT_ROLL_WONError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::RollVote(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_LOOT_ROLL_WONError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<RollVoteError> for SMSG_LOOT_ROLL_WONError {
    fn from(e: RollVoteError) -> Self {
        Self::RollVote(e)
    }
}

