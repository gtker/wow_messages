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
pub struct SMSG_LOOT_ROLL_WON {
    pub looted_target_guid: Guid,
    pub loot_slot: u32,
    pub item_id: u32,
    pub item_random_suffix: u32,
    pub item_random_property_id: u32,
    pub winning_player_guid: Guid,
    pub winning_roll: u8,
    pub vote: RollVote,
}

impl ServerMessageWrite for SMSG_LOOT_ROLL_WON {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_LOOT_ROLL_WON {
    const OPCODE: u16 = 0x029f;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = SMSG_LOOT_ROLL_WONError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // looted_target_guid: Guid
        let looted_target_guid = Guid::read(r)?;

        // loot_slot: u32
        let loot_slot = crate::util::read_u32_le(r)?;

        // item_id: u32
        let item_id = crate::util::read_u32_le(r)?;

        // item_random_suffix: u32
        let item_random_suffix = crate::util::read_u32_le(r)?;

        // item_random_property_id: u32
        let item_random_property_id = crate::util::read_u32_le(r)?;

        // winning_player_guid: Guid
        let winning_player_guid = Guid::read(r)?;

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

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // looted_target_guid: Guid
        self.looted_target_guid.write(w)?;

        // loot_slot: u32
        w.write_all(&self.loot_slot.to_le_bytes())?;

        // item_id: u32
        w.write_all(&self.item_id.to_le_bytes())?;

        // item_random_suffix: u32
        w.write_all(&self.item_random_suffix.to_le_bytes())?;

        // item_random_property_id: u32
        w.write_all(&self.item_random_property_id.to_le_bytes())?;

        // winning_player_guid: Guid
        self.winning_player_guid.write(w)?;

        // winning_roll: u8
        w.write_all(&self.winning_roll.to_le_bytes())?;

        // vote: RollVote
        self.vote.write(w)?;

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // looted_target_guid: Guid
        let looted_target_guid = Guid::tokio_read(r).await?;

        // loot_slot: u32
        let loot_slot = crate::util::tokio_read_u32_le(r).await?;

        // item_id: u32
        let item_id = crate::util::tokio_read_u32_le(r).await?;

        // item_random_suffix: u32
        let item_random_suffix = crate::util::tokio_read_u32_le(r).await?;

        // item_random_property_id: u32
        let item_random_property_id = crate::util::tokio_read_u32_le(r).await?;

        // winning_player_guid: Guid
        let winning_player_guid = Guid::tokio_read(r).await?;

        // winning_roll: u8
        let winning_roll = crate::util::tokio_read_u8_le(r).await?;

        // vote: RollVote
        let vote = RollVote::tokio_read(r).await?;

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

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // looted_target_guid: Guid
        self.looted_target_guid.tokio_write(w).await?;

        // loot_slot: u32
        w.write_all(&self.loot_slot.to_le_bytes()).await?;

        // item_id: u32
        w.write_all(&self.item_id.to_le_bytes()).await?;

        // item_random_suffix: u32
        w.write_all(&self.item_random_suffix.to_le_bytes()).await?;

        // item_random_property_id: u32
        w.write_all(&self.item_random_property_id.to_le_bytes()).await?;

        // winning_player_guid: Guid
        self.winning_player_guid.tokio_write(w).await?;

        // winning_roll: u8
        w.write_all(&self.winning_roll.to_le_bytes()).await?;

        // vote: RollVote
        self.vote.tokio_write(w).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // looted_target_guid: Guid
        let looted_target_guid = Guid::astd_read(r).await?;

        // loot_slot: u32
        let loot_slot = crate::util::astd_read_u32_le(r).await?;

        // item_id: u32
        let item_id = crate::util::astd_read_u32_le(r).await?;

        // item_random_suffix: u32
        let item_random_suffix = crate::util::astd_read_u32_le(r).await?;

        // item_random_property_id: u32
        let item_random_property_id = crate::util::astd_read_u32_le(r).await?;

        // winning_player_guid: Guid
        let winning_player_guid = Guid::astd_read(r).await?;

        // winning_roll: u8
        let winning_roll = crate::util::astd_read_u8_le(r).await?;

        // vote: RollVote
        let vote = RollVote::astd_read(r).await?;

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

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // looted_target_guid: Guid
        self.looted_target_guid.astd_write(w).await?;

        // loot_slot: u32
        w.write_all(&self.loot_slot.to_le_bytes()).await?;

        // item_id: u32
        w.write_all(&self.item_id.to_le_bytes()).await?;

        // item_random_suffix: u32
        w.write_all(&self.item_random_suffix.to_le_bytes()).await?;

        // item_random_property_id: u32
        w.write_all(&self.item_random_property_id.to_le_bytes()).await?;

        // winning_player_guid: Guid
        self.winning_player_guid.astd_write(w).await?;

        // winning_roll: u8
        w.write_all(&self.winning_roll.to_le_bytes()).await?;

        // vote: RollVote
        self.vote.astd_write(w).await?;

        Ok(())
    }

}

impl ConstantSized for SMSG_LOOT_ROLL_WON {}

impl MaximumPossibleSized for SMSG_LOOT_ROLL_WON {
    fn maximum_possible_size() -> usize {
        8 // looted_target_guid: Guid
        + 4 // loot_slot: u32
        + 4 // item_id: u32
        + 4 // item_random_suffix: u32
        + 4 // item_random_property_id: u32
        + 8 // winning_player_guid: Guid
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

