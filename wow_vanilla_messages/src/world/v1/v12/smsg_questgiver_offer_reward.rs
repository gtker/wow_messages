use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::NpcTextUpdateEmote;
use crate::world::v1::v12::QuestItemReward;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

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

impl SMSG_QUESTGIVER_OFFER_REWARD {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
        // npc: Guid
        w.write_all(&self.npc.guid().to_le_bytes())?;

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
            w.write_all(&(i.as_bytes()?))?;
        }

        // amount_of_choice_item_rewards: u32
        w.write_all(&(self.choice_item_rewards.len() as u32).to_le_bytes())?;

        // choice_item_rewards: QuestItemReward[amount_of_choice_item_rewards]
        for i in self.choice_item_rewards.iter() {
            w.write_all(&(i.as_bytes()?))?;
        }

        // amount_of_item_rewards: u32
        w.write_all(&(self.item_rewards.len() as u32).to_le_bytes())?;

        // item_rewards: QuestItemReward[amount_of_item_rewards]
        for i in self.item_rewards.iter() {
            w.write_all(&(i.as_bytes()?))?;
        }

        // money_reward: u32
        w.write_all(&self.money_reward.to_le_bytes())?;

        // reward_spell: u32
        w.write_all(&self.reward_spell.to_le_bytes())?;

        // reward_spell_cast: u32
        w.write_all(&self.reward_spell_cast.to_le_bytes())?;

        Ok(w)
    }
}

impl ServerMessage for SMSG_QUESTGIVER_OFFER_REWARD {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // npc: Guid
        w.write_all(&self.npc.guid().to_le_bytes())?;

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
            w.write_all(&(i.as_bytes()?))?;
        }

        // amount_of_choice_item_rewards: u32
        w.write_all(&(self.choice_item_rewards.len() as u32).to_le_bytes())?;

        // choice_item_rewards: QuestItemReward[amount_of_choice_item_rewards]
        for i in self.choice_item_rewards.iter() {
            w.write_all(&(i.as_bytes()?))?;
        }

        // amount_of_item_rewards: u32
        w.write_all(&(self.item_rewards.len() as u32).to_le_bytes())?;

        // item_rewards: QuestItemReward[amount_of_item_rewards]
        for i in self.item_rewards.iter() {
            w.write_all(&(i.as_bytes()?))?;
        }

        // money_reward: u32
        w.write_all(&self.money_reward.to_le_bytes())?;

        // reward_spell: u32
        w.write_all(&self.reward_spell.to_le_bytes())?;

        // reward_spell_cast: u32
        w.write_all(&self.reward_spell_cast.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x018d;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_QUESTGIVER_OFFER_REWARDError;

    #[cfg(feature = "sync")]
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
            // npc: Guid
            let npc = Guid::tokio_read(r).await?;

            // quest_id: u32
            let quest_id = crate::util::tokio_read_u32_le(r).await?;

            // title: CString
            let title = crate::util::tokio_read_c_string_to_vec(r).await?;
            let title = String::from_utf8(title)?;

            // offer_reward_text: CString
            let offer_reward_text = crate::util::tokio_read_c_string_to_vec(r).await?;
            let offer_reward_text = String::from_utf8(offer_reward_text)?;

            // enable_next: u32
            let enable_next = crate::util::tokio_read_u32_le(r).await?;

            // amount_of_emotes: u32
            let amount_of_emotes = crate::util::tokio_read_u32_le(r).await?;

            // emotes: NpcTextUpdateEmote[amount_of_emotes]
            let mut emotes = Vec::with_capacity(amount_of_emotes as usize);
            for i in 0..amount_of_emotes {
                emotes.push(NpcTextUpdateEmote::tokio_read(r).await?);
            }

            // amount_of_choice_item_rewards: u32
            let amount_of_choice_item_rewards = crate::util::tokio_read_u32_le(r).await?;

            // choice_item_rewards: QuestItemReward[amount_of_choice_item_rewards]
            let mut choice_item_rewards = Vec::with_capacity(amount_of_choice_item_rewards as usize);
            for i in 0..amount_of_choice_item_rewards {
                choice_item_rewards.push(QuestItemReward::tokio_read(r).await?);
            }

            // amount_of_item_rewards: u32
            let amount_of_item_rewards = crate::util::tokio_read_u32_le(r).await?;

            // item_rewards: QuestItemReward[amount_of_item_rewards]
            let mut item_rewards = Vec::with_capacity(amount_of_item_rewards as usize);
            for i in 0..amount_of_item_rewards {
                item_rewards.push(QuestItemReward::tokio_read(r).await?);
            }

            // money_reward: u32
            let money_reward = crate::util::tokio_read_u32_le(r).await?;

            // reward_spell: u32
            let reward_spell = crate::util::tokio_read_u32_le(r).await?;

            // reward_spell_cast: u32
            let reward_spell_cast = crate::util::tokio_read_u32_le(r).await?;

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
            // npc: Guid
            let npc = Guid::astd_read(r).await?;

            // quest_id: u32
            let quest_id = crate::util::astd_read_u32_le(r).await?;

            // title: CString
            let title = crate::util::astd_read_c_string_to_vec(r).await?;
            let title = String::from_utf8(title)?;

            // offer_reward_text: CString
            let offer_reward_text = crate::util::astd_read_c_string_to_vec(r).await?;
            let offer_reward_text = String::from_utf8(offer_reward_text)?;

            // enable_next: u32
            let enable_next = crate::util::astd_read_u32_le(r).await?;

            // amount_of_emotes: u32
            let amount_of_emotes = crate::util::astd_read_u32_le(r).await?;

            // emotes: NpcTextUpdateEmote[amount_of_emotes]
            let mut emotes = Vec::with_capacity(amount_of_emotes as usize);
            for i in 0..amount_of_emotes {
                emotes.push(NpcTextUpdateEmote::astd_read(r).await?);
            }

            // amount_of_choice_item_rewards: u32
            let amount_of_choice_item_rewards = crate::util::astd_read_u32_le(r).await?;

            // choice_item_rewards: QuestItemReward[amount_of_choice_item_rewards]
            let mut choice_item_rewards = Vec::with_capacity(amount_of_choice_item_rewards as usize);
            for i in 0..amount_of_choice_item_rewards {
                choice_item_rewards.push(QuestItemReward::astd_read(r).await?);
            }

            // amount_of_item_rewards: u32
            let amount_of_item_rewards = crate::util::astd_read_u32_le(r).await?;

            // item_rewards: QuestItemReward[amount_of_item_rewards]
            let mut item_rewards = Vec::with_capacity(amount_of_item_rewards as usize);
            for i in 0..amount_of_item_rewards {
                item_rewards.push(QuestItemReward::astd_read(r).await?);
            }

            // money_reward: u32
            let money_reward = crate::util::astd_read_u32_le(r).await?;

            // reward_spell: u32
            let reward_spell = crate::util::astd_read_u32_le(r).await?;

            // reward_spell_cast: u32
            let reward_spell_cast = crate::util::astd_read_u32_le(r).await?;

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
        })
    }

}

impl SMSG_QUESTGIVER_OFFER_REWARD {
    pub fn size(&self) -> usize {
        0
        + 8 // npc: Guid
        + 4 // quest_id: u32
        + self.title.len() + 1 // title: CString
        + self.offer_reward_text.len() + 1 // offer_reward_text: CString
        + 4 // enable_next: u32
        + 4 // amount_of_emotes: u32
        + self.emotes.len() * 8 // emotes: NpcTextUpdateEmote[amount_of_emotes]
        + 4 // amount_of_choice_item_rewards: u32
        + self.choice_item_rewards.len() * 8 // choice_item_rewards: QuestItemReward[amount_of_choice_item_rewards]
        + 4 // amount_of_item_rewards: u32
        + self.item_rewards.len() * 8 // item_rewards: QuestItemReward[amount_of_item_rewards]
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

