use std::io::{Read, Write};

use crate::Guid;
use crate::tbc::{
    QuestDetailsEmote, QuestItemReward,
};
use wow_world_base::shared::gold_vanilla_tbc_wrath::Gold;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_questgiver_quest_details.wowm:27`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_questgiver_quest_details.wowm#L27):
/// ```text
/// smsg SMSG_QUESTGIVER_QUEST_DETAILS = 0x0188 {
///     Guid guid;
///     u32 quest_id;
///     CString title;
///     CString details;
///     CString objectives;
///     Bool32 auto_finish;
///     u32 suggested_players;
///     u32 amount_of_choice_item_rewards;
///     QuestItemReward[amount_of_choice_item_rewards] choice_item_rewards;
///     u32 amount_of_item_rewards;
///     QuestItemReward[amount_of_item_rewards] item_rewards;
///     Gold money_reward;
///     u32 honor_reward;
///     u32 reward_spell;
///     u32 casted_spell;
///     u32 title_reward;
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
    pub auto_finish: bool,
    pub suggested_players: u32,
    pub choice_item_rewards: Vec<QuestItemReward>,
    pub item_rewards: Vec<QuestItemReward>,
    pub money_reward: Gold,
    pub honor_reward: u32,
    /// mangosone: reward spell, this spell will display (icon) (casted if RewSpellCast==0)
    ///
    pub reward_spell: u32,
    pub casted_spell: u32,
    /// mangosone: CharTitle, new 2.4.0, player gets this title (bit index from CharTitles)
    ///
    pub title_reward: u32,
    pub emotes: Vec<QuestDetailsEmote>,
}

impl crate::private::Sealed for SMSG_QUESTGIVER_QUEST_DETAILS {}
impl crate::Message for SMSG_QUESTGIVER_QUEST_DETAILS {
    const OPCODE: u32 = 0x0188;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
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

        // auto_finish: Bool32
        w.write_all(u32::from(self.auto_finish).to_le_bytes().as_slice())?;

        // suggested_players: u32
        w.write_all(&self.suggested_players.to_le_bytes())?;

        // amount_of_choice_item_rewards: u32
        w.write_all(&(self.choice_item_rewards.len() as u32).to_le_bytes())?;

        // choice_item_rewards: QuestItemReward[amount_of_choice_item_rewards]
        for i in self.choice_item_rewards.iter() {
            i.write_into_vec(&mut w)?;
        }

        // amount_of_item_rewards: u32
        w.write_all(&(self.item_rewards.len() as u32).to_le_bytes())?;

        // item_rewards: QuestItemReward[amount_of_item_rewards]
        for i in self.item_rewards.iter() {
            i.write_into_vec(&mut w)?;
        }

        // money_reward: Gold
        w.write_all(u32::from(self.money_reward.as_int()).to_le_bytes().as_slice())?;

        // honor_reward: u32
        w.write_all(&self.honor_reward.to_le_bytes())?;

        // reward_spell: u32
        w.write_all(&self.reward_spell.to_le_bytes())?;

        // casted_spell: u32
        w.write_all(&self.casted_spell.to_le_bytes())?;

        // title_reward: u32
        w.write_all(&self.title_reward.to_le_bytes())?;

        // amount_of_emotes: u32
        w.write_all(&(self.emotes.len() as u32).to_le_bytes())?;

        // emotes: QuestDetailsEmote[amount_of_emotes]
        for i in self.emotes.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(55..=65535).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0188, size: body_size as u32 });
        }

        // guid: Guid
        let guid = Guid::read(&mut r)?;

        // quest_id: u32
        let quest_id = crate::util::read_u32_le(&mut r)?;

        // title: CString
        let title = {
            let title = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(title)?
        };

        // details: CString
        let details = {
            let details = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(details)?
        };

        // objectives: CString
        let objectives = {
            let objectives = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(objectives)?
        };

        // auto_finish: Bool32
        let auto_finish = crate::util::read_u32_le(&mut r)? != 0;

        // suggested_players: u32
        let suggested_players = crate::util::read_u32_le(&mut r)?;

        // amount_of_choice_item_rewards: u32
        let amount_of_choice_item_rewards = crate::util::read_u32_le(&mut r)?;

        // choice_item_rewards: QuestItemReward[amount_of_choice_item_rewards]
        let choice_item_rewards = {
            let mut choice_item_rewards = Vec::with_capacity(amount_of_choice_item_rewards as usize);
            for i in 0..amount_of_choice_item_rewards {
                choice_item_rewards.push(QuestItemReward::read(&mut r)?);
            }
            choice_item_rewards
        };

        // amount_of_item_rewards: u32
        let amount_of_item_rewards = crate::util::read_u32_le(&mut r)?;

        // item_rewards: QuestItemReward[amount_of_item_rewards]
        let item_rewards = {
            let mut item_rewards = Vec::with_capacity(amount_of_item_rewards as usize);
            for i in 0..amount_of_item_rewards {
                item_rewards.push(QuestItemReward::read(&mut r)?);
            }
            item_rewards
        };

        // money_reward: Gold
        let money_reward = Gold::new(crate::util::read_u32_le(&mut r)?);

        // honor_reward: u32
        let honor_reward = crate::util::read_u32_le(&mut r)?;

        // reward_spell: u32
        let reward_spell = crate::util::read_u32_le(&mut r)?;

        // casted_spell: u32
        let casted_spell = crate::util::read_u32_le(&mut r)?;

        // title_reward: u32
        let title_reward = crate::util::read_u32_le(&mut r)?;

        // amount_of_emotes: u32
        let amount_of_emotes = crate::util::read_u32_le(&mut r)?;

        // emotes: QuestDetailsEmote[amount_of_emotes]
        let emotes = {
            let mut emotes = Vec::with_capacity(amount_of_emotes as usize);
            for i in 0..amount_of_emotes {
                emotes.push(QuestDetailsEmote::read(&mut r)?);
            }
            emotes
        };

        Ok(Self {
            guid,
            quest_id,
            title,
            details,
            objectives,
            auto_finish,
            suggested_players,
            choice_item_rewards,
            item_rewards,
            money_reward,
            honor_reward,
            reward_spell,
            casted_spell,
            title_reward,
            emotes,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_QUESTGIVER_QUEST_DETAILS {}

impl SMSG_QUESTGIVER_QUEST_DETAILS {
    pub(crate) fn size(&self) -> usize {
        8 // guid: Guid
        + 4 // quest_id: u32
        + self.title.len() + 1 // title: CString
        + self.details.len() + 1 // details: CString
        + self.objectives.len() + 1 // objectives: CString
        + 4 // auto_finish: Bool32
        + 4 // suggested_players: u32
        + 4 // amount_of_choice_item_rewards: u32
        + self.choice_item_rewards.len() * 8 // choice_item_rewards: QuestItemReward[amount_of_choice_item_rewards]
        + 4 // amount_of_item_rewards: u32
        + self.item_rewards.len() * 8 // item_rewards: QuestItemReward[amount_of_item_rewards]
        + 4 // money_reward: Gold
        + 4 // honor_reward: u32
        + 4 // reward_spell: u32
        + 4 // casted_spell: u32
        + 4 // title_reward: u32
        + 4 // amount_of_emotes: u32
        + self.emotes.len() * 8 // emotes: QuestDetailsEmote[amount_of_emotes]
    }
}

