use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::{
    QuestDetailsEmote, QuestGiverReward,
};
use wow_world_base::shared::gold_vanilla_tbc_wrath::Gold;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_questgiver_quest_details.wowm:62`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_questgiver_quest_details.wowm#L62):
/// ```text
/// smsg SMSG_QUESTGIVER_QUEST_DETAILS = 0x0188 {
///     Guid guid;
///     Guid guid2;
///     u32 quest_id;
///     CString title;
///     CString details;
///     CString objectives;
///     Bool auto_finish;
///     u32 quest_flags;
///     u32 suggested_players;
///     u8 is_finished;
///     u32 amount_of_choice_item_rewards;
///     QuestGiverReward[amount_of_choice_item_rewards] choice_item_rewards;
///     u32 amount_of_item_rewards;
///     QuestGiverReward[amount_of_item_rewards] item_rewards;
///     Gold money_reward;
///     u32 experience_reward;
///     u32 honor_reward;
///     f32 honor_reward_multiplier;
///     u32 reward_spell;
///     u32 casted_spell;
///     u32 title_reward;
///     u32 talent_reward;
///     u32 arena_point_reward;
///     u32 unknown2;
///     u32[5] reward_factions;
///     u32[5] reward_reputations;
///     u32[5] reward_reputations_override;
///     u32 amount_of_emotes;
///     QuestDetailsEmote[amount_of_emotes] emotes;
/// }
/// ```
pub struct SMSG_QUESTGIVER_QUEST_DETAILS {
    pub guid: Guid,
    /// arcemu also sends guid2 if guid is a player. Otherwise sends 0.
    pub guid2: Guid,
    pub quest_id: u32,
    pub title: String,
    pub details: String,
    pub objectives: String,
    pub auto_finish: bool,
    pub quest_flags: u32,
    pub suggested_players: u32,
    /// arcemu: MANGOS: IsFinished? value is sent back to server in quest accept packet
    pub is_finished: u8,
    pub choice_item_rewards: Vec<QuestGiverReward>,
    pub item_rewards: Vec<QuestGiverReward>,
    pub money_reward: Gold,
    /// arcemu: New 3.3 - this is the XP you'll see on the quest reward panel too, but I think it is fine not to show it, because it can change if the player levels up before completing the quest.
    pub experience_reward: u32,
    pub honor_reward: u32,
    /// arcemu: new 3.3
    pub honor_reward_multiplier: f32,
    /// mangosone: reward spell, this spell will display (icon) (casted if RewSpellCast==0)
    pub reward_spell: u32,
    pub casted_spell: u32,
    /// mangosone: CharTitle, new 2.4.0, player gets this title (bit index from CharTitles)
    pub title_reward: u32,
    pub talent_reward: u32,
    pub arena_point_reward: u32,
    /// arcemu: new 3.3.0
    pub unknown2: u32,
    pub reward_factions: [u32; 5],
    /// mangostwo: columnid in QuestFactionReward.dbc (if negative, from second row)
    pub reward_reputations: [u32; 5],
    /// mangostwo: reward reputation override. No diplomacy bonus is expected given, reward also does not display in chat window
    pub reward_reputations_override: [u32; 5],
    pub emotes: Vec<QuestDetailsEmote>,
}

#[cfg(feature = "print-testcase")]
impl SMSG_QUESTGIVER_QUEST_DETAILS {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_QUESTGIVER_QUEST_DETAILS {{").unwrap();
        // Members
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();
        writeln!(s, "    guid2 = {};", self.guid2.guid()).unwrap();
        writeln!(s, "    quest_id = {};", self.quest_id).unwrap();
        writeln!(s, "    title = \"{}\";", self.title).unwrap();
        writeln!(s, "    details = \"{}\";", self.details).unwrap();
        writeln!(s, "    objectives = \"{}\";", self.objectives).unwrap();
        writeln!(s, "    auto_finish = {};", if self.auto_finish { "TRUE" } else { "FALSE" }).unwrap();
        writeln!(s, "    quest_flags = {};", self.quest_flags).unwrap();
        writeln!(s, "    suggested_players = {};", self.suggested_players).unwrap();
        writeln!(s, "    is_finished = {};", self.is_finished).unwrap();
        writeln!(s, "    amount_of_choice_item_rewards = {};", self.choice_item_rewards.len()).unwrap();
        write!(s, "    choice_item_rewards = [").unwrap();
        for v in self.choice_item_rewards.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        item = {};", v.item).unwrap();
            writeln!(s, "        item_count = {};", v.item_count).unwrap();
            writeln!(s, "        display_id = {};", v.display_id).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();
        writeln!(s, "    amount_of_item_rewards = {};", self.item_rewards.len()).unwrap();
        write!(s, "    item_rewards = [").unwrap();
        for v in self.item_rewards.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        item = {};", v.item).unwrap();
            writeln!(s, "        item_count = {};", v.item_count).unwrap();
            writeln!(s, "        display_id = {};", v.display_id).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();
        writeln!(s, "    money_reward = {};", self.money_reward.as_int()).unwrap();
        writeln!(s, "    experience_reward = {};", self.experience_reward).unwrap();
        writeln!(s, "    honor_reward = {};", self.honor_reward).unwrap();
        writeln!(s, "    {}", if self.honor_reward_multiplier.to_string().contains(".") { self.honor_reward_multiplier.to_string() } else { format!("{}.0", self.honor_reward_multiplier) }).unwrap();
        writeln!(s, "    reward_spell = {};", self.reward_spell).unwrap();
        writeln!(s, "    casted_spell = {};", self.casted_spell).unwrap();
        writeln!(s, "    title_reward = {};", self.title_reward).unwrap();
        writeln!(s, "    talent_reward = {};", self.talent_reward).unwrap();
        writeln!(s, "    arena_point_reward = {};", self.arena_point_reward).unwrap();
        writeln!(s, "    unknown2 = {};", self.unknown2).unwrap();
        write!(s, "    reward_factions = [").unwrap();
        for v in self.reward_factions.as_slice() {
            write!(s, "{v:#04X}, ").unwrap();
        }
        writeln!(s, "];").unwrap();
        write!(s, "    reward_reputations = [").unwrap();
        for v in self.reward_reputations.as_slice() {
            write!(s, "{v:#04X}, ").unwrap();
        }
        writeln!(s, "];").unwrap();
        write!(s, "    reward_reputations_override = [").unwrap();
        for v in self.reward_reputations_override.as_slice() {
            write!(s, "{v:#04X}, ").unwrap();
        }
        writeln!(s, "];").unwrap();
        writeln!(s, "    amount_of_emotes = {};", self.emotes.len()).unwrap();
        write!(s, "    emotes = [").unwrap();
        for v in self.emotes.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        emote = {};", v.emote).unwrap();
            writeln!(s, "        emote_delay = {};", v.emote_delay.as_millis()).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 392_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid2", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "quest_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.title.len() + 1, "title", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.details.len() + 1, "details", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.objectives.len() + 1, "objectives", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "auto_finish", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "quest_flags", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "suggested_players", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "is_finished", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_choice_item_rewards", "    ");
        if !self.choice_item_rewards.is_empty() {
            writeln!(s, "    /* choice_item_rewards: QuestGiverReward[amount_of_choice_item_rewards] start */").unwrap();
            for (i, v) in self.choice_item_rewards.iter().enumerate() {
                writeln!(s, "    /* choice_item_rewards: QuestGiverReward[amount_of_choice_item_rewards] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 4, "item", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "item_count", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "display_id", "        ");
                writeln!(s, "    /* choice_item_rewards: QuestGiverReward[amount_of_choice_item_rewards] {i} end */").unwrap();
            }
            writeln!(s, "    /* choice_item_rewards: QuestGiverReward[amount_of_choice_item_rewards] end */").unwrap();
        }
        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_item_rewards", "    ");
        if !self.item_rewards.is_empty() {
            writeln!(s, "    /* item_rewards: QuestGiverReward[amount_of_item_rewards] start */").unwrap();
            for (i, v) in self.item_rewards.iter().enumerate() {
                writeln!(s, "    /* item_rewards: QuestGiverReward[amount_of_item_rewards] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 4, "item", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "item_count", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "display_id", "        ");
                writeln!(s, "    /* item_rewards: QuestGiverReward[amount_of_item_rewards] {i} end */").unwrap();
            }
            writeln!(s, "    /* item_rewards: QuestGiverReward[amount_of_item_rewards] end */").unwrap();
        }
        crate::util::write_bytes(&mut s, &mut bytes, 4, "money_reward", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "experience_reward", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "honor_reward", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "honor_reward_multiplier", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "reward_spell", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "casted_spell", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "title_reward", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "talent_reward", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "arena_point_reward", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown2", "    ");
        writeln!(s, "    /* reward_factions: u32[5] start */").unwrap();
        for (i, v) in self.reward_factions.iter().enumerate() {
            crate::util::write_bytes(&mut s, &mut bytes, 4, &format!("reward_factions {i}"), "    ");
        }
        writeln!(s, "    /* reward_factions: u32[5] end */").unwrap();
        writeln!(s, "    /* reward_reputations: u32[5] start */").unwrap();
        for (i, v) in self.reward_reputations.iter().enumerate() {
            crate::util::write_bytes(&mut s, &mut bytes, 4, &format!("reward_reputations {i}"), "    ");
        }
        writeln!(s, "    /* reward_reputations: u32[5] end */").unwrap();
        writeln!(s, "    /* reward_reputations_override: u32[5] start */").unwrap();
        for (i, v) in self.reward_reputations_override.iter().enumerate() {
            crate::util::write_bytes(&mut s, &mut bytes, 4, &format!("reward_reputations_override {i}"), "    ");
        }
        writeln!(s, "    /* reward_reputations_override: u32[5] end */").unwrap();
        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_emotes", "    ");
        if !self.emotes.is_empty() {
            writeln!(s, "    /* emotes: QuestDetailsEmote[amount_of_emotes] start */").unwrap();
            for (i, v) in self.emotes.iter().enumerate() {
                writeln!(s, "    /* emotes: QuestDetailsEmote[amount_of_emotes] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 4, "emote", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "emote_delay", "        ");
                writeln!(s, "    /* emotes: QuestDetailsEmote[amount_of_emotes] {i} end */").unwrap();
            }
            writeln!(s, "    /* emotes: QuestDetailsEmote[amount_of_emotes] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for SMSG_QUESTGIVER_QUEST_DETAILS {}
impl crate::Message for SMSG_QUESTGIVER_QUEST_DETAILS {
    const OPCODE: u32 = 0x0188;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        SMSG_QUESTGIVER_QUEST_DETAILS::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // guid2: Guid
        w.write_all(&self.guid2.guid().to_le_bytes())?;

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

        // auto_finish: Bool
        w.write_all(u8::from(self.auto_finish).to_le_bytes().as_slice())?;

        // quest_flags: u32
        w.write_all(&self.quest_flags.to_le_bytes())?;

        // suggested_players: u32
        w.write_all(&self.suggested_players.to_le_bytes())?;

        // is_finished: u8
        w.write_all(&self.is_finished.to_le_bytes())?;

        // amount_of_choice_item_rewards: u32
        w.write_all(&(self.choice_item_rewards.len() as u32).to_le_bytes())?;

        // choice_item_rewards: QuestGiverReward[amount_of_choice_item_rewards]
        for i in self.choice_item_rewards.iter() {
            i.write_into_vec(&mut w)?;
        }

        // amount_of_item_rewards: u32
        w.write_all(&(self.item_rewards.len() as u32).to_le_bytes())?;

        // item_rewards: QuestGiverReward[amount_of_item_rewards]
        for i in self.item_rewards.iter() {
            i.write_into_vec(&mut w)?;
        }

        // money_reward: Gold
        w.write_all((self.money_reward.as_int()).to_le_bytes().as_slice())?;

        // experience_reward: u32
        w.write_all(&self.experience_reward.to_le_bytes())?;

        // honor_reward: u32
        w.write_all(&self.honor_reward.to_le_bytes())?;

        // honor_reward_multiplier: f32
        w.write_all(&self.honor_reward_multiplier.to_le_bytes())?;

        // reward_spell: u32
        w.write_all(&self.reward_spell.to_le_bytes())?;

        // casted_spell: u32
        w.write_all(&self.casted_spell.to_le_bytes())?;

        // title_reward: u32
        w.write_all(&self.title_reward.to_le_bytes())?;

        // talent_reward: u32
        w.write_all(&self.talent_reward.to_le_bytes())?;

        // arena_point_reward: u32
        w.write_all(&self.arena_point_reward.to_le_bytes())?;

        // unknown2: u32
        w.write_all(&self.unknown2.to_le_bytes())?;

        // reward_factions: u32[5]
        for i in self.reward_factions.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        // reward_reputations: u32[5]
        for i in self.reward_reputations.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        // reward_reputations_override: u32[5]
        for i in self.reward_reputations_override.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        // amount_of_emotes: u32
        w.write_all(&(self.emotes.len() as u32).to_le_bytes())?;

        // emotes: QuestDetailsEmote[amount_of_emotes]
        for i in self.emotes.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(145..=16777215).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0188, size: body_size });
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        // guid2: Guid
        let guid2 = crate::util::read_guid(&mut r)?;

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

        // auto_finish: Bool
        let auto_finish = crate::util::read_u8_le(&mut r)? != 0;

        // quest_flags: u32
        let quest_flags = crate::util::read_u32_le(&mut r)?;

        // suggested_players: u32
        let suggested_players = crate::util::read_u32_le(&mut r)?;

        // is_finished: u8
        let is_finished = crate::util::read_u8_le(&mut r)?;

        // amount_of_choice_item_rewards: u32
        let amount_of_choice_item_rewards = crate::util::read_u32_le(&mut r)?;

        // choice_item_rewards: QuestGiverReward[amount_of_choice_item_rewards]
        let choice_item_rewards = {
            let mut choice_item_rewards = Vec::with_capacity(amount_of_choice_item_rewards as usize);
            for _ in 0..amount_of_choice_item_rewards {
                choice_item_rewards.push(QuestGiverReward::read(&mut r)?);
            }
            choice_item_rewards
        };

        // amount_of_item_rewards: u32
        let amount_of_item_rewards = crate::util::read_u32_le(&mut r)?;

        // item_rewards: QuestGiverReward[amount_of_item_rewards]
        let item_rewards = {
            let mut item_rewards = Vec::with_capacity(amount_of_item_rewards as usize);
            for _ in 0..amount_of_item_rewards {
                item_rewards.push(QuestGiverReward::read(&mut r)?);
            }
            item_rewards
        };

        // money_reward: Gold
        let money_reward = Gold::new(crate::util::read_u32_le(&mut r)?);

        // experience_reward: u32
        let experience_reward = crate::util::read_u32_le(&mut r)?;

        // honor_reward: u32
        let honor_reward = crate::util::read_u32_le(&mut r)?;

        // honor_reward_multiplier: f32
        let honor_reward_multiplier = crate::util::read_f32_le(&mut r)?;

        // reward_spell: u32
        let reward_spell = crate::util::read_u32_le(&mut r)?;

        // casted_spell: u32
        let casted_spell = crate::util::read_u32_le(&mut r)?;

        // title_reward: u32
        let title_reward = crate::util::read_u32_le(&mut r)?;

        // talent_reward: u32
        let talent_reward = crate::util::read_u32_le(&mut r)?;

        // arena_point_reward: u32
        let arena_point_reward = crate::util::read_u32_le(&mut r)?;

        // unknown2: u32
        let unknown2 = crate::util::read_u32_le(&mut r)?;

        // reward_factions: u32[5]
        let reward_factions = {
            let mut reward_factions = [u32::default(); 5];
            for i in reward_factions.iter_mut() {
                *i = crate::util::read_u32_le(&mut r)?;
            }
            reward_factions
        };

        // reward_reputations: u32[5]
        let reward_reputations = {
            let mut reward_reputations = [u32::default(); 5];
            for i in reward_reputations.iter_mut() {
                *i = crate::util::read_u32_le(&mut r)?;
            }
            reward_reputations
        };

        // reward_reputations_override: u32[5]
        let reward_reputations_override = {
            let mut reward_reputations_override = [u32::default(); 5];
            for i in reward_reputations_override.iter_mut() {
                *i = crate::util::read_u32_le(&mut r)?;
            }
            reward_reputations_override
        };

        // amount_of_emotes: u32
        let amount_of_emotes = crate::util::read_u32_le(&mut r)?;

        // emotes: QuestDetailsEmote[amount_of_emotes]
        let emotes = {
            let mut emotes = Vec::with_capacity(amount_of_emotes as usize);
            for _ in 0..amount_of_emotes {
                emotes.push(QuestDetailsEmote::read(&mut r)?);
            }
            emotes
        };

        Ok(Self {
            guid,
            guid2,
            quest_id,
            title,
            details,
            objectives,
            auto_finish,
            quest_flags,
            suggested_players,
            is_finished,
            choice_item_rewards,
            item_rewards,
            money_reward,
            experience_reward,
            honor_reward,
            honor_reward_multiplier,
            reward_spell,
            casted_spell,
            title_reward,
            talent_reward,
            arena_point_reward,
            unknown2,
            reward_factions,
            reward_reputations,
            reward_reputations_override,
            emotes,
        })
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_QUESTGIVER_QUEST_DETAILS {}

impl SMSG_QUESTGIVER_QUEST_DETAILS {
    pub(crate) fn size(&self) -> usize {
        8 // guid: Guid
        + 8 // guid2: Guid
        + 4 // quest_id: u32
        + self.title.len() + 1 // title: CString
        + self.details.len() + 1 // details: CString
        + self.objectives.len() + 1 // objectives: CString
        + 1 // auto_finish: Bool
        + 4 // quest_flags: u32
        + 4 // suggested_players: u32
        + 1 // is_finished: u8
        + 4 // amount_of_choice_item_rewards: u32
        + self.choice_item_rewards.len() * 12 // choice_item_rewards: QuestGiverReward[amount_of_choice_item_rewards]
        + 4 // amount_of_item_rewards: u32
        + self.item_rewards.len() * 12 // item_rewards: QuestGiverReward[amount_of_item_rewards]
        + 4 // money_reward: Gold
        + 4 // experience_reward: u32
        + 4 // honor_reward: u32
        + 4 // honor_reward_multiplier: f32
        + 4 // reward_spell: u32
        + 4 // casted_spell: u32
        + 4 // title_reward: u32
        + 4 // talent_reward: u32
        + 4 // arena_point_reward: u32
        + 4 // unknown2: u32
        + 20 // reward_factions: u32[5]
        + 20 // reward_reputations: u32[5]
        + 20 // reward_reputations_override: u32[5]
        + 4 // amount_of_emotes: u32
        + self.emotes.len() * 8 // emotes: QuestDetailsEmote[amount_of_emotes]
    }
}

