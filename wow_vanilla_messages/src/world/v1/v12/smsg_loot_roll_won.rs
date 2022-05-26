use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::RollVote;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

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

impl SMSG_LOOT_ROLL_WON {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 34], std::io::Error> {
        let mut array_w = [0u8; 34];
        let mut w = array_w.as_mut_slice();
        // looted_target_guid: Guid
        w.write_all(&self.looted_target_guid.guid().to_le_bytes())?;

        // loot_slot: u32
        w.write_all(&self.loot_slot.to_le_bytes())?;

        // item_id: u32
        w.write_all(&self.item_id.to_le_bytes())?;

        // item_random_suffix: u32
        w.write_all(&self.item_random_suffix.to_le_bytes())?;

        // item_random_property_id: u32
        w.write_all(&self.item_random_property_id.to_le_bytes())?;

        // winning_player_guid: Guid
        w.write_all(&self.winning_player_guid.guid().to_le_bytes())?;

        // winning_roll: u8
        w.write_all(&self.winning_roll.to_le_bytes())?;

        // vote: RollVote
        w.write_all(&(self.vote.as_int() as u8).to_le_bytes())?;

        Ok(array_w)
    }
}

impl ServerMessage for SMSG_LOOT_ROLL_WON {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // looted_target_guid: Guid
        w.write_all(&self.looted_target_guid.guid().to_le_bytes())?;

        // loot_slot: u32
        w.write_all(&self.loot_slot.to_le_bytes())?;

        // item_id: u32
        w.write_all(&self.item_id.to_le_bytes())?;

        // item_random_suffix: u32
        w.write_all(&self.item_random_suffix.to_le_bytes())?;

        // item_random_property_id: u32
        w.write_all(&self.item_random_property_id.to_le_bytes())?;

        // winning_player_guid: Guid
        w.write_all(&self.winning_player_guid.guid().to_le_bytes())?;

        // winning_roll: u8
        w.write_all(&self.winning_roll.to_le_bytes())?;

        // vote: RollVote
        w.write_all(&(self.vote.as_int() as u8).to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x029f;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        34
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
        let vote: RollVote = crate::util::read_u8_le(r)?.try_into()?;

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

    #[cfg(feature = "tokio")]
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
            let vote: RollVote = crate::util::tokio_read_u8_le(r).await?.try_into()?;

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
        })
    }

    #[cfg(feature = "async-std")]
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
            let vote: RollVote = crate::util::astd_read_u8_le(r).await?.try_into()?;

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
        })
    }

}

#[derive(Debug)]
pub enum SMSG_LOOT_ROLL_WONError {
    Io(std::io::Error),
    Enum(crate::errors::EnumError),
}

impl std::error::Error for SMSG_LOOT_ROLL_WONError {}
impl std::fmt::Display for SMSG_LOOT_ROLL_WONError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::Enum(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_LOOT_ROLL_WONError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<crate::errors::EnumError> for SMSG_LOOT_ROLL_WONError {
    fn from(e: crate::errors::EnumError) -> Self {
        Self::Enum(e)
    }
}

