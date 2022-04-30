use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::QuestItemReward;
use crate::{WorldServerMessageWrite, MessageBody};
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
pub struct SMSG_QUESTGIVER_QUEST_COMPLETE {
    pub quest_id: u32,
    pub unknown: u32,
    pub experience_reward: u32,
    pub money_reward: u32,
    pub item_rewards: Vec<QuestItemReward>,
}

impl WorldServerMessageWrite for SMSG_QUESTGIVER_QUEST_COMPLETE {
    const OPCODE: u16 = 0x191;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (self.size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (self.size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl MessageBody for SMSG_QUESTGIVER_QUEST_COMPLETE {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // quest_id: u32
        let quest_id = crate::util::read_u32_le(r)?;

        // unknown: u32
        let unknown = crate::util::read_u32_le(r)?;

        // experience_reward: u32
        let experience_reward = crate::util::read_u32_le(r)?;

        // money_reward: u32
        let money_reward = crate::util::read_u32_le(r)?;

        // amount_of_item_rewards: u32
        let amount_of_item_rewards = crate::util::read_u32_le(r)?;

        // item_rewards: QuestItemReward[amount_of_item_rewards]
        let mut item_rewards = Vec::with_capacity(amount_of_item_rewards as usize);
        for i in 0..amount_of_item_rewards {
            item_rewards.push(QuestItemReward::read(r)?);
        }

        Ok(Self {
            quest_id,
            unknown,
            experience_reward,
            money_reward,
            item_rewards,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        // unknown: u32
        w.write_all(&self.unknown.to_le_bytes())?;

        // experience_reward: u32
        w.write_all(&self.experience_reward.to_le_bytes())?;

        // money_reward: u32
        w.write_all(&self.money_reward.to_le_bytes())?;

        // amount_of_item_rewards: u32
        w.write_all(&(self.item_rewards.len() as u32).to_le_bytes())?;

        // item_rewards: QuestItemReward[amount_of_item_rewards]
        for i in self.item_rewards.iter() {
            i.write(w)?;
        }

        Ok(())
    }
}

impl VariableSized for SMSG_QUESTGIVER_QUEST_COMPLETE {
    fn size(&self) -> usize {
        4 // quest_id: u32
        + 4 // unknown: u32
        + 4 // experience_reward: u32
        + 4 // money_reward: u32
        + 4 // amount_of_item_rewards: u32
        + self.item_rewards.iter().fold(0, |acc, x| acc + QuestItemReward::size()) // item_rewards: QuestItemReward[amount_of_item_rewards]
    }
}

impl MaximumPossibleSized for SMSG_QUESTGIVER_QUEST_COMPLETE {
    fn maximum_possible_size() -> usize {
        4 // quest_id: u32
        + 4 // unknown: u32
        + 4 // experience_reward: u32
        + 4 // money_reward: u32
        + 4 // amount_of_item_rewards: u32
        + 4294967295 * QuestItemReward::maximum_possible_size() // item_rewards: QuestItemReward[amount_of_item_rewards]
    }
}

