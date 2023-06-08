use std::io::{Read, Write};

use crate::wrath::{
    Faction, QuestItemRequirement, QuestItemReward, QuestObjective, Vector2d,
};
use wow_world_base::shared::gold_vanilla_tbc_wrath::Gold;
use wow_world_base::shared::level_vanilla_tbc_wrath::Level;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_quest_query_response.wowm:110`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_quest_query_response.wowm#L110):
/// ```text
/// smsg SMSG_QUEST_QUERY_RESPONSE = 0x005D {
///     u32 quest_id;
///     u32 quest_method;
///     Level32 quest_level;
///     Level32 minimum_quest_level;
///     u32 zone_or_sort;
///     u32 quest_type;
///     u32 suggest_player_amount;
///     Faction reputation_objective_faction;
///     u32 reputation_objective_value;
///     Faction required_opposite_faction;
///     u32 required_opposite_reputation_value;
///     u32 next_quest_in_chain;
///     Gold money_reward;
///     Gold max_level_money_reward;
///     u32 reward_spell;
///     u32 casted_reward_spell;
///     u32 honor_reward;
///     f32 honor_reward_multiplier;
///     u32 source_item_id;
///     u32 quest_flags;
///     u32 title_reward;
///     u32 players_slain;
///     u32 bonus_talents;
///     u32 bonus_arena_points;
///     u32 unknown1;
///     QuestItemReward[4] rewards;
///     QuestItemReward[6] choice_rewards;
///     u32[5] reputation_rewards;
///     u32[5] reputation_reward_amounts;
///     u32[5] reputation_reward_overrides;
///     u32 point_map_id;
///     Vector2d position;
///     u32 point_opt;
///     CString title;
///     CString objective_text;
///     CString details;
///     CString end_text;
///     CString completed_text;
///     QuestObjective[4] objectives;
///     QuestItemRequirement[6] item_requirements;
///     CString[4] objective_texts;
/// }
/// ```
pub struct SMSG_QUEST_QUERY_RESPONSE {
    pub quest_id: u32,
    /// Accepted values: 0, 1 or 2. 0==IsAutoComplete() (skip objectives/details)
    pub quest_method: u32,
    pub quest_level: Level,
    /// min required level to obtain (added for 3.3).
    /// Assumed allowed (database) range is -1 to 255 (still using uint32, since negative value would not be of any known use for client)
    pub minimum_quest_level: Level,
    pub zone_or_sort: u32,
    pub quest_type: u32,
    pub suggest_player_amount: u32,
    /// cmangos: shown in quest log as part of quest objective
    pub reputation_objective_faction: Faction,
    /// cmangos: shown in quest log as part of quest objective
    pub reputation_objective_value: u32,
    /// cmangos: RequiredOpositeRepFaction, required faction value with another (oposite) faction (objective). cmangos sets to 0
    pub required_opposite_faction: Faction,
    /// cmangos: RequiredOpositeRepValue, required faction value with another (oposite) faction (objective). cmangos sets to 0
    pub required_opposite_reputation_value: u32,
    pub next_quest_in_chain: u32,
    pub money_reward: Gold,
    /// cmangos: used in XP calculation at client
    pub max_level_money_reward: Gold,
    /// cmangos: reward spell, this spell will display (icon) (casted if RewSpellCast==0)
    pub reward_spell: u32,
    /// mangosone: casted spell
    pub casted_reward_spell: u32,
    pub honor_reward: u32,
    /// new reward honor (multiplied by around 62 at client side)
    pub honor_reward_multiplier: f32,
    pub source_item_id: u32,
    pub quest_flags: u32,
    /// CharTitleId, new 2.4.0, player gets this title (id from CharTitles)
    pub title_reward: u32,
    pub players_slain: u32,
    pub bonus_talents: u32,
    pub bonus_arena_points: u32,
    pub unknown1: u32,
    pub rewards: [QuestItemReward; 4],
    pub choice_rewards: [QuestItemReward; 6],
    pub reputation_rewards: [u32; 5],
    pub reputation_reward_amounts: [u32; 5],
    pub reputation_reward_overrides: [u32; 5],
    pub point_map_id: u32,
    pub position: Vector2d,
    pub point_opt: u32,
    pub title: String,
    pub objective_text: String,
    pub details: String,
    pub end_text: String,
    pub completed_text: String,
    pub objectives: [QuestObjective; 4],
    pub item_requirements: [QuestItemRequirement; 6],
    pub objective_texts: [String; 4],
}

impl crate::private::Sealed for SMSG_QUEST_QUERY_RESPONSE {}
impl crate::Message for SMSG_QUEST_QUERY_RESPONSE {
    const OPCODE: u32 = 0x005d;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_QUEST_QUERY_RESPONSE {{").unwrap();
        // Members
        writeln!(s, "    quest_id = {};", self.quest_id).unwrap();
        writeln!(s, "    quest_method = {};", self.quest_method).unwrap();
        writeln!(s, "    quest_level = {};", self.quest_level.as_int()).unwrap();
        writeln!(s, "    minimum_quest_level = {};", self.minimum_quest_level.as_int()).unwrap();
        writeln!(s, "    zone_or_sort = {};", self.zone_or_sort).unwrap();
        writeln!(s, "    quest_type = {};", self.quest_type).unwrap();
        writeln!(s, "    suggest_player_amount = {};", self.suggest_player_amount).unwrap();
        writeln!(s, "    reputation_objective_faction = {};", self.reputation_objective_faction.as_test_case_value()).unwrap();
        writeln!(s, "    reputation_objective_value = {};", self.reputation_objective_value).unwrap();
        writeln!(s, "    required_opposite_faction = {};", self.required_opposite_faction.as_test_case_value()).unwrap();
        writeln!(s, "    required_opposite_reputation_value = {};", self.required_opposite_reputation_value).unwrap();
        writeln!(s, "    next_quest_in_chain = {};", self.next_quest_in_chain).unwrap();
        writeln!(s, "    money_reward = {};", self.money_reward.as_int()).unwrap();
        writeln!(s, "    max_level_money_reward = {};", self.max_level_money_reward.as_int()).unwrap();
        writeln!(s, "    reward_spell = {};", self.reward_spell).unwrap();
        writeln!(s, "    casted_reward_spell = {};", self.casted_reward_spell).unwrap();
        writeln!(s, "    honor_reward = {};", self.honor_reward).unwrap();
        writeln!(s, "    {}", if self.honor_reward_multiplier.to_string().contains(".") { self.honor_reward_multiplier.to_string() } else { format!("{}.0", self.honor_reward_multiplier) }).unwrap();
        writeln!(s, "    source_item_id = {};", self.source_item_id).unwrap();
        writeln!(s, "    quest_flags = {};", self.quest_flags).unwrap();
        writeln!(s, "    title_reward = {};", self.title_reward).unwrap();
        writeln!(s, "    players_slain = {};", self.players_slain).unwrap();
        writeln!(s, "    bonus_talents = {};", self.bonus_talents).unwrap();
        writeln!(s, "    bonus_arena_points = {};", self.bonus_arena_points).unwrap();
        writeln!(s, "    unknown1 = {};", self.unknown1).unwrap();
        write!(s, "    rewards = [").unwrap();
        for v in self.rewards.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        item = {};", v.item).unwrap();
            writeln!(s, "        item_count = {};", v.item_count).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();
        write!(s, "    choice_rewards = [").unwrap();
        for v in self.choice_rewards.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        item = {};", v.item).unwrap();
            writeln!(s, "        item_count = {};", v.item_count).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();
        write!(s, "    reputation_rewards = [").unwrap();
        for v in self.reputation_rewards.as_slice() {
            write!(s, "{v:#04X}, ").unwrap();
        }
        writeln!(s, "];").unwrap();
        write!(s, "    reputation_reward_amounts = [").unwrap();
        for v in self.reputation_reward_amounts.as_slice() {
            write!(s, "{v:#04X}, ").unwrap();
        }
        writeln!(s, "];").unwrap();
        write!(s, "    reputation_reward_overrides = [").unwrap();
        for v in self.reputation_reward_overrides.as_slice() {
            write!(s, "{v:#04X}, ").unwrap();
        }
        writeln!(s, "];").unwrap();
        writeln!(s, "    point_map_id = {};", self.point_map_id).unwrap();
        // position: Vector2d
        writeln!(s, "    position = {{").unwrap();
        // Members
        writeln!(s, "    {}", if self.position.x.to_string().contains(".") { self.position.x.to_string() } else { format!("{}.0", self.position.x) }).unwrap();
        writeln!(s, "    {}", if self.position.y.to_string().contains(".") { self.position.y.to_string() } else { format!("{}.0", self.position.y) }).unwrap();

        writeln!(s, "    }};").unwrap();
        writeln!(s, "    point_opt = {};", self.point_opt).unwrap();
        writeln!(s, "    title = \"{}\";", self.title).unwrap();
        writeln!(s, "    objective_text = \"{}\";", self.objective_text).unwrap();
        writeln!(s, "    details = \"{}\";", self.details).unwrap();
        writeln!(s, "    end_text = \"{}\";", self.end_text).unwrap();
        writeln!(s, "    completed_text = \"{}\";", self.completed_text).unwrap();
        write!(s, "    objectives = [").unwrap();
        for v in self.objectives.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        creature_id = {};", v.creature_id).unwrap();
            writeln!(s, "        kill_count = {};", v.kill_count).unwrap();
            writeln!(s, "        required_item_id = {};", v.required_item_id).unwrap();
            writeln!(s, "        required_item_count = {};", v.required_item_count).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();
        write!(s, "    item_requirements = [").unwrap();
        for v in self.item_requirements.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        item = {};", v.item).unwrap();
            writeln!(s, "        item_count = {};", v.item_count).unwrap();
            writeln!(s, "        item_display_id = {};", v.item_display_id).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();
        write!(s, "    objective_texts = [").unwrap();
        for v in self.objective_texts.as_slice() {
            write!(s, "\"{v}\", ").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 93_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "quest_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "quest_method", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "quest_level", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "minimum_quest_level", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "zone_or_sort", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "quest_type", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "suggest_player_amount", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 2, "reputation_objective_faction", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "reputation_objective_value", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 2, "required_opposite_faction", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "required_opposite_reputation_value", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "next_quest_in_chain", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "money_reward", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "max_level_money_reward", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "reward_spell", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "casted_reward_spell", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "honor_reward", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "honor_reward_multiplier", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "source_item_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "quest_flags", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "title_reward", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "players_slain", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "bonus_talents", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "bonus_arena_points", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown1", "    ");
        writeln!(s, "    /* rewards: QuestItemReward[4] start */").unwrap();
        for (i, v) in self.rewards.iter().enumerate() {
            writeln!(s, "    /* rewards: QuestItemReward[4] {i} start */").unwrap();
            crate::util::write_bytes(&mut s, &mut bytes, 4, "item", "        ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "item_count", "        ");
            writeln!(s, "    /* rewards: QuestItemReward[4] {i} end */").unwrap();
        }
        writeln!(s, "    /* rewards: QuestItemReward[4] end */").unwrap();
        writeln!(s, "    /* choice_rewards: QuestItemReward[6] start */").unwrap();
        for (i, v) in self.choice_rewards.iter().enumerate() {
            writeln!(s, "    /* choice_rewards: QuestItemReward[6] {i} start */").unwrap();
            crate::util::write_bytes(&mut s, &mut bytes, 4, "item", "        ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "item_count", "        ");
            writeln!(s, "    /* choice_rewards: QuestItemReward[6] {i} end */").unwrap();
        }
        writeln!(s, "    /* choice_rewards: QuestItemReward[6] end */").unwrap();
        writeln!(s, "    /* reputation_rewards: u32[5] start */").unwrap();
        for (i, v) in self.reputation_rewards.iter().enumerate() {
            crate::util::write_bytes(&mut s, &mut bytes, 4, &format!("reputation_rewards {i}"), "    ");
        }
        writeln!(s, "    /* reputation_rewards: u32[5] end */").unwrap();
        writeln!(s, "    /* reputation_reward_amounts: u32[5] start */").unwrap();
        for (i, v) in self.reputation_reward_amounts.iter().enumerate() {
            crate::util::write_bytes(&mut s, &mut bytes, 4, &format!("reputation_reward_amounts {i}"), "    ");
        }
        writeln!(s, "    /* reputation_reward_amounts: u32[5] end */").unwrap();
        writeln!(s, "    /* reputation_reward_overrides: u32[5] start */").unwrap();
        for (i, v) in self.reputation_reward_overrides.iter().enumerate() {
            crate::util::write_bytes(&mut s, &mut bytes, 4, &format!("reputation_reward_overrides {i}"), "    ");
        }
        writeln!(s, "    /* reputation_reward_overrides: u32[5] end */").unwrap();
        crate::util::write_bytes(&mut s, &mut bytes, 4, "point_map_id", "    ");
        writeln!(s, "    /* position: Vector2d start */").unwrap();
        crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "        ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "        ");
        writeln!(s, "    /* position: Vector2d end */").unwrap();
        crate::util::write_bytes(&mut s, &mut bytes, 4, "point_opt", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.title.len() + 1, "title", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.objective_text.len() + 1, "objective_text", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.details.len() + 1, "details", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.end_text.len() + 1, "end_text", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.completed_text.len() + 1, "completed_text", "    ");
        writeln!(s, "    /* objectives: QuestObjective[4] start */").unwrap();
        for (i, v) in self.objectives.iter().enumerate() {
            writeln!(s, "    /* objectives: QuestObjective[4] {i} start */").unwrap();
            crate::util::write_bytes(&mut s, &mut bytes, 4, "creature_id", "        ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "kill_count", "        ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "required_item_id", "        ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "required_item_count", "        ");
            writeln!(s, "    /* objectives: QuestObjective[4] {i} end */").unwrap();
        }
        writeln!(s, "    /* objectives: QuestObjective[4] end */").unwrap();
        writeln!(s, "    /* item_requirements: QuestItemRequirement[6] start */").unwrap();
        for (i, v) in self.item_requirements.iter().enumerate() {
            writeln!(s, "    /* item_requirements: QuestItemRequirement[6] {i} start */").unwrap();
            crate::util::write_bytes(&mut s, &mut bytes, 4, "item", "        ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "item_count", "        ");
            crate::util::write_bytes(&mut s, &mut bytes, 4, "item_display_id", "        ");
            writeln!(s, "    /* item_requirements: QuestItemRequirement[6] {i} end */").unwrap();
        }
        writeln!(s, "    /* item_requirements: QuestItemRequirement[6] end */").unwrap();
        writeln!(s, "    /* objective_texts: CString[4] start */").unwrap();
        for (i, v) in self.objective_texts.iter().enumerate() {
            crate::util::write_bytes(&mut s, &mut bytes, v.len() + 1, &format!("objective_texts {i}"), "    ");
        }
        writeln!(s, "    /* objective_texts: CString[4] end */").unwrap();


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        // quest_method: u32
        w.write_all(&self.quest_method.to_le_bytes())?;

        // quest_level: Level32
        w.write_all(&u32::from(self.quest_level.as_int()).to_le_bytes())?;

        // minimum_quest_level: Level32
        w.write_all(&u32::from(self.minimum_quest_level.as_int()).to_le_bytes())?;

        // zone_or_sort: u32
        w.write_all(&self.zone_or_sort.to_le_bytes())?;

        // quest_type: u32
        w.write_all(&self.quest_type.to_le_bytes())?;

        // suggest_player_amount: u32
        w.write_all(&self.suggest_player_amount.to_le_bytes())?;

        // reputation_objective_faction: Faction
        w.write_all(&(self.reputation_objective_faction.as_int().to_le_bytes()))?;

        // reputation_objective_value: u32
        w.write_all(&self.reputation_objective_value.to_le_bytes())?;

        // required_opposite_faction: Faction
        w.write_all(&(self.required_opposite_faction.as_int().to_le_bytes()))?;

        // required_opposite_reputation_value: u32
        w.write_all(&self.required_opposite_reputation_value.to_le_bytes())?;

        // next_quest_in_chain: u32
        w.write_all(&self.next_quest_in_chain.to_le_bytes())?;

        // money_reward: Gold
        w.write_all((self.money_reward.as_int()).to_le_bytes().as_slice())?;

        // max_level_money_reward: Gold
        w.write_all((self.max_level_money_reward.as_int()).to_le_bytes().as_slice())?;

        // reward_spell: u32
        w.write_all(&self.reward_spell.to_le_bytes())?;

        // casted_reward_spell: u32
        w.write_all(&self.casted_reward_spell.to_le_bytes())?;

        // honor_reward: u32
        w.write_all(&self.honor_reward.to_le_bytes())?;

        // honor_reward_multiplier: f32
        w.write_all(&self.honor_reward_multiplier.to_le_bytes())?;

        // source_item_id: u32
        w.write_all(&self.source_item_id.to_le_bytes())?;

        // quest_flags: u32
        w.write_all(&self.quest_flags.to_le_bytes())?;

        // title_reward: u32
        w.write_all(&self.title_reward.to_le_bytes())?;

        // players_slain: u32
        w.write_all(&self.players_slain.to_le_bytes())?;

        // bonus_talents: u32
        w.write_all(&self.bonus_talents.to_le_bytes())?;

        // bonus_arena_points: u32
        w.write_all(&self.bonus_arena_points.to_le_bytes())?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // rewards: QuestItemReward[4]
        for i in self.rewards.iter() {
            i.write_into_vec(&mut w)?;
        }

        // choice_rewards: QuestItemReward[6]
        for i in self.choice_rewards.iter() {
            i.write_into_vec(&mut w)?;
        }

        // reputation_rewards: u32[5]
        for i in self.reputation_rewards.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        // reputation_reward_amounts: u32[5]
        for i in self.reputation_reward_amounts.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        // reputation_reward_overrides: u32[5]
        for i in self.reputation_reward_overrides.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        // point_map_id: u32
        w.write_all(&self.point_map_id.to_le_bytes())?;

        // position: Vector2d
        self.position.write_into_vec(&mut w)?;

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

        // completed_text: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.completed_text.as_bytes().iter().rev().next(), Some(&0_u8), "String `completed_text` must not be null-terminated.");
        w.write_all(self.completed_text.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // objectives: QuestObjective[4]
        for i in self.objectives.iter() {
            i.write_into_vec(&mut w)?;
        }

        // item_requirements: QuestItemRequirement[6]
        for i in self.item_requirements.iter() {
            i.write_into_vec(&mut w)?;
        }

        // objective_texts: CString[4]
        for i in self.objective_texts.iter() {
            w.write_all(i.as_bytes())?;
            w.write_all(&[0])?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(397..=2692).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x005D, size: body_size });
        }

        // quest_id: u32
        let quest_id = crate::util::read_u32_le(&mut r)?;

        // quest_method: u32
        let quest_method = crate::util::read_u32_le(&mut r)?;

        // quest_level: Level32
        let quest_level = Level::new(crate::util::read_u32_le(&mut r)? as u8);

        // minimum_quest_level: Level32
        let minimum_quest_level = Level::new(crate::util::read_u32_le(&mut r)? as u8);

        // zone_or_sort: u32
        let zone_or_sort = crate::util::read_u32_le(&mut r)?;

        // quest_type: u32
        let quest_type = crate::util::read_u32_le(&mut r)?;

        // suggest_player_amount: u32
        let suggest_player_amount = crate::util::read_u32_le(&mut r)?;

        // reputation_objective_faction: Faction
        let reputation_objective_faction = crate::util::read_u16_le(&mut r)?.try_into()?;

        // reputation_objective_value: u32
        let reputation_objective_value = crate::util::read_u32_le(&mut r)?;

        // required_opposite_faction: Faction
        let required_opposite_faction = crate::util::read_u16_le(&mut r)?.try_into()?;

        // required_opposite_reputation_value: u32
        let required_opposite_reputation_value = crate::util::read_u32_le(&mut r)?;

        // next_quest_in_chain: u32
        let next_quest_in_chain = crate::util::read_u32_le(&mut r)?;

        // money_reward: Gold
        let money_reward = Gold::new(crate::util::read_u32_le(&mut r)?);

        // max_level_money_reward: Gold
        let max_level_money_reward = Gold::new(crate::util::read_u32_le(&mut r)?);

        // reward_spell: u32
        let reward_spell = crate::util::read_u32_le(&mut r)?;

        // casted_reward_spell: u32
        let casted_reward_spell = crate::util::read_u32_le(&mut r)?;

        // honor_reward: u32
        let honor_reward = crate::util::read_u32_le(&mut r)?;

        // honor_reward_multiplier: f32
        let honor_reward_multiplier = crate::util::read_f32_le(&mut r)?;

        // source_item_id: u32
        let source_item_id = crate::util::read_u32_le(&mut r)?;

        // quest_flags: u32
        let quest_flags = crate::util::read_u32_le(&mut r)?;

        // title_reward: u32
        let title_reward = crate::util::read_u32_le(&mut r)?;

        // players_slain: u32
        let players_slain = crate::util::read_u32_le(&mut r)?;

        // bonus_talents: u32
        let bonus_talents = crate::util::read_u32_le(&mut r)?;

        // bonus_arena_points: u32
        let bonus_arena_points = crate::util::read_u32_le(&mut r)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(&mut r)?;

        // rewards: QuestItemReward[4]
        let rewards = {
            let mut rewards = [QuestItemReward::default(); 4];
            for i in rewards.iter_mut() {
                *i = QuestItemReward::read(&mut r)?;
            }
            rewards
        };

        // choice_rewards: QuestItemReward[6]
        let choice_rewards = {
            let mut choice_rewards = [QuestItemReward::default(); 6];
            for i in choice_rewards.iter_mut() {
                *i = QuestItemReward::read(&mut r)?;
            }
            choice_rewards
        };

        // reputation_rewards: u32[5]
        let reputation_rewards = {
            let mut reputation_rewards = [u32::default(); 5];
            for i in reputation_rewards.iter_mut() {
                *i = crate::util::read_u32_le(&mut r)?;
            }
            reputation_rewards
        };

        // reputation_reward_amounts: u32[5]
        let reputation_reward_amounts = {
            let mut reputation_reward_amounts = [u32::default(); 5];
            for i in reputation_reward_amounts.iter_mut() {
                *i = crate::util::read_u32_le(&mut r)?;
            }
            reputation_reward_amounts
        };

        // reputation_reward_overrides: u32[5]
        let reputation_reward_overrides = {
            let mut reputation_reward_overrides = [u32::default(); 5];
            for i in reputation_reward_overrides.iter_mut() {
                *i = crate::util::read_u32_le(&mut r)?;
            }
            reputation_reward_overrides
        };

        // point_map_id: u32
        let point_map_id = crate::util::read_u32_le(&mut r)?;

        // position: Vector2d
        let position = Vector2d::read(&mut r)?;

        // point_opt: u32
        let point_opt = crate::util::read_u32_le(&mut r)?;

        // title: CString
        let title = {
            let title = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(title)?
        };

        // objective_text: CString
        let objective_text = {
            let objective_text = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(objective_text)?
        };

        // details: CString
        let details = {
            let details = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(details)?
        };

        // end_text: CString
        let end_text = {
            let end_text = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(end_text)?
        };

        // completed_text: CString
        let completed_text = {
            let completed_text = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(completed_text)?
        };

        // objectives: QuestObjective[4]
        let objectives = {
            let mut objectives = [QuestObjective::default(); 4];
            for i in objectives.iter_mut() {
                *i = QuestObjective::read(&mut r)?;
            }
            objectives
        };

        // item_requirements: QuestItemRequirement[6]
        let item_requirements = {
            let mut item_requirements = [QuestItemRequirement::default(); 6];
            for i in item_requirements.iter_mut() {
                *i = QuestItemRequirement::read(&mut r)?;
            }
            item_requirements
        };

        // objective_texts: CString[4]
        let objective_texts = {
            let mut objective_texts = [(); 4].map(|_| String::default());
            for i in objective_texts.iter_mut() {
                let s = crate::util::read_c_string_to_vec(&mut r)?;
                *i = String::from_utf8(s)?;
            }
            objective_texts
        };

        Ok(Self {
            quest_id,
            quest_method,
            quest_level,
            minimum_quest_level,
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
            honor_reward_multiplier,
            source_item_id,
            quest_flags,
            title_reward,
            players_slain,
            bonus_talents,
            bonus_arena_points,
            unknown1,
            rewards,
            choice_rewards,
            reputation_rewards,
            reputation_reward_amounts,
            reputation_reward_overrides,
            point_map_id,
            position,
            point_opt,
            title,
            objective_text,
            details,
            end_text,
            completed_text,
            objectives,
            item_requirements,
            objective_texts,
        })
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_QUEST_QUERY_RESPONSE {}

impl SMSG_QUEST_QUERY_RESPONSE {
    pub(crate) fn size(&self) -> usize {
        4 // quest_id: u32
        + 4 // quest_method: u32
        + 4 // quest_level: Level32
        + 4 // minimum_quest_level: Level32
        + 4 // zone_or_sort: u32
        + 4 // quest_type: u32
        + 4 // suggest_player_amount: u32
        + 2 // reputation_objective_faction: Faction
        + 4 // reputation_objective_value: u32
        + 2 // required_opposite_faction: Faction
        + 4 // required_opposite_reputation_value: u32
        + 4 // next_quest_in_chain: u32
        + 4 // money_reward: Gold
        + 4 // max_level_money_reward: Gold
        + 4 // reward_spell: u32
        + 4 // casted_reward_spell: u32
        + 4 // honor_reward: u32
        + 4 // honor_reward_multiplier: f32
        + 4 // source_item_id: u32
        + 4 // quest_flags: u32
        + 4 // title_reward: u32
        + 4 // players_slain: u32
        + 4 // bonus_talents: u32
        + 4 // bonus_arena_points: u32
        + 4 // unknown1: u32
        + 4 * 8 // rewards: QuestItemReward[4]
        + 6 * 8 // choice_rewards: QuestItemReward[6]
        + 20 // reputation_rewards: u32[5]
        + 20 // reputation_reward_amounts: u32[5]
        + 20 // reputation_reward_overrides: u32[5]
        + 4 // point_map_id: u32
        + 8 // position: Vector2d
        + 4 // point_opt: u32
        + self.title.len() + 1 // title: CString
        + self.objective_text.len() + 1 // objective_text: CString
        + self.details.len() + 1 // details: CString
        + self.end_text.len() + 1 // end_text: CString
        + self.completed_text.len() + 1 // completed_text: CString
        + 4 * 16 // objectives: QuestObjective[4]
        + 6 * 12 // item_requirements: QuestItemRequirement[6]
        + self.objective_texts.iter().fold(0, |acc, x| acc + x.len() + 1) // objective_texts: CString[4]
    }
}

