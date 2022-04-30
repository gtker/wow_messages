use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::NpcTextUpdateEmote;
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
pub struct SMSG_QUESTGIVER_OFFER_REWARD {
    pub npc: Guid,
    pub quest_id: u32,
    pub title: String,
    pub offer_reward_text: String,
    pub enable_next: u32,
    pub emotes: Vec<NpcTextUpdateEmote>,
    pub choice_item_rewards: Vec<QuestItemReward>,
    pub item_rewards: Vec<QuestItemReward>,
    pub money_reward: u32,
    pub reward_spell: u32,
    pub reward_spell_cast: u32,
}

impl ServerMessageWrite for SMSG_QUESTGIVER_OFFER_REWARD {}

impl MessageBody for SMSG_QUESTGIVER_OFFER_REWARD {
    const OPCODE: u16 = 0x018d;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_QUESTGIVER_OFFER_REWARDError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // npc: Guid
        let npc = Guid::read(r)?;

        // quest_id: u32
        let quest_id = crate::util::read_u32_le(r)?;

        // title: CString
        let title = crate::util::read_c_string_to_vec(r)?;
        let title = String::from_utf8(title)?;

        // offer_reward_text: CString
        let offer_reward_text = crate::util::read_c_string_to_vec(r)?;
        let offer_reward_text = String::from_utf8(offer_reward_text)?;

        // enable_next: u32
        let enable_next = crate::util::read_u32_le(r)?;

        // amount_of_emotes: u32
        let amount_of_emotes = crate::util::read_u32_le(r)?;

        // emotes: NpcTextUpdateEmote[amount_of_emotes]
        let mut emotes = Vec::with_capacity(amount_of_emotes as usize);
        for i in 0..amount_of_emotes {
            emotes.push(NpcTextUpdateEmote::read(r)?);
        }

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

        // reward_spell_cast: u32
        let reward_spell_cast = crate::util::read_u32_le(r)?;

        Ok(Self {
            npc,
            quest_id,
            title,
            offer_reward_text,
            enable_next,
            emotes,
            choice_item_rewards,
            item_rewards,
            money_reward,
            reward_spell,
            reward_spell_cast,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // npc: Guid
        self.npc.write(w)?;

        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        // title: CString
        w.write_all(self.title.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // offer_reward_text: CString
        w.write_all(self.offer_reward_text.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // enable_next: u32
        w.write_all(&self.enable_next.to_le_bytes())?;

        // amount_of_emotes: u32
        w.write_all(&(self.emotes.len() as u32).to_le_bytes())?;

        // emotes: NpcTextUpdateEmote[amount_of_emotes]
        for i in self.emotes.iter() {
            i.write(w)?;
        }

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

        // reward_spell_cast: u32
        w.write_all(&self.reward_spell_cast.to_le_bytes())?;

        Ok(())
    }
}

impl VariableSized for SMSG_QUESTGIVER_OFFER_REWARD {
    fn size(&self) -> usize {
        8 // npc: Guid
        + 4 // quest_id: u32
        + self.title.len() + 1 // title: CString and Null Terminator
        + self.offer_reward_text.len() + 1 // offer_reward_text: CString and Null Terminator
        + 4 // enable_next: u32
        + 4 // amount_of_emotes: u32
        + self.emotes.iter().fold(0, |acc, x| acc + NpcTextUpdateEmote::size()) // emotes: NpcTextUpdateEmote[amount_of_emotes]
        + 4 // amount_of_choice_item_rewards: u32
        + self.choice_item_rewards.iter().fold(0, |acc, x| acc + QuestItemReward::size()) // choice_item_rewards: QuestItemReward[amount_of_choice_item_rewards]
        + 4 // amount_of_item_rewards: u32
        + self.item_rewards.iter().fold(0, |acc, x| acc + QuestItemReward::size()) // item_rewards: QuestItemReward[amount_of_item_rewards]
        + 4 // money_reward: u32
        + 4 // reward_spell: u32
        + 4 // reward_spell_cast: u32
    }
}

impl MaximumPossibleSized for SMSG_QUESTGIVER_OFFER_REWARD {
    fn maximum_possible_size() -> usize {
        8 // npc: Guid
        + 4 // quest_id: u32
        + 256 // title: CString
        + 256 // offer_reward_text: CString
        + 4 // enable_next: u32
        + 4 // amount_of_emotes: u32
        + 4294967295 * NpcTextUpdateEmote::maximum_possible_size() // emotes: NpcTextUpdateEmote[amount_of_emotes]
        + 4 // amount_of_choice_item_rewards: u32
        + 4294967295 * QuestItemReward::maximum_possible_size() // choice_item_rewards: QuestItemReward[amount_of_choice_item_rewards]
        + 4 // amount_of_item_rewards: u32
        + 4294967295 * QuestItemReward::maximum_possible_size() // item_rewards: QuestItemReward[amount_of_item_rewards]
        + 4 // money_reward: u32
        + 4 // reward_spell: u32
        + 4 // reward_spell_cast: u32
    }
}

#[derive(Debug)]
pub enum SMSG_QUESTGIVER_OFFER_REWARDError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for SMSG_QUESTGIVER_OFFER_REWARDError {}
impl std::fmt::Display for SMSG_QUESTGIVER_OFFER_REWARDError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_QUESTGIVER_OFFER_REWARDError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_QUESTGIVER_OFFER_REWARDError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

