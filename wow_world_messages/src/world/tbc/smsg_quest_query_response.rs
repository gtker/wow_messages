use std::convert::{TryFrom, TryInto};
use crate::tbc::QuestItemReward;
use crate::tbc::QuestObjective;
use crate::tbc::Vector2d;
use crate::tbc::Faction;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_quest_query_response.wowm:57`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_quest_query_response.wowm#L57):
/// ```text
/// smsg SMSG_QUEST_QUERY_RESPONSE = 0x005D {
///     u32 quest_id;
///     u32 quest_method;
///     u32 quest_level;
///     u32 zone_or_sort;
///     u32 quest_type;
///     u32 suggest_player_amount;
///     Faction reputation_objective_faction;
///     u32 reputation_objective_value;
///     Faction required_opposite_faction;
///     u32 required_opposite_reputation_value;
///     u32 next_quest_in_chain;
///     u32 money_reward;
///     u32 max_level_money_reward;
///     u32 reward_spell;
///     u32 casted_reward_spell;
///     u32 honor_reward;
///     u32 source_item_id;
///     u32 quest_flags;
///     u32 title_reward;
///     QuestItemReward[4] rewards;
///     QuestItemReward[6] choice_rewards;
///     u32 point_map_id;
///     Vector2d position;
///     u32 point_opt;
///     CString title;
///     CString objective_text;
///     CString details;
///     CString end_text;
///     QuestObjective[4] objectives;
///     CString[4] objective_texts;
/// }
/// ```
pub struct SMSG_QUEST_QUERY_RESPONSE {
    pub quest_id: u32,
    /// Accepted values: 0, 1 or 2. 0==IsAutoComplete() (skip objectives/details)
    ///
    pub quest_method: u32,
    pub quest_level: u32,
    pub zone_or_sort: u32,
    pub quest_type: u32,
    pub suggest_player_amount: u32,
    /// cmangos: shown in quest log as part of quest objective
    ///
    pub reputation_objective_faction: Faction,
    /// cmangos: shown in quest log as part of quest objective
    ///
    pub reputation_objective_value: u32,
    /// cmangos: RequiredOpositeRepFaction, required faction value with another (oposite) faction (objective). cmangos sets to 0
    ///
    pub required_opposite_faction: Faction,
    /// cmangos: RequiredOpositeRepValue, required faction value with another (oposite) faction (objective). cmangos sets to 0
    ///
    pub required_opposite_reputation_value: u32,
    pub next_quest_in_chain: u32,
    pub money_reward: u32,
    /// cmangos: used in XP calculation at client
    ///
    pub max_level_money_reward: u32,
    /// cmangos: reward spell, this spell will display (icon) (casted if RewSpellCast==0)
    ///
    pub reward_spell: u32,
    /// mangosone: casted spell
    ///
    pub casted_reward_spell: u32,
    pub honor_reward: u32,
    pub source_item_id: u32,
    pub quest_flags: u32,
    /// CharTitleId, new 2.4.0, player gets this title (id from CharTitles)
    ///
    pub title_reward: u32,
    pub rewards: [QuestItemReward; 4],
    pub choice_rewards: [QuestItemReward; 6],
    pub point_map_id: u32,
    pub position: Vector2d,
    pub point_opt: u32,
    pub title: String,
    pub objective_text: String,
    pub details: String,
    pub end_text: String,
    pub objectives: [QuestObjective; 4],
    pub objective_texts: [String; 4],
}

impl crate::Message for SMSG_QUEST_QUERY_RESPONSE {
    const OPCODE: u32 = 0x005d;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
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

        // suggest_player_amount: u32
        w.write_all(&self.suggest_player_amount.to_le_bytes())?;

        // reputation_objective_faction: Faction
        w.write_all(&(self.reputation_objective_faction.as_int() as u16).to_le_bytes())?;

        // reputation_objective_value: u32
        w.write_all(&self.reputation_objective_value.to_le_bytes())?;

        // required_opposite_faction: Faction
        w.write_all(&(self.required_opposite_faction.as_int() as u16).to_le_bytes())?;

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

        // casted_reward_spell: u32
        w.write_all(&self.casted_reward_spell.to_le_bytes())?;

        // honor_reward: u32
        w.write_all(&self.honor_reward.to_le_bytes())?;

        // source_item_id: u32
        w.write_all(&self.source_item_id.to_le_bytes())?;

        // quest_flags: u32
        w.write_all(&self.quest_flags.to_le_bytes())?;

        // title_reward: u32
        w.write_all(&self.title_reward.to_le_bytes())?;

        // rewards: QuestItemReward[4]
        for i in self.rewards.iter() {
            i.write_into_vec(w)?;
        }

        // choice_rewards: QuestItemReward[6]
        for i in self.choice_rewards.iter() {
            i.write_into_vec(w)?;
        }

        // point_map_id: u32
        w.write_all(&self.point_map_id.to_le_bytes())?;

        // position: Vector2d
        self.position.write_into_vec(w)?;

        // point_opt: u32
        w.write_all(&self.point_opt.to_le_bytes())?;

        // title: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.title.as_bytes().iter().rev().next(), Some(&0_u8), "String `title` must not be null-terminated.");
        w.write_all(self.title.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // objective_text: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.objective_text.as_bytes().iter().rev().next(), Some(&0_u8), "String `objective_text` must not be null-terminated.");
        w.write_all(self.objective_text.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // details: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.details.as_bytes().iter().rev().next(), Some(&0_u8), "String `details` must not be null-terminated.");
        w.write_all(self.details.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // end_text: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.end_text.as_bytes().iter().rev().next(), Some(&0_u8), "String `end_text` must not be null-terminated.");
        w.write_all(self.end_text.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // objectives: QuestObjective[4]
        for i in self.objectives.iter() {
            i.write_into_vec(w)?;
        }

        // objective_texts: CString[4]
        for i in self.objective_texts.iter() {
            w.write_all(i.as_bytes())?;
            w.write_all(&[0])?;
        }

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(240..=2280).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x005D, size: body_size as u32 });
        }

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

        // suggest_player_amount: u32
        let suggest_player_amount = crate::util::read_u32_le(r)?;

        // reputation_objective_faction: Faction
        let reputation_objective_faction: Faction = crate::util::read_u16_le(r)?.try_into()?;

        // reputation_objective_value: u32
        let reputation_objective_value = crate::util::read_u32_le(r)?;

        // required_opposite_faction: Faction
        let required_opposite_faction: Faction = crate::util::read_u16_le(r)?.try_into()?;

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

        // casted_reward_spell: u32
        let casted_reward_spell = crate::util::read_u32_le(r)?;

        // honor_reward: u32
        let honor_reward = crate::util::read_u32_le(r)?;

        // source_item_id: u32
        let source_item_id = crate::util::read_u32_le(r)?;

        // quest_flags: u32
        let quest_flags = crate::util::read_u32_le(r)?;

        // title_reward: u32
        let title_reward = crate::util::read_u32_le(r)?;

        // rewards: QuestItemReward[4]
        let mut rewards = [QuestItemReward::default(); 4];
        for i in rewards.iter_mut() {
            *i = QuestItemReward::read(r)?;
        }

        // choice_rewards: QuestItemReward[6]
        let mut choice_rewards = [QuestItemReward::default(); 6];
        for i in choice_rewards.iter_mut() {
            *i = QuestItemReward::read(r)?;
        }

        // point_map_id: u32
        let point_map_id = crate::util::read_u32_le(r)?;

        // position: Vector2d
        let position = Vector2d::read(r)?;

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
        let mut objectives = [QuestObjective::default(); 4];
        for i in objectives.iter_mut() {
            *i = QuestObjective::read(r)?;
        }

        // objective_texts: CString[4]
        let mut objective_texts = Vec::with_capacity(4);
        for i in 0..4 {
            let s = crate::util::read_c_string_to_vec(r)?;
            objective_texts.push(String::from_utf8(s)?);
        }
        let objective_texts = objective_texts.try_into().unwrap();

        Ok(Self {
            quest_id,
            quest_method,
            quest_level,
            zone_or_sort,
            quest_type,
            suggest_player_amount,
            reputation_objective_faction,
            reputation_objective_value,
            required_opposite_faction,
            required_opposite_reputation_value,
            next_quest_in_chain,
            money_reward,
            max_level_money_reward,
            reward_spell,
            casted_reward_spell,
            honor_reward,
            source_item_id,
            quest_flags,
            title_reward,
            rewards,
            choice_rewards,
            point_map_id,
            position,
            point_opt,
            title,
            objective_text,
            details,
            end_text,
            objectives,
            objective_texts,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_QUEST_QUERY_RESPONSE {}

impl SMSG_QUEST_QUERY_RESPONSE {
    pub(crate) fn size(&self) -> usize {
        4 // quest_id: u32
        + 4 // quest_method: u32
        + 4 // quest_level: u32
        + 4 // zone_or_sort: u32
        + 4 // quest_type: u32
        + 4 // suggest_player_amount: u32
        + 2 // reputation_objective_faction: Faction
        + 4 // reputation_objective_value: u32
        + 2 // required_opposite_faction: Faction
        + 4 // required_opposite_reputation_value: u32
        + 4 // next_quest_in_chain: u32
        + 4 // money_reward: u32
        + 4 // max_level_money_reward: u32
        + 4 // reward_spell: u32
        + 4 // casted_reward_spell: u32
        + 4 // honor_reward: u32
        + 4 // source_item_id: u32
        + 4 // quest_flags: u32
        + 4 // title_reward: u32
        + 4 * 8 // rewards: QuestItemReward[4]
        + 6 * 8 // choice_rewards: QuestItemReward[6]
        + 4 // point_map_id: u32
        + 8 // position: Vector2d
        + 4 // point_opt: u32
        + self.title.len() + 1 // title: CString
        + self.objective_text.len() + 1 // objective_text: CString
        + self.details.len() + 1 // details: CString
        + self.end_text.len() + 1 // end_text: CString
        + 4 * 16 // objectives: QuestObjective[4]
        + self.objective_texts.iter().fold(0, |acc, x| acc + x.len() + 1) // objective_texts: CString[4]
    }
}

