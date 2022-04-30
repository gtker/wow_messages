use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::QuestDetailsEmote;
use crate::world::v1::v12::QuestItemReward;
use crate::{ServerMessageWrite, MessageBody};
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
pub struct SMSG_QUESTGIVER_QUEST_DETAILS {
    pub guid: Guid,
    pub quest_id: u32,
    pub title: String,
    pub details: String,
    pub objectives: String,
    pub auto_finish: u32,
    pub choice_item_rewards: Vec<QuestItemReward>,
    pub item_rewards: Vec<QuestItemReward>,
    pub money_reward: u32,
    pub reward_spell: u32,
    pub emotes: Vec<QuestDetailsEmote>,
}

impl ServerMessageWrite for SMSG_QUESTGIVER_QUEST_DETAILS {
    const OPCODE: u16 = 0x188;

    fn size_without_size_field(&self) -> u16 {
        self.size() as u16
    }

}

impl MessageBody for SMSG_QUESTGIVER_QUEST_DETAILS {
    type Error = SMSG_QUESTGIVER_QUEST_DETAILSError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // quest_id: u32
        let quest_id = crate::util::read_u32_le(r)?;

        // title: CString
        let title = crate::util::read_c_string_to_vec(r)?;
        let title = String::from_utf8(title)?;

        // details: CString
        let details = crate::util::read_c_string_to_vec(r)?;
        let details = String::from_utf8(details)?;

        // objectives: CString
        let objectives = crate::util::read_c_string_to_vec(r)?;
        let objectives = String::from_utf8(objectives)?;

        // auto_finish: u32
        let auto_finish = crate::util::read_u32_le(r)?;

        // amount_of_choice_item_rewards: u32
        let amount_of_choice_item_rewards = crate::util::read_u32_le(r)?;

        // choice_item_rewards: QuestItemReward[amount_of_choice_item_rewards]
        let mut choice_item_rewards = Vec::with_capacity(amount_of_choice_item_rewards as usize);
        for i in 0..amount_of_choice_item_rewards {
            choice_item_rewards.push(QuestItemReward::read(r)?);
        }

        // amount_of_item_rewards: u32
        let amount_of_item_rewards = crate::util::read_u32_le(r)?;

        // item_rewards: QuestItemReward[amount_of_item_rewards]
        let mut item_rewards = Vec::with_capacity(amount_of_item_rewards as usize);
        for i in 0..amount_of_item_rewards {
            item_rewards.push(QuestItemReward::read(r)?);
        }

        // money_reward: u32
        let money_reward = crate::util::read_u32_le(r)?;

        // reward_spell: u32
        let reward_spell = crate::util::read_u32_le(r)?;

        // amount_of_emotes: u32
        let amount_of_emotes = crate::util::read_u32_le(r)?;

        // emotes: QuestDetailsEmote[amount_of_emotes]
        let mut emotes = Vec::with_capacity(amount_of_emotes as usize);
        for i in 0..amount_of_emotes {
            emotes.push(QuestDetailsEmote::read(r)?);
        }

        Ok(Self {
            guid,
            quest_id,
            title,
            details,
            objectives,
            auto_finish,
            choice_item_rewards,
            item_rewards,
            money_reward,
            reward_spell,
            emotes,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        // title: CString
        w.write_all(self.title.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // details: CString
        w.write_all(self.details.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // objectives: CString
        w.write_all(self.objectives.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // auto_finish: u32
        w.write_all(&self.auto_finish.to_le_bytes())?;

        // amount_of_choice_item_rewards: u32
        w.write_all(&(self.choice_item_rewards.len() as u32).to_le_bytes())?;

        // choice_item_rewards: QuestItemReward[amount_of_choice_item_rewards]
        for i in self.choice_item_rewards.iter() {
            i.write(w)?;
        }

        // amount_of_item_rewards: u32
        w.write_all(&(self.item_rewards.len() as u32).to_le_bytes())?;

        // item_rewards: QuestItemReward[amount_of_item_rewards]
        for i in self.item_rewards.iter() {
            i.write(w)?;
        }

        // money_reward: u32
        w.write_all(&self.money_reward.to_le_bytes())?;

        // reward_spell: u32
        w.write_all(&self.reward_spell.to_le_bytes())?;

        // amount_of_emotes: u32
        w.write_all(&(self.emotes.len() as u32).to_le_bytes())?;

        // emotes: QuestDetailsEmote[amount_of_emotes]
        for i in self.emotes.iter() {
            i.write(w)?;
        }

        Ok(())
    }
}

impl VariableSized for SMSG_QUESTGIVER_QUEST_DETAILS {
    fn size(&self) -> usize {
        8 // guid: Guid
        + 4 // quest_id: u32
        + self.title.len() + 1 // title: CString and Null Terminator
        + self.details.len() + 1 // details: CString and Null Terminator
        + self.objectives.len() + 1 // objectives: CString and Null Terminator
        + 4 // auto_finish: u32
        + 4 // amount_of_choice_item_rewards: u32
        + self.choice_item_rewards.iter().fold(0, |acc, x| acc + QuestItemReward::size()) // choice_item_rewards: QuestItemReward[amount_of_choice_item_rewards]
        + 4 // amount_of_item_rewards: u32
        + self.item_rewards.iter().fold(0, |acc, x| acc + QuestItemReward::size()) // item_rewards: QuestItemReward[amount_of_item_rewards]
        + 4 // money_reward: u32
        + 4 // reward_spell: u32
        + 4 // amount_of_emotes: u32
        + self.emotes.iter().fold(0, |acc, x| acc + QuestDetailsEmote::size()) // emotes: QuestDetailsEmote[amount_of_emotes]
    }
}

impl MaximumPossibleSized for SMSG_QUESTGIVER_QUEST_DETAILS {
    fn maximum_possible_size() -> usize {
        8 // guid: Guid
        + 4 // quest_id: u32
        + 256 // title: CString
        + 256 // details: CString
        + 256 // objectives: CString
        + 4 // auto_finish: u32
        + 4 // amount_of_choice_item_rewards: u32
        + 4294967295 * QuestItemReward::maximum_possible_size() // choice_item_rewards: QuestItemReward[amount_of_choice_item_rewards]
        + 4 // amount_of_item_rewards: u32
        + 4294967295 * QuestItemReward::maximum_possible_size() // item_rewards: QuestItemReward[amount_of_item_rewards]
        + 4 // money_reward: u32
        + 4 // reward_spell: u32
        + 4 // amount_of_emotes: u32
        + 4294967295 * QuestDetailsEmote::maximum_possible_size() // emotes: QuestDetailsEmote[amount_of_emotes]
    }
}

#[derive(Debug)]
pub enum SMSG_QUESTGIVER_QUEST_DETAILSError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for SMSG_QUESTGIVER_QUEST_DETAILSError {}
impl std::fmt::Display for SMSG_QUESTGIVER_QUEST_DETAILSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_QUESTGIVER_QUEST_DETAILSError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_QUESTGIVER_QUEST_DETAILSError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

