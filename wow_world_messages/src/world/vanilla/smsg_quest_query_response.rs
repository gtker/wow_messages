use std::io::{Read, Write};

use crate::vanilla::{
    Faction, QuestItemReward, QuestObjective, Vector2d,
};
use wow_world_base::shared::gold_vanilla_tbc_wrath::Gold;
use wow_world_base::shared::level_vanilla_tbc_wrath::Level;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_quest_query_response.wowm:12`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_quest_query_response.wowm#L12):
/// ```text
/// smsg SMSG_QUEST_QUERY_RESPONSE = 0x005D {
///     u32 quest_id;
///     u32 quest_method;
///     Level32 quest_level;
///     u32 zone_or_sort;
///     u32 quest_type;
///     Faction reputation_objective_faction;
///     u32 reputation_objective_value;
///     Faction required_opposite_faction;
///     u32 required_opposite_reputation_value;
///     u32 next_quest_in_chain;
///     Gold money_reward;
///     Gold max_level_money_reward;
///     u32 reward_spell;
///     u32 source_item_id;
///     u32 quest_flags;
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
    pub quest_method: u32,
    pub quest_level: Level,
    pub zone_or_sort: u32,
    pub quest_type: u32,
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
    pub source_item_id: u32,
    pub quest_flags: u32,
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

impl crate::private::Sealed for SMSG_QUEST_QUERY_RESPONSE {}
impl SMSG_QUEST_QUERY_RESPONSE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(224..=2264).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // quest_id: u32
        let quest_id = crate::util::read_u32_le(&mut r)?;

        // quest_method: u32
        let quest_method = crate::util::read_u32_le(&mut r)?;

        // quest_level: Level32
        let quest_level = Level::new(crate::util::read_u32_le(&mut r)? as u8);

        // zone_or_sort: u32
        let zone_or_sort = crate::util::read_u32_le(&mut r)?;

        // quest_type: u32
        let quest_type = crate::util::read_u32_le(&mut r)?;

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

        // source_item_id: u32
        let source_item_id = crate::util::read_u32_le(&mut r)?;

        // quest_flags: u32
        let quest_flags = crate::util::read_u32_le(&mut r)?;

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

        // objectives: QuestObjective[4]
        let objectives = {
            let mut objectives = [QuestObjective::default(); 4];
            for i in objectives.iter_mut() {
                *i = QuestObjective::read(&mut r)?;
            }
            objectives
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
        writeln!(s, "    zone_or_sort = {};", self.zone_or_sort).unwrap();
        writeln!(s, "    quest_type = {};", self.quest_type).unwrap();
        writeln!(s, "    reputation_objective_faction = {};", self.reputation_objective_faction.as_test_case_value()).unwrap();
        writeln!(s, "    reputation_objective_value = {};", self.reputation_objective_value).unwrap();
        writeln!(s, "    required_opposite_faction = {};", self.required_opposite_faction.as_test_case_value()).unwrap();
        writeln!(s, "    required_opposite_reputation_value = {};", self.required_opposite_reputation_value).unwrap();
        writeln!(s, "    next_quest_in_chain = {};", self.next_quest_in_chain).unwrap();
        writeln!(s, "    money_reward = {};", self.money_reward.as_int()).unwrap();
        writeln!(s, "    max_level_money_reward = {};", self.max_level_money_reward.as_int()).unwrap();
        writeln!(s, "    reward_spell = {};", self.reward_spell).unwrap();
        writeln!(s, "    source_item_id = {};", self.source_item_id).unwrap();
        writeln!(s, "    quest_flags = {};", self.quest_flags).unwrap();
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
        writeln!(s, "    point_map_id = {};", self.point_map_id).unwrap();
        // position: Vector2d
        writeln!(s, "    position = {{").unwrap();
        // Members
        writeln!(s, "    {}", if self.position.x.to_string().contains('.') { self.position.x.to_string() } else { format!("{}.0", self.position.x) }).unwrap();
        writeln!(s, "    {}", if self.position.y.to_string().contains('.') { self.position.y.to_string() } else { format!("{}.0", self.position.y) }).unwrap();

        writeln!(s, "    }};").unwrap();
        writeln!(s, "    point_opt = {};", self.point_opt).unwrap();
        writeln!(s, "    title = \"{}\";", self.title).unwrap();
        writeln!(s, "    objective_text = \"{}\";", self.objective_text).unwrap();
        writeln!(s, "    details = \"{}\";", self.details).unwrap();
        writeln!(s, "    end_text = \"{}\";", self.end_text).unwrap();
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
        crate::util::write_bytes(&mut s, &mut bytes, 4, "zone_or_sort", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "quest_type", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 2, "reputation_objective_faction", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "reputation_objective_value", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 2, "required_opposite_faction", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "required_opposite_reputation_value", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "next_quest_in_chain", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "money_reward", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "max_level_money_reward", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "reward_spell", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "source_item_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "quest_flags", "    ");
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
        writeln!(s, "    /* objective_texts: CString[4] start */").unwrap();
        for (i, v) in self.objective_texts.iter().enumerate() {
            crate::util::write_bytes(&mut s, &mut bytes, v.len() + 1, &format!("objective_texts {i}"), "    ");
        }
        writeln!(s, "    /* objective_texts: CString[4] end */").unwrap();


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1.12".to_string())).unwrap();
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

        // zone_or_sort: u32
        w.write_all(&self.zone_or_sort.to_le_bytes())?;

        // quest_type: u32
        w.write_all(&self.quest_type.to_le_bytes())?;

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

        // source_item_id: u32
        w.write_all(&self.source_item_id.to_le_bytes())?;

        // quest_flags: u32
        w.write_all(&self.quest_flags.to_le_bytes())?;

        // rewards: QuestItemReward[4]
        for i in self.rewards.iter() {
            i.write_into_vec(&mut w)?;
        }

        // choice_rewards: QuestItemReward[6]
        for i in self.choice_rewards.iter() {
            i.write_into_vec(&mut w)?;
        }

        // point_map_id: u32
        w.write_all(&self.point_map_id.to_le_bytes())?;

        // position: Vector2d
        self.position.write_into_vec(&mut w)?;

        // point_opt: u32
        w.write_all(&self.point_opt.to_le_bytes())?;

        // title: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.title.as_bytes().iter().next_back(), Some(&0_u8), "String `title` must not be null-terminated.");
        w.write_all(self.title.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // objective_text: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.objective_text.as_bytes().iter().next_back(), Some(&0_u8), "String `objective_text` must not be null-terminated.");
        w.write_all(self.objective_text.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // details: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.details.as_bytes().iter().next_back(), Some(&0_u8), "String `details` must not be null-terminated.");
        w.write_all(self.details.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // end_text: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.end_text.as_bytes().iter().next_back(), Some(&0_u8), "String `end_text` must not be null-terminated.");
        w.write_all(self.end_text.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // objectives: QuestObjective[4]
        for i in self.objectives.iter() {
            i.write_into_vec(&mut w)?;
        }

        // objective_texts: CString[4]
        for i in self.objective_texts.iter() {
            w.write_all(i.as_bytes())?;
            w.write_all(&[0])?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(93, "SMSG_QUEST_QUERY_RESPONSE", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_QUEST_QUERY_RESPONSE {}

impl SMSG_QUEST_QUERY_RESPONSE {
    pub(crate) fn size(&self) -> usize {
        4 // quest_id: u32
        + 4 // quest_method: u32
        + 4 // quest_level: Level32
        + 4 // zone_or_sort: u32
        + 4 // quest_type: u32
        + 2 // reputation_objective_faction: Faction
        + 4 // reputation_objective_value: u32
        + 2 // required_opposite_faction: Faction
        + 4 // required_opposite_reputation_value: u32
        + 4 // next_quest_in_chain: u32
        + 4 // money_reward: Gold
        + 4 // max_level_money_reward: Gold
        + 4 // reward_spell: u32
        + 4 // source_item_id: u32
        + 4 // quest_flags: u32
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

