use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::vanilla::QuestDetailsEmote;
use crate::world::vanilla::QuestItemReward;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_questgiver_quest_details.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_questgiver_quest_details.wowm#L8):
/// ```text
/// smsg SMSG_QUESTGIVER_QUEST_DETAILS = 0x0188 {
///     Guid guid;
///     u32 quest_id;
///     CString title;
///     CString details;
///     CString objectives;
///     u32 auto_finish;
///     u32 amount_of_choice_item_rewards;
///     QuestItemReward[amount_of_choice_item_rewards] choice_item_rewards;
///     u32 amount_of_item_rewards;
///     QuestItemReward[amount_of_item_rewards] item_rewards;
///     u32 money_reward;
///     u32 reward_spell;
///     u32 amount_of_emotes;
///     QuestDetailsEmote[amount_of_emotes] emotes;
/// }
/// ```
pub struct SMSG_QUESTGIVER_QUEST_DETAILS {
    pub guid: Guid,
    pub quest_id: u32,
    pub title: String,
    pub details: String,
    pub objectives: String,
    /// cmangos/vmangos/mangoszero: sets to 1 for true and 0 for false
    ///
    pub auto_finish: u32,
    pub choice_item_rewards: Vec<QuestItemReward>,
    pub item_rewards: Vec<QuestItemReward>,
    pub money_reward: u32,
    pub reward_spell: u32,
    pub emotes: Vec<QuestDetailsEmote>,
}

impl crate::Message for SMSG_QUESTGIVER_QUEST_DETAILS {
    const OPCODE: u32 = 0x0188;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        // title: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.title.as_bytes().iter().rev().next(), Some(&0_u8), "String `title` must not be null-terminated.");
        w.write_all(self.title.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // details: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.details.as_bytes().iter().rev().next(), Some(&0_u8), "String `details` must not be null-terminated.");
        w.write_all(self.details.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // objectives: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.objectives.as_bytes().iter().rev().next(), Some(&0_u8), "String `objectives` must not be null-terminated.");
        w.write_all(self.objectives.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // auto_finish: u32
        w.write_all(&self.auto_finish.to_le_bytes())?;

        // amount_of_choice_item_rewards: u32
        w.write_all(&(self.choice_item_rewards.len() as u32).to_le_bytes())?;

        // choice_item_rewards: QuestItemReward[amount_of_choice_item_rewards]
        for i in self.choice_item_rewards.iter() {
            i.write_into_vec(w)?;
        }

        // amount_of_item_rewards: u32
        w.write_all(&(self.item_rewards.len() as u32).to_le_bytes())?;

        // item_rewards: QuestItemReward[amount_of_item_rewards]
        for i in self.item_rewards.iter() {
            i.write_into_vec(w)?;
        }

        // money_reward: u32
        w.write_all(&self.money_reward.to_le_bytes())?;

        // reward_spell: u32
        w.write_all(&self.reward_spell.to_le_bytes())?;

        // amount_of_emotes: u32
        w.write_all(&(self.emotes.len() as u32).to_le_bytes())?;

        // emotes: QuestDetailsEmote[amount_of_emotes]
        for i in self.emotes.iter() {
            i.write_into_vec(w)?;
        }

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
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

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_QUESTGIVER_QUEST_DETAILS {}

impl SMSG_QUESTGIVER_QUEST_DETAILS {
    pub(crate) fn size(&self) -> usize {
        8 // guid: Guid
        + 4 // quest_id: u32
        + self.title.len() + 1 // title: CString
        + self.details.len() + 1 // details: CString
        + self.objectives.len() + 1 // objectives: CString
        + 4 // auto_finish: u32
        + 4 // amount_of_choice_item_rewards: u32
        + self.choice_item_rewards.len() * 8 // choice_item_rewards: QuestItemReward[amount_of_choice_item_rewards]
        + 4 // amount_of_item_rewards: u32
        + self.item_rewards.len() * 8 // item_rewards: QuestItemReward[amount_of_item_rewards]
        + 4 // money_reward: u32
        + 4 // reward_spell: u32
        + 4 // amount_of_emotes: u32
        + self.emotes.len() * 8 // emotes: QuestDetailsEmote[amount_of_emotes]
    }
}

