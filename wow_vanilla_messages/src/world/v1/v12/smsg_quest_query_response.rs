use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::QuestItemReward;
use crate::world::v1::v12::QuestObjective;
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_QUEST_QUERY_RESPONSE {
    pub quest_id: u32,
    pub quest_method: u32,
    pub quest_level: u32,
    pub zone_or_sort: u32,
    pub quest_type: u32,
    pub reputation_objective_faction: u32,
    pub reputation_objective_value: u32,
    pub required_opposite_faction: u32,
    pub required_opposite_reputation_value: u32,
    pub next_quest_in_chain: u32,
    pub money_reward: u32,
    pub max_level_money_reward: u32,
    pub reward_spell: u32,
    pub source_item_id: u32,
    pub quest_flags: u32,
    pub rewards: [QuestItemReward; 4],
    pub choice_rewards: [QuestItemReward; 6],
    pub point_map_id: u32,
    pub position_x: f32,
    pub position_y: f32,
    pub point_opt: u32,
    pub title: String,
    pub objective_text: String,
    pub details: String,
    pub end_text: String,
    pub objectives: [QuestObjective; 4],
    pub objective_texts: [String; 4],
}

impl ServerMessageWrite for SMSG_QUEST_QUERY_RESPONSE {}

impl SMSG_QUEST_QUERY_RESPONSE {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(2268);
        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        // quest_method: u32
        w.write_all(&self.quest_method.to_le_bytes())?;

        // quest_level: u32
        w.write_all(&self.quest_level.to_le_bytes())?;

        // zone_or_sort: u32
        w.write_all(&self.zone_or_sort.to_le_bytes())?;

        // quest_type: u32
        w.write_all(&self.quest_type.to_le_bytes())?;

        // reputation_objective_faction: u32
        w.write_all(&self.reputation_objective_faction.to_le_bytes())?;

        // reputation_objective_value: u32
        w.write_all(&self.reputation_objective_value.to_le_bytes())?;

        // required_opposite_faction: u32
        w.write_all(&self.required_opposite_faction.to_le_bytes())?;

        // required_opposite_reputation_value: u32
        w.write_all(&self.required_opposite_reputation_value.to_le_bytes())?;

        // next_quest_in_chain: u32
        w.write_all(&self.next_quest_in_chain.to_le_bytes())?;

        // money_reward: u32
        w.write_all(&self.money_reward.to_le_bytes())?;

        // max_level_money_reward: u32
        w.write_all(&self.max_level_money_reward.to_le_bytes())?;

        // reward_spell: u32
        w.write_all(&self.reward_spell.to_le_bytes())?;

        // source_item_id: u32
        w.write_all(&self.source_item_id.to_le_bytes())?;

        // quest_flags: u32
        w.write_all(&self.quest_flags.to_le_bytes())?;

        // rewards: QuestItemReward[4]
        for i in self.rewards.iter() {
            w.write_all(&(i.as_bytes()?))?;
        }

        // choice_rewards: QuestItemReward[6]
        for i in self.choice_rewards.iter() {
            w.write_all(&(i.as_bytes()?))?;
        }

        // point_map_id: u32
        w.write_all(&self.point_map_id.to_le_bytes())?;

        // position_x: f32
        w.write_all(&self.position_x.to_le_bytes())?;

        // position_y: f32
        w.write_all(&self.position_y.to_le_bytes())?;

        // point_opt: u32
        w.write_all(&self.point_opt.to_le_bytes())?;

        // title: CString
        w.write_all(self.title.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // objective_text: CString
        w.write_all(self.objective_text.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // details: CString
        w.write_all(self.details.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // end_text: CString
        w.write_all(self.end_text.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // objectives: QuestObjective[4]
        for i in self.objectives.iter() {
            w.write_all(&(i.as_bytes()?))?;
        }

        // objective_texts: CString[4]
        for i in self.objective_texts.iter() {
            w.write_all(&i.as_bytes())?;
            w.write_all(&[0])?;
        }

        Ok(w)
    }
}

impl MessageBody for SMSG_QUEST_QUERY_RESPONSE {
    const OPCODE: u16 = 0x005d;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_QUEST_QUERY_RESPONSEError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // quest_id: u32
        let quest_id = crate::util::read_u32_le(r)?;

        // quest_method: u32
        let quest_method = crate::util::read_u32_le(r)?;

        // quest_level: u32
        let quest_level = crate::util::read_u32_le(r)?;

        // zone_or_sort: u32
        let zone_or_sort = crate::util::read_u32_le(r)?;

        // quest_type: u32
        let quest_type = crate::util::read_u32_le(r)?;

        // reputation_objective_faction: u32
        let reputation_objective_faction = crate::util::read_u32_le(r)?;

        // reputation_objective_value: u32
        let reputation_objective_value = crate::util::read_u32_le(r)?;

        // required_opposite_faction: u32
        let required_opposite_faction = crate::util::read_u32_le(r)?;

        // required_opposite_reputation_value: u32
        let required_opposite_reputation_value = crate::util::read_u32_le(r)?;

        // next_quest_in_chain: u32
        let next_quest_in_chain = crate::util::read_u32_le(r)?;

        // money_reward: u32
        let money_reward = crate::util::read_u32_le(r)?;

        // max_level_money_reward: u32
        let max_level_money_reward = crate::util::read_u32_le(r)?;

        // reward_spell: u32
        let reward_spell = crate::util::read_u32_le(r)?;

        // source_item_id: u32
        let source_item_id = crate::util::read_u32_le(r)?;

        // quest_flags: u32
        let quest_flags = crate::util::read_u32_le(r)?;

        // rewards: QuestItemReward[4]
        let mut rewards = Vec::with_capacity(4 as usize);
        for i in 0..4 {
            rewards.push(QuestItemReward::read(r)?);
        }
        let rewards = rewards.try_into().unwrap();

        // choice_rewards: QuestItemReward[6]
        let mut choice_rewards = Vec::with_capacity(6 as usize);
        for i in 0..6 {
            choice_rewards.push(QuestItemReward::read(r)?);
        }
        let choice_rewards = choice_rewards.try_into().unwrap();

        // point_map_id: u32
        let point_map_id = crate::util::read_u32_le(r)?;

        // position_x: f32
        let position_x = crate::util::read_f32_le(r)?;
        // position_y: f32
        let position_y = crate::util::read_f32_le(r)?;
        // point_opt: u32
        let point_opt = crate::util::read_u32_le(r)?;

        // title: CString
        let title = crate::util::read_c_string_to_vec(r)?;
        let title = String::from_utf8(title)?;

        // objective_text: CString
        let objective_text = crate::util::read_c_string_to_vec(r)?;
        let objective_text = String::from_utf8(objective_text)?;

        // details: CString
        let details = crate::util::read_c_string_to_vec(r)?;
        let details = String::from_utf8(details)?;

        // end_text: CString
        let end_text = crate::util::read_c_string_to_vec(r)?;
        let end_text = String::from_utf8(end_text)?;

        // objectives: QuestObjective[4]
        let mut objectives = Vec::with_capacity(4 as usize);
        for i in 0..4 {
            objectives.push(QuestObjective::read(r)?);
        }
        let objectives = objectives.try_into().unwrap();

        // objective_texts: CString[4]
        let mut objective_texts = Vec::with_capacity(4 as usize);
        for i in 0..4 {
            let s = crate::util::read_c_string_to_vec(r)?;
            objective_texts[i] = String::from_utf8(s)?;
        }
        let objective_texts = objective_texts.try_into().unwrap();

        Ok(Self {
            quest_id,
            quest_method,
            quest_level,
            zone_or_sort,
            quest_type,
            reputation_objective_faction,
            reputation_objective_value,
            required_opposite_faction,
            required_opposite_reputation_value,
            next_quest_in_chain,
            money_reward,
            max_level_money_reward,
            reward_spell,
            source_item_id,
            quest_flags,
            rewards,
            choice_rewards,
            point_map_id,
            position_x,
            position_y,
            point_opt,
            title,
            objective_text,
            details,
            end_text,
            objectives,
            objective_texts,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let inner = self.as_bytes()?;
        w.write_all(&inner)
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
            // quest_id: u32
            let quest_id = crate::util::tokio_read_u32_le(r).await?;

            // quest_method: u32
            let quest_method = crate::util::tokio_read_u32_le(r).await?;

            // quest_level: u32
            let quest_level = crate::util::tokio_read_u32_le(r).await?;

            // zone_or_sort: u32
            let zone_or_sort = crate::util::tokio_read_u32_le(r).await?;

            // quest_type: u32
            let quest_type = crate::util::tokio_read_u32_le(r).await?;

            // reputation_objective_faction: u32
            let reputation_objective_faction = crate::util::tokio_read_u32_le(r).await?;

            // reputation_objective_value: u32
            let reputation_objective_value = crate::util::tokio_read_u32_le(r).await?;

            // required_opposite_faction: u32
            let required_opposite_faction = crate::util::tokio_read_u32_le(r).await?;

            // required_opposite_reputation_value: u32
            let required_opposite_reputation_value = crate::util::tokio_read_u32_le(r).await?;

            // next_quest_in_chain: u32
            let next_quest_in_chain = crate::util::tokio_read_u32_le(r).await?;

            // money_reward: u32
            let money_reward = crate::util::tokio_read_u32_le(r).await?;

            // max_level_money_reward: u32
            let max_level_money_reward = crate::util::tokio_read_u32_le(r).await?;

            // reward_spell: u32
            let reward_spell = crate::util::tokio_read_u32_le(r).await?;

            // source_item_id: u32
            let source_item_id = crate::util::tokio_read_u32_le(r).await?;

            // quest_flags: u32
            let quest_flags = crate::util::tokio_read_u32_le(r).await?;

            // rewards: QuestItemReward[4]
            let mut rewards = Vec::with_capacity(4 as usize);
            for i in 0..4 {
                rewards.push(QuestItemReward::tokio_read(r).await?);
            }
            let rewards = rewards.try_into().unwrap();

            // choice_rewards: QuestItemReward[6]
            let mut choice_rewards = Vec::with_capacity(6 as usize);
            for i in 0..6 {
                choice_rewards.push(QuestItemReward::tokio_read(r).await?);
            }
            let choice_rewards = choice_rewards.try_into().unwrap();

            // point_map_id: u32
            let point_map_id = crate::util::tokio_read_u32_le(r).await?;

            // position_x: f32
            let position_x = crate::util::tokio_read_f32_le(r).await?;
            // position_y: f32
            let position_y = crate::util::tokio_read_f32_le(r).await?;
            // point_opt: u32
            let point_opt = crate::util::tokio_read_u32_le(r).await?;

            // title: CString
            let title = crate::util::tokio_read_c_string_to_vec(r).await?;
            let title = String::from_utf8(title)?;

            // objective_text: CString
            let objective_text = crate::util::tokio_read_c_string_to_vec(r).await?;
            let objective_text = String::from_utf8(objective_text)?;

            // details: CString
            let details = crate::util::tokio_read_c_string_to_vec(r).await?;
            let details = String::from_utf8(details)?;

            // end_text: CString
            let end_text = crate::util::tokio_read_c_string_to_vec(r).await?;
            let end_text = String::from_utf8(end_text)?;

            // objectives: QuestObjective[4]
            let mut objectives = Vec::with_capacity(4 as usize);
            for i in 0..4 {
                objectives.push(QuestObjective::tokio_read(r).await?);
            }
            let objectives = objectives.try_into().unwrap();

            // objective_texts: CString[4]
            let mut objective_texts = Vec::with_capacity(4 as usize);
            for i in 0..4 {
                let s = crate::util::tokio_read_c_string_to_vec(r).await?;
                objective_texts[i] = String::from_utf8(s)?;
            }
            let objective_texts = objective_texts.try_into().unwrap();

            Ok(Self {
                quest_id,
                quest_method,
                quest_level,
                zone_or_sort,
                quest_type,
                reputation_objective_faction,
                reputation_objective_value,
                required_opposite_faction,
                required_opposite_reputation_value,
                next_quest_in_chain,
                money_reward,
                max_level_money_reward,
                reward_spell,
                source_item_id,
                quest_flags,
                rewards,
                choice_rewards,
                point_map_id,
                position_x,
                position_y,
                point_opt,
                title,
                objective_text,
                details,
                end_text,
                objectives,
                objective_texts,
            })
        })
    }

    #[cfg(feature = "tokio")]
    fn tokio_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let inner = self.as_bytes()?;
            w.write_all(&inner).await
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
            // quest_id: u32
            let quest_id = crate::util::astd_read_u32_le(r).await?;

            // quest_method: u32
            let quest_method = crate::util::astd_read_u32_le(r).await?;

            // quest_level: u32
            let quest_level = crate::util::astd_read_u32_le(r).await?;

            // zone_or_sort: u32
            let zone_or_sort = crate::util::astd_read_u32_le(r).await?;

            // quest_type: u32
            let quest_type = crate::util::astd_read_u32_le(r).await?;

            // reputation_objective_faction: u32
            let reputation_objective_faction = crate::util::astd_read_u32_le(r).await?;

            // reputation_objective_value: u32
            let reputation_objective_value = crate::util::astd_read_u32_le(r).await?;

            // required_opposite_faction: u32
            let required_opposite_faction = crate::util::astd_read_u32_le(r).await?;

            // required_opposite_reputation_value: u32
            let required_opposite_reputation_value = crate::util::astd_read_u32_le(r).await?;

            // next_quest_in_chain: u32
            let next_quest_in_chain = crate::util::astd_read_u32_le(r).await?;

            // money_reward: u32
            let money_reward = crate::util::astd_read_u32_le(r).await?;

            // max_level_money_reward: u32
            let max_level_money_reward = crate::util::astd_read_u32_le(r).await?;

            // reward_spell: u32
            let reward_spell = crate::util::astd_read_u32_le(r).await?;

            // source_item_id: u32
            let source_item_id = crate::util::astd_read_u32_le(r).await?;

            // quest_flags: u32
            let quest_flags = crate::util::astd_read_u32_le(r).await?;

            // rewards: QuestItemReward[4]
            let mut rewards = Vec::with_capacity(4 as usize);
            for i in 0..4 {
                rewards.push(QuestItemReward::astd_read(r).await?);
            }
            let rewards = rewards.try_into().unwrap();

            // choice_rewards: QuestItemReward[6]
            let mut choice_rewards = Vec::with_capacity(6 as usize);
            for i in 0..6 {
                choice_rewards.push(QuestItemReward::astd_read(r).await?);
            }
            let choice_rewards = choice_rewards.try_into().unwrap();

            // point_map_id: u32
            let point_map_id = crate::util::astd_read_u32_le(r).await?;

            // position_x: f32
            let position_x = crate::util::astd_read_f32_le(r).await?;
            // position_y: f32
            let position_y = crate::util::astd_read_f32_le(r).await?;
            // point_opt: u32
            let point_opt = crate::util::astd_read_u32_le(r).await?;

            // title: CString
            let title = crate::util::astd_read_c_string_to_vec(r).await?;
            let title = String::from_utf8(title)?;

            // objective_text: CString
            let objective_text = crate::util::astd_read_c_string_to_vec(r).await?;
            let objective_text = String::from_utf8(objective_text)?;

            // details: CString
            let details = crate::util::astd_read_c_string_to_vec(r).await?;
            let details = String::from_utf8(details)?;

            // end_text: CString
            let end_text = crate::util::astd_read_c_string_to_vec(r).await?;
            let end_text = String::from_utf8(end_text)?;

            // objectives: QuestObjective[4]
            let mut objectives = Vec::with_capacity(4 as usize);
            for i in 0..4 {
                objectives.push(QuestObjective::astd_read(r).await?);
            }
            let objectives = objectives.try_into().unwrap();

            // objective_texts: CString[4]
            let mut objective_texts = Vec::with_capacity(4 as usize);
            for i in 0..4 {
                let s = crate::util::astd_read_c_string_to_vec(r).await?;
                objective_texts[i] = String::from_utf8(s)?;
            }
            let objective_texts = objective_texts.try_into().unwrap();

            Ok(Self {
                quest_id,
                quest_method,
                quest_level,
                zone_or_sort,
                quest_type,
                reputation_objective_faction,
                reputation_objective_value,
                required_opposite_faction,
                required_opposite_reputation_value,
                next_quest_in_chain,
                money_reward,
                max_level_money_reward,
                reward_spell,
                source_item_id,
                quest_flags,
                rewards,
                choice_rewards,
                point_map_id,
                position_x,
                position_y,
                point_opt,
                title,
                objective_text,
                details,
                end_text,
                objectives,
                objective_texts,
            })
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let inner = self.as_bytes()?;
            w.write_all(&inner).await
        })
    }

}

impl SMSG_QUEST_QUERY_RESPONSE {
    pub fn size(&self) -> usize {
        0
        + 4 // quest_id: u32
        + 4 // quest_method: u32
        + 4 // quest_level: u32
        + 4 // zone_or_sort: u32
        + 4 // quest_type: u32
        + 4 // reputation_objective_faction: u32
        + 4 // reputation_objective_value: u32
        + 4 // required_opposite_faction: u32
        + 4 // required_opposite_reputation_value: u32
        + 4 // next_quest_in_chain: u32
        + 4 // money_reward: u32
        + 4 // max_level_money_reward: u32
        + 4 // reward_spell: u32
        + 4 // source_item_id: u32
        + 4 // quest_flags: u32
        + 4 * QuestItemReward::size() // rewards: QuestItemReward[4]
        + 6 * QuestItemReward::size() // choice_rewards: QuestItemReward[6]
        + 4 // point_map_id: u32
        + 4 // position_x: f32
        + 4 // position_y: f32
        + 4 // point_opt: u32
        + self.title.len() + 1 // title: CString
        + self.objective_text.len() + 1 // objective_text: CString
        + self.details.len() + 1 // details: CString
        + self.end_text.len() + 1 // end_text: CString
        + 4 * QuestObjective::size() // objectives: QuestObjective[4]
        + self.objective_texts.iter().fold(0, |acc, x| acc + x.len() + 1) // objective_texts: CString[4]
    }
}

#[derive(Debug)]
pub enum SMSG_QUEST_QUERY_RESPONSEError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for SMSG_QUEST_QUERY_RESPONSEError {}
impl std::fmt::Display for SMSG_QUEST_QUERY_RESPONSEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_QUEST_QUERY_RESPONSEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_QUEST_QUERY_RESPONSEError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

